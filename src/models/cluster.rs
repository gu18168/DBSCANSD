use crate::{
  models::trajectory_point::TrajectoryPoint
};

pub struct Cluster{
  cluster: Vec<TrajectoryPoint>
}

impl Cluster {
  pub fn new(cluster: Vec<TrajectoryPoint>) -> Self {
    Self {
      cluster
    }
  }

  pub fn get_cluster(&self) -> &Vec<TrajectoryPoint> {
    &self.cluster
  }

  pub fn cal_average_dir(&self) -> f64 {
    let mut sum: f64 = 0.0;

    for p in self.get_cluster() {
      sum += p.get_cog();
    }

    sum / (self.cluster.len() as f64)
  }
}