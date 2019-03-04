use crate::{
  models::{
    work_point::WorkPoint,
    trajectory_point::TrajectoryPoint
  }
};
use uuid::Uuid;
use std::collections::HashMap;
use std::slice::Iter;

pub struct PointSet {
  point_set: Vec<WorkPoint>,
  is_core_map: HashMap<Uuid, bool>
}

impl PointSet {
  pub fn new(points: &Vec<TrajectoryPoint>) -> Self {
    let mut point_set: Vec<WorkPoint> = Vec::new();

    for point in points {
      point_set.push(WorkPoint::new(point));
    }

    Self {
      point_set,
      is_core_map: HashMap::new()
    }
  }

  pub fn get_core_map(&self) -> &HashMap<Uuid, bool> {
    &self.is_core_map
  }

  pub fn set_point_core(&mut self, uuid: &Uuid) {
    // @Clone
    self.is_core_map.insert(uuid.clone(), true);
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
}