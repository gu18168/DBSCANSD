use crate::{
  models::{
    cluster::Cluster,
    trajectory_point::TrajectoryPoint,
    merge_indexs::MergeIndexs,
    work_point::WorkPoint,
    point_set::PointSet,
  },
  dbscan_utility::is_density_reachable,
};
use rayon::ThreadPoolBuilder;
use uuid::Uuid;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::{HashSet, HashMap};

pub fn apply_dbscansd(
  point_set: &mut PointSet,
  eps: f64,
  min_points: i32,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Box<Vec<Cluster>>
{
  let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

  let mut result_clusters: Vec<Cluster> = Vec::new();
  let mut core_uuids: Vec<Uuid> = Vec::new();
  let len = point_set.len();

  for i in 0..len {
    println!("making cluster {} of {}", i, len);
    let point: &WorkPoint = point_set.get(i);
    let iter_of_point_set = point_set.iter();

    let cluster = pool.install(|| {
      let mut cluster_raw: Vec<WorkPoint> = Vec::new();

      for p in iter_of_point_set {
        if is_density_reachable(p.get_point(), point.get_point(), eps, max_spd, max_dir, is_stop_point) {
          // @Clone
          cluster_raw.push(p.clone());
        }
      }

      cluster_raw
    });

    if cluster.len() >= (min_points as usize) {
      // @Clone
      core_uuids.push(point.get_uuid().clone());
      
      result_clusters.push(Cluster::new(cluster));
    }
  }

  for core_uuid in core_uuids {
    point_set.set_point_core(&core_uuid);
  }

  let mut real_result_clusters: Vec<Cluster> = Vec::new();

  let merge_indexs = MergeIndexs::new();
  let merge_indexs = Arc::new(Mutex::new(merge_indexs));

  let len = result_clusters.len();
  for i in 0..len {
    println!("merging cluster index {} of {}", i, len);
    let clone_merge_indexs = Arc::clone(&merge_indexs);

    pool.install(|| {
      let mut can_merge_index: Vec<usize> = Vec::new();

      let to_cluster = result_clusters.get(i).unwrap();
      for j in 0..i {
          let from_cluster = result_clusters.get(j).unwrap();
          if can_merge(to_cluster, from_cluster, point_set.get_core_map()) {
            can_merge_index.push(j);
          }
      }

      if can_merge_index.len() == 0 {
        // 没有可以合并的，就索引对自己
        clone_merge_indexs.lock().unwrap().push(i)
      } else {
        // 有可以合并的，就将这些索引都设置为最小值
        clone_merge_indexs.lock().unwrap().set_to_min(&can_merge_index);
      }
    });
  }

  // 事实证明，合并最需要时间
  let merge_cluster_indexs = merge_indexs.lock().unwrap().map_indexs();
  let len = merge_cluster_indexs.len();
  for (index, merge_cluster_index) in merge_cluster_indexs.iter().enumerate() {
    println!("merging cluster {} of {}", index, len);
    let merged_cluster = pool.install(|| {
      metge_clusters(&result_clusters, &merge_cluster_index)
    });

    real_result_clusters.push(merged_cluster);
  }

  Box::new(real_result_clusters)
}

fn can_merge(c1: &Cluster, c2: &Cluster, core_map: &HashMap<Uuid, bool>) -> bool {
  let points_c1 = c1.get_cluster();
  let points_c2 = c2.get_cluster();

  if points_c1.len() == 0 || points_c2.len() == 0 {
    return false;
  }

  for p in points_c2 {
    if is_point_core(core_map, p.get_uuid()) && points_c1.contains(p) {
      return true;
    }
  }

  false
}

fn is_point_core(core_map: &HashMap<Uuid, bool>, uuid: &Uuid) -> bool {
  if let Some(val) = core_map.get(uuid) {
    return *val;
  }

  false
}

fn metge_clusters(clusters: &Vec<Cluster>, indexs: &Vec<usize>) -> Cluster {
  let mut raw_points: HashSet<WorkPoint> = HashSet::new();
  let len = indexs.len();

  for (i, index) in indexs.iter().enumerate() {
    println!("real merging cluster {} of {}", i, len);
    let cluster = clusters.get(*index).unwrap().get_cluster();
    for point in cluster {
      // @Clone
      raw_points.insert(point.clone());
    }
  }

  let result = Cluster::new(raw_points.into_iter().collect::<Vec<WorkPoint>>());
  result
}