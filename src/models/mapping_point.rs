use crate::{
  models::trajectory_point::TrajectoryPoint
};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct MappingPoint {
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64,
  mappingtude: f64
}

impl MappingPoint {
  pub fn map_point(point: &TrajectoryPoint, avg_cog: f64) -> Self {
    let mut mappingtude: f64 = 0.0;
    let angle = (avg_cog / 180.0) * PI;

    if avg_cog >= 0.0 && avg_cog < 90.0 {
      mappingtude = (point.get_longitude() + (1.0 / angle.tan()) * point.get_latitude()) * angle.sin();
    } else if avg_cog >= 270.0 && avg_cog < 360.0 {
      mappingtude = (point.get_latitude() - (PI * 2.0 - angle).tan() * point.get_longitude()) * (PI * 2.0 - angle).cos(); 
    } else if avg_cog >= 90.0 && avg_cog < 180.0 {
      mappingtude = ((PI - angle).tan() * point.get_longitude() - point.get_latitude()) * (PI - angle).cos();
    } else if avg_cog >= 180.0 && avg_cog < 270.0 {
      mappingtude = -((1.0 / (angle - PI).tan()) * point.get_latitude() + point.get_longitude()) * (angle - PI).sin();
    }

    Self {
      longitude: point.get_longitude(),
      latitude: point.get_latitude(),
      sog: point.get_sog(),
      cog: point.get_cog(),
      mappingtude
    }
  }

  pub fn get_mappingtude(&self) -> f64 {
    self.mappingtude
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