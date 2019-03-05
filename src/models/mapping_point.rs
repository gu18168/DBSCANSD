//! 映射点，将普通的轨迹点映射到某一个方向的结果
use crate::{
  models::trajectory_point::TrajectoryPoint
};
use std::f64::consts::PI;
use std::cmp::Ordering;

/// mappingtude 属性由经纬度根据指定方向计算得到，
/// 该属性也是抽象 GV 的关键属性，用于判断两个运动点在指定方向上是否足够接近。
pub struct MappingPoint {
  longitude: f64,
  latitude: f64,
  sog: f64,
  cog: f64,
  mappingtude: f64
}

impl PartialEq for MappingPoint {
  /// 利用 mappingtude 来判断两个 MappingPoint 的偏序关系
  fn eq(&self, other: &MappingPoint) -> bool {
    self.mappingtude == other.mappingtude
  }
}

impl Eq for MappingPoint {}

impl Ord for MappingPoint {
  /// 利用 mappingtude 来判断两个 MappingPoint 的大小关系
  fn cmp(&self, other: &MappingPoint) -> Ordering {
    if self.mappingtude > other.mappingtude {
      return Ordering::Greater;
    } else if self.mappingtude < other.mappingtude {
      return Ordering::Less;
    }

    Ordering::Equal
  }
}

impl PartialOrd for MappingPoint {
  /// 利用 mappingtude 来判断两个 MappingPoint 的偏序大小关系
  fn partial_cmp(&self, other: &MappingPoint) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl MappingPoint {
  /// 将普通的轨迹点根据指定方向映射成映射点。
  pub fn map_point(point: &TrajectoryPoint, avg_cog: f64) -> Self {
    let mut mappingtude: f64 = 0.0;
    let angle = (avg_cog / 180.0) * PI;

    // 根据指定方向来计算 mappintude
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