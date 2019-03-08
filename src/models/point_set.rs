//! 点集，从 csv 文件读取生成的所有点的集合
use crate::{
  models::{
    work_point::WorkPoint,
    trajectory_point::TrajectoryPoint
  }
};
use std::slice::Iter;

/// 点集除了存所有的点，还有一个映射，能够用 uuid 找到真实的点索引。
pub struct PointSet {
  point_set: Vec<WorkPoint>
}

impl PointSet {
  // 直接将原有 Vec 转化为 PointSet
  // 不再需要保留原有的 Vec 的所有权，因为到了 point_set 中
  pub fn new(points: Vec<TrajectoryPoint>) -> Self {
    let mut point_set: Vec<WorkPoint> = Vec::new();

    for point in points {
      point_set.push(WorkPoint::new(point));
    }

    Self {
      point_set
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
}