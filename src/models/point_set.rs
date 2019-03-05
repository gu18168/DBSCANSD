//! 点集，从 csv 文件读取生成的所有点的集合
use crate::{
  models::{
    work_point::WorkPoint,
    trajectory_point::TrajectoryPoint,
    cluster::Cluster,
    uuid_cluster::UuidCluster
  }
};
use uuid::Uuid;
use std::collections::HashMap;
use std::slice::Iter;

/// 点集除了存所有的点，还有一个映射，能够用 uuid 找到真实的点索引。
pub struct PointSet {
  point_set: Vec<WorkPoint>,
  uuid_map_index: HashMap<Uuid, usize>
}

impl PointSet {
  // 直接将原有 Vec 转化为 PointSet
  // 不再需要保留原有的 Vec 的所有权，因为到了 point_set 中
  pub fn new(points: Vec<TrajectoryPoint>) -> Self {
    let mut point_set: Vec<WorkPoint> = Vec::new();
    let mut uuid_map_index: HashMap<Uuid, usize> = HashMap::new();

    for point in points {
      // 利用轨迹点获得工作点
      let work_point = WorkPoint::new(point);
      // @Clone
      // 为了方便直接 Clone ，建立 uuid -> index 的映射
      uuid_map_index.insert(work_point.get_uuid().clone(), point_set.len());
      point_set.push(work_point);
    }

    Self {
      point_set,
      uuid_map_index
    }
  }

  pub fn len(&self) -> usize {
    self.point_set.len()
  }

  pub fn get(&self, index: usize) -> &WorkPoint {
    self.point_set.get(index).unwrap()
  }

  /// 获得点集的迭代器
  pub fn iter(&self) -> Iter<WorkPoint> {
    self.point_set.iter()
  }

  /// 通过 [UuidCluster](../uuid_cluster/struct.UuidCluster.html) 获得 
  /// [Cluster](../cluster/struct.Cluster.html)。
  /// 主要是利用自身的 uuid -> index 的映射，然后根据 index 获得轨迹点的引用
  /// 以构建 [Cluster](../cluster/struct.Cluster.html)。
  pub fn map_uuid_to_cluster(&self, uuid_cluster: UuidCluster) -> Cluster {
    let mut cluster: Vec<&TrajectoryPoint> = Vec::new();

    for uuid in uuid_cluster.get_cluster() {
      if let Some(index) = self.uuid_map_index.get(uuid) {
        // @Clone
        // Cluster 不再存真实的点，而是 PointSet 内的点的引用
        // 因为 PointSet 的生命周期肯定比 Cluster 长
        cluster.push(self.point_set.get(*index).unwrap().get_point());
      }
    }

    Cluster::new(cluster)
  }
}