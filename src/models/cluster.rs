use crate::{
  models::trajectory_point::TrajectoryPoint
};
use std::borrow::Cow;

/// # Should Know
/// * [Cow - Chinese](http://wiki.jikexueyuan.com/project/rust-primer/intoborrow/cow.html)
/// * [Cow - English](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
pub struct Cluster<'a> {
  avg_sog: f64,
  avg_cog: f64,
  cluster: Cow<'a, Vec<TrajectoryPoint>>
}

impl<'a> Cluster<'a> {
  pub fn new(cluster: Vec<TrajectoryPoint>) -> Self {
    Self {
      avg_sog: 0.0,
      avg_cog: 0.0,
      cluster: Cow::Owned(cluster)
    }
  }

  pub fn get_cluster(&self) -> &Vec<TrajectoryPoint> {
    &self.cluster
  }

  pub fn get_mut_cluster(&mut self) -> &mut Vec<TrajectoryPoint> {
    self.cluster.to_mut()
  }
}