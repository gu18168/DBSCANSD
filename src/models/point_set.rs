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

pub struct PointSet {
  point_set: Vec<WorkPoint>,
  is_core_map: HashMap<Uuid, bool>,
  uuid_map_index: HashMap<Uuid, usize>
}

impl PointSet {
  // 直接将原有 Vec 转化为 PointSet
  // 不再需要保留原有的 Vec 的所有权，因为到了 point_set 中
  pub fn new(points: Vec<TrajectoryPoint>) -> Self {
    let mut point_set: Vec<WorkPoint> = Vec::new();
    let mut uuid_map_index: HashMap<Uuid, usize> = HashMap::new();

    for point in points {
      let work_point = WorkPoint::new(point);
      uuid_map_index.insert(work_point.get_uuid().clone(), point_set.len());
      point_set.push(work_point);
    }

    Self {
      point_set,
      is_core_map: HashMap::new(),
      uuid_map_index
    }
  }

  pub fn get_core_map(&self) -> &HashMap<Uuid, bool> {
    &self.is_core_map
  }

  pub fn set_point_core(&mut self, uuid: Uuid) {
    // @Clone
    // 直接转移所有权
    self.is_core_map.insert(uuid, true);
  }

  pub fn len(&self) -> usize {
    self.point_set.len()
  }

  pub fn get(&self, index: usize) -> &WorkPoint {
    self.point_set.get(index).unwrap()
  }

  pub fn iter(&self) -> Iter<WorkPoint> {
    self.point_set.iter()
  }

  pub fn map_uuid_to_cluster(&self, uuid_cluster: UuidCluster) -> Cluster {
    let mut cluster: Vec<TrajectoryPoint> = Vec::new();

    for uuid in uuid_cluster.get_cluster() {
      if let Some(index) = self.uuid_map_index.get(uuid) {
        cluster.push(self.point_set.get(*index).unwrap().get_point().clone());
      }
    }

    Cluster::new(cluster)
  }
}