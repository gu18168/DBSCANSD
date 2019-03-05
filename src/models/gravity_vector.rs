//! 簇重心向量，由运动点簇抽象出的结果

/// GV 由运动点簇抽象得到
pub struct GravityVector {
  median_distance: f64,
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64
}

impl GravityVector {
  pub fn new(
    median_distance: f64,
    longitude: f64,
    latitude: f64,
    sog: f64,
    cog: f64)-> Self 
  {
    Self {
      median_distance,
      longitude,
      latitude,
      sog,
      cog
    }
  }

  pub fn get_longitude(&self) -> f64 {
    self.longitude
  }

  pub fn get_latitude(&self) -> f64 {
    self.latitude
  }

  pub fn get_sog(&self) -> f64 {
    self.sog
  }

  pub fn get_cog(&self) -> f64 {
    self.cog
  }

  pub fn get_median_distance(&self) -> f64 {
    self.median_distance
  }
}