use crate::{
  models::work_point::WorkPoint
};

pub struct Cluster{
  cluster: Vec<WorkPoint>
}

impl Cluster {
  pub fn new(cluster: Vec<WorkPoint>) -> Self {
    Self {
      cluster
    }
  }

  pub fn get_cluster(&self) -> &Vec<WorkPoint> {
    &self.cluster
  }

  pub fn cal_average_dir(&self) -> f64 {
    let mut sum: f64 = 0.0;

    for p in self.get_cluster() {
      sum += p.get_point().get_cog();
    }

    sum / (self.cluster.len() as f64)
  }
}