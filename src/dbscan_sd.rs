use crate::{
  models::{
    cluster::Cluster,
    trajectory_point::TrajectoryPoint
  },
  dbscan_utility::is_density_reachable
};

pub fn apply_dbscansd(
  points: &mut Vec<TrajectoryPoint>,
  eps: f64,
  min_points: i32,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Vec<Cluster>
{
  let mut result_clusters: Vec<Cluster> = Vec::new();
  let len = points.len() - 2;

  for i in 0..len {
    let mut cluster_raw: Vec<TrajectoryPoint> = Vec::new();
    let point: &TrajectoryPoint = points.get(i).unwrap();

    // 主要是这一个全遍历然后还得算非常耗时间
    // 是不是可以考虑多线程来运算？将一个点的聚类分给一个线程来进行
    // 当全部点跑完之后，再让主合并聚类结果
    // * 多线程主要的竞争点在于 result_clusters 的共享
    // * 其他的数据由于只是读，所以很安全
    for p in points.iter() {
      if is_density_reachable(p, point, eps, max_spd, max_dir, is_stop_point) {
        cluster_raw.push(p.clone());
      }
    }

    if cluster_raw.len() >= (min_points as usize) {
      let point = points.get_mut(i).unwrap();
      point.is_core_point = true;

      result_clusters.push(Cluster::new(cluster_raw));
    }
  }

  let mut real_result_clusters: Vec<Cluster> = Vec::new();
  for cluster in result_clusters.iter() {
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

      // 删除合并之前的簇
      for index in indexs {
        real_result_clusters.remove(index);
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
    for point in clusters.get(*index).unwrap().get_cluster() {
      if !points.contains(point) {
        points.push(point.clone());
      }
    }
  }

  let result = Cluster::new(points);
  result
}