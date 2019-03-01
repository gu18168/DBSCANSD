use crate::{
  models::{
    cluster::Cluster,
    trajectory_point::TrajectoryPoint
  },
  dbscan_utility::is_density_reachable
};
use rayon::ThreadPoolBuilder;

pub fn apply_dbscansd(
  points: &mut Vec<TrajectoryPoint>,
  eps: f64,
  min_points: i32,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Vec<Cluster>
{
  let pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();

  let mut result_clusters: Vec<Cluster> = Vec::new();
  let len = points.len();

  for i in 0..len {
    println!("making cluster {}", i);
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
      let point = points.get_mut(i).unwrap();
      point.is_core_point = true;

      result_clusters.push(Cluster::new(cluster));
    }
  }

  // 合并簇也比较慢，是否能优化
  let mut real_result_clusters: Vec<Cluster> = Vec::new();
  for (ll, cluster) in result_clusters.iter().enumerate() {
    println!("merging cluster {}", ll);
    let mut indexs: Vec<usize> = Vec::new();

    // 记录所有可以合并的簇索引将它们一起合并
    for (index, to_cluster) in real_result_clusters.iter().enumerate() {
      if is_merge(cluster, to_cluster) {
        indexs.push(index);
      }
    } 

    // 先将目前这个加入到簇末尾
    real_result_clusters.push(
        Cluster::new(cluster.get_cluster().clone())
    );

    if indexs.len() == 0 {
      // 没有能合并的，直接进入下一个循环
      continue;
    } else {
      // 要把目前新加入的这个也合并进去
      indexs.push(real_result_clusters.len() - 1);

      let merged_cluster: Cluster = merge_clusters(&real_result_clusters, &indexs);
      // 删除合并之前的簇，注意需要逆序来删除
      for i in (0..indexs.len()).rev() {
        real_result_clusters.remove(indexs[i]);
      }
      // 加入合并完的簇
      real_result_clusters.push(merged_cluster);
    }
  }

  real_result_clusters
}

fn is_merge(c1: &Cluster, c2: &Cluster) -> bool {
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

fn merge_clusters(clusters: &Vec<Cluster>, indexs: &Vec<usize>) -> Cluster {
  let mut points: Vec<TrajectoryPoint> = Vec::new();

  for index in indexs {
    let cluster = clusters.get(*index).unwrap().get_cluster();

    for point in cluster {
      if !points.contains(point) {
        points.push(point.clone());
      }
    }
  }

  let result = Cluster::new(points);
  result
}