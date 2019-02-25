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
}