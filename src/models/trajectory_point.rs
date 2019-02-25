/// # Should Know
/// * Rust doesn't support field mutability
/// * Mutability is a property of the binding, not of the struct
pub struct TrajectoryPoint {
  mmsi: String,
  timestamp: i64,
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64,
  is_visited: bool,
  is_core_point: bool
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
}