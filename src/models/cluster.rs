//! 簇，聚类的结果
use crate::{
  models::trajectory_point::TrajectoryPoint
};

/// 簇中存放的是轨迹点的读引用，
/// 因为簇没有写操作，只有生成 GV 或者输出到 IO 的操作。
pub struct Cluster<'a>{
  cluster: Vec<&'a TrajectoryPoint>
}

impl<'a> Cluster<'a> {
  pub fn new(cluster: Vec<&'a TrajectoryPoint>) -> Self {
    Self {
      cluster
    }
  }

  /// 获得实际存储的轨迹点动态数组的读引用。
  pub fn get_cluster(&self) -> &Vec<&TrajectoryPoint> {
    &self.cluster
  }

  /// 计算该簇中轨迹点的平均方向，
  /// 用于 GV 的生成。
  pub fn cal_average_dir(&self) -> f64 {
    let mut sum: f64 = 0.0;

    for p in self.get_cluster() {
      sum += p.get_cog();
    }

    sum / (self.cluster.len() as f64)
  }
}