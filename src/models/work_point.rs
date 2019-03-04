use crate::{
  models::{
    trajectory_point::TrajectoryPoint
  }
};
use uuid::Uuid;
use std::hash::{Hash, Hasher};

/// # Notes
/// * PartialEq: .contains()
/// * Clone: .clone() 
#[derive(Clone)]
pub struct WorkPoint {
  uuid: Uuid,
  point: TrajectoryPoint
}

impl PartialEq for WorkPoint {
  fn eq(&self, other: &WorkPoint) -> bool {
    self.uuid == other.uuid
  }
}

impl Eq for WorkPoint {}

impl Hash for WorkPoint {
  fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl WorkPoint {
  // @Clone
  pub fn new(point: &TrajectoryPoint) -> Self {
    Self {
      uuid: Uuid::new_v4(),
      point: point.clone()
    }
  }

  pub fn get_uuid(&self) -> &Uuid {
    &self.uuid
  }

  pub fn get_point(&self) -> &TrajectoryPoint {
    &self.point
  }
}