use crate::{
  models::{
    cluster::Cluster,
    merge_indexs::MergeIndexs,
    work_point::WorkPoint,
    point_set::PointSet,
    uuid_cluster::UuidCluster
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
  is_stop_point: bool) -> Vec<Cluster>
{
  let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

  let mut result_uuid_clusters: Vec<UuidCluster> = Vec::new();
  let mut core_uuids: Vec<Uuid> = Vec::new();
  let len = point_set.len();

  // 采用 UuidCluster 代替原来的 Cluster
  // Uuid 的 Clone 优于 TrajectoryPoint 的 Clone
  for i in 0..len {
    println!("making cluster {} of {}", i, len);
    let point: &WorkPoint = point_set.get(i);
    let iter_of_point_set = point_set.iter();

    let uuid_cluster = pool.install(|| {
      let mut uuid_cluster_raw: Vec<Uuid> = Vec::new();

      for p in iter_of_point_set {
        if is_density_reachable(p.get_point(), point.get_point(), eps, max_spd, max_dir, is_stop_point) {
          // @Clone
          // Uuid Clone
          uuid_cluster_raw.push(p.get_uuid().clone());
        }
      }

      uuid_cluster_raw
    });

    if uuid_cluster.len() >= (min_points as usize) {
      // @Clone
      // Uuid Clone
      core_uuids.push(point.get_uuid().clone());
      
      result_uuid_clusters.push(UuidCluster::new(uuid_cluster));
    }
  }

  for core_uuid in core_uuids {
    point_set.set_point_core(core_uuid);
  }

  let mut real_uuid_result_clusters: Vec<UuidCluster> = Vec::new();

  let merge_indexs = MergeIndexs::new();
  let merge_indexs = Arc::new(Mutex::new(merge_indexs));

  // 利用 Uuid 进行 merge 即可
  let len = result_uuid_clusters.len();
  for i in 0..len {
    println!("merging cluster index {} of {}", i, len);
    let clone_merge_indexs = Arc::clone(&merge_indexs);

    pool.install(|| {
      let mut can_merge_index: Vec<usize> = Vec::new();

      let to_uuid_cluster = result_uuid_clusters.get(i).unwrap();
      for j in 0..i {
          let from_uuid_cluster = result_uuid_clusters.get(j).unwrap();
          if can_merge(to_uuid_cluster, from_uuid_cluster, point_set.get_core_map()) {
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
      merge_clusters(&result_uuid_clusters, &merge_cluster_index)
    });

    real_uuid_result_clusters.push(merged_cluster);
  }

  let mut result_clusters: Vec<Cluster> = Vec::new();

  for real_uuid_result_cluster in real_uuid_result_clusters {
    result_clusters.push(point_set.map_uuid_to_cluster(real_uuid_result_cluster));
  }

  result_clusters
}

fn can_merge(c1: &UuidCluster, c2: &UuidCluster, core_map: &HashMap<Uuid, bool>) -> bool {
  let uuids_c1 = c1.get_cluster();
  let uuids_c2 = c2.get_cluster();

  if uuids_c1.len() == 0 || uuids_c2.len() == 0 {
    return false;
  }

  for uuid in uuids_c2 {
    if is_point_core(core_map, uuid) && uuids_c1.contains(uuid) {
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

fn merge_clusters(uuid_clusters: &Vec<UuidCluster>, indexs: &Vec<usize>) -> UuidCluster {
  let mut raw_uuids: HashSet<Uuid> = HashSet::new();
  let len = indexs.len();

  for (i, index) in indexs.iter().enumerate() {
    println!("real merging cluster {} of {}", i, len);
    let uuid_cluster = uuid_clusters.get(*index).unwrap().get_cluster();
    for uuid in uuid_cluster {
      // @Clone
      // 这里就用 raw_uuids 空间换了时间的效率
      raw_uuids.insert(uuid.clone());
    }
  }

  let result = UuidCluster::new(raw_uuids.into_iter().collect::<Vec<Uuid>>());
  result
}