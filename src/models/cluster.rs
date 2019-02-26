use crate::{
  models::trajectory_point::TrajectoryPoint
};
use std::borrow::Cow;

/// # Should Know
/// * [Cow - Chinese](http://wiki.jikexueyuan.com/project/rust-primer/intoborrow/cow.html)
/// * [Cow - English](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
pub struct Cluster<'a> {
  cluster: Cow<'a, Vec<TrajectoryPoint>>
}

impl<'a> Cluster<'a> {
  pub fn new(cluster: Vec<TrajectoryPoint>) -> Self {
    Self {
      cluster: Cow::Owned(cluster)
    }
  }

  pub fn get_cluster(&self) -> &Vec<TrajectoryPoint> {
    &self.cluster
  }

  pub fn get_mut_cluster(&mut self) -> &mut Vec<TrajectoryPoint> {
    self.cluster.to_mut()
  }

  pub fn cal_average_dir(&self) -> f64 {
    let mut sum: f64 = 0.0;

    for p in self.get_cluster() {
      sum += p.get_cog();
    }

    sum / (self.cluster.len() as f64)
  }
}