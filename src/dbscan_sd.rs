//! DBSCANSD 算法实现
use crate::{
  models::{
    cluster::Cluster,
    merge_indexs::MergeIndexs,
    work_point::WorkPoint,
    point_set::PointSet,
    trajectory_point::TrajectoryPoint
  },
  dbscan_utility::{
    is_density_reachable,
    is_density_reachable_of_many
  }
};
use rayon::ThreadPoolBuilder;
use uuid::Uuid;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};

/// 执行 DBSCANSD 算法
pub fn apply_dbscansd(
  point_set: &mut PointSet,
  eps: f64,
  min_points: usize,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Vec<Cluster>
{
  let pool = ThreadPoolBuilder::new().num_threads(16).build().unwrap();

  let len = point_set.len();

  let merge_indexs = MergeIndexs::new();
  let merge_indexs = Arc::new(Mutex::new(merge_indexs));
  let core_uuids: HashSet<&Uuid> = HashSet::new();
  let core_uuids = Arc::new(RwLock::new(core_uuids));

  for i in 0..len {
    println!("making cluster {} of {}", i, len);

    let point: &WorkPoint = point_set.get(i);
    let iter_of_point_set = point_set.iter().enumerate();

    let clone_merge_indexs = Arc::clone(&merge_indexs);
    let clone_core_uuids = Arc::clone(&core_uuids);

    let is_core = pool.install(|| {
      let mut cluster_size: usize = 0;
      let mut can_merge_index: Vec<usize> = Vec::new();

      for (index, p) in iter_of_point_set {
        if is_density_reachable(p.get_point(), point.get_point(), eps, max_spd, max_dir, is_stop_point) {
          cluster_size += 1;

          if clone_core_uuids.read().unwrap().contains(p.get_uuid()) {
            can_merge_index.push(index);
          }
        }
      }

      if cluster_size >= min_points {
        if can_merge_index.len() > 0 {
          clone_merge_indexs.lock().unwrap().set_to_min(&can_merge_index, i);
        } else {
          clone_merge_indexs.lock().unwrap().push(i);
        }

        return true;
      }

      false
    });

    if is_core {
      core_uuids.write().unwrap().insert(point.get_uuid());
    }
  }

  merge_indexs.lock().unwrap().correct_indexs();
  let merge_cluster_indexs = merge_indexs.lock().unwrap().map_indexs();
  let mut result_clusters: Vec<Cluster> = Vec::new();
  let merge_len = merge_cluster_indexs.len();

  // 很耗时，不知道能不能优化
  // 接下来要做的是将 Vec<Vec<usize>> 里面的 Vec<usize> 变成 Vec<TrajectoryPoint> 然后再变成 Cluster
  for (index, merge_cluster_index) in merge_cluster_indexs.iter().enumerate() {
    println!("making cluster {} of {}", index, merge_len);
    let mut core_points: Vec<&TrajectoryPoint> = Vec::new();
    
    // 构建 Vec<TrajectoryPoint>
    for index in merge_cluster_index {
      core_points.push(point_set.get(*index).get_point());
    }

    // 构建 Cluster
    let iter_of_point_set = point_set.iter();
    let cluster = pool.install(|| {
      let mut cluster_raw: Vec<&TrajectoryPoint> = Vec::new();

      for p in iter_of_point_set {
        let point: &TrajectoryPoint = p.get_point();
        if is_density_reachable_of_many(&core_points, point, eps, max_spd, max_dir, is_stop_point) {
          cluster_raw.push(point);
        }
      }

      cluster_raw
    });
    
    result_clusters.push(Cluster::new(cluster));
  }

  result_clusters
}
