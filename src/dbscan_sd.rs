use crate::{
  models::{
    cluster::Cluster,
    trajectory_point::TrajectoryPoint,
    merge_indexs::MergeIndexs,
  },
  dbscan_utility::is_density_reachable,
};
use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashSet;

pub fn apply_dbscansd(
  points: &mut Vec<TrajectoryPoint>,
  eps: f64,
  min_points: i32,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Box<Vec<Cluster>>
{
  let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

  let mut result_clusters: Vec<Cluster> = Vec::new();
  let len = points.len();

  for i in 0..len {
    println!("making cluster {} of {}", i, len);
    let point: &TrajectoryPoint = points.get(i).unwrap();
    let iter_of_points = points.iter();

    let cluster = pool.install(|| {
      let mut cluster_raw: Vec<TrajectoryPoint> = Vec::new();

      for p in iter_of_points {
        if is_density_reachable(p, point, eps, max_spd, max_dir, is_stop_point) {
          cluster_raw.push(p.clone());
        }
      }

      cluster_raw
    });

    if cluster.len() >= (min_points as usize) {
      // @BUG:
      // 这里可能导致多线程数据读取不一致
      // 且这里的 cluster 是改数据之前的，所以导致有问题，后面没有 merge 成功
      // @TODO
      // 考虑使用一种新的数据结构来聚合点集
      // 然后取消点里面的 core 属性，来移植到数据结构中
      // 例如 hashmap<usize, bool> 
      let point = points.get_mut(i).unwrap();
      point.is_core_point = true;

      result_clusters.push(Cluster::new(cluster));
    }
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
          if can_merge(to_cluster, from_cluster) {
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

fn can_merge(c1: &Cluster, c2: &Cluster) -> bool {
  let points_c1 = c1.get_cluster();
  let points_c2 = c2.get_cluster();

  if points_c1.len() == 0 || points_c2.len() == 0 {
    return false;
  }

  for p in points_c2 {
    if p.is_core_point && points_c1.contains(p) {
      return true;
    }
  }

  false
}

fn metge_clusters(clusters: &Vec<Cluster>, indexs: &Vec<usize>) -> Cluster {
  let mut raw_points: HashSet<TrajectoryPoint> = HashSet::new();
  let len = indexs.len();

  for (i, index) in indexs.iter().enumerate() {
    println!("real merging cluster {} of {}", i, len);
    let cluster = clusters.get(*index).unwrap().get_cluster();
    for point in cluster {
      raw_points.insert(point.clone());
    }
  }

  let result = Cluster::new(raw_points.into_iter().collect::<Vec<TrajectoryPoint>>());
  result
}