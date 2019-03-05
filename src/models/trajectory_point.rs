//! 轨迹点，从文件中读取的原始点数据格式

/// 直接根据 csv 文件内容构建。
pub struct TrajectoryPoint {
  mmsi: String,
  timestamp: i64,
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64
}

impl TrajectoryPoint {
  pub fn new(
    mmsi: &str, 
    timestamp: i64,
    longitude: f64,
    latitude: f64,
    sog: f64,
    cog: f64)-> Self 
  {
    Self {
      mmsi: mmsi.to_string(),
      timestamp,
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
}