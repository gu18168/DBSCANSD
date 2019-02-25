/// # Should Know
/// * Rust doesn't support field mutability
/// * Mutability is a property of the binding, not of the struct
/// 
/// # Notes
/// * PartialEq: .contains()
/// * Clone: .clone() 
#[derive(PartialEq, Clone)]
pub struct TrajectoryPoint {
  mmsi: String,
  timestamp: i64,
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64,
  pub is_visited: bool,
  pub is_core_point: bool
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
      cog,
      is_visited: false,
      is_core_point: false
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