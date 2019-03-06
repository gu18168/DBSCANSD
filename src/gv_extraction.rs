//! 用于从运动点簇中获得 GravityVector
use crate::{
  models::{
    gravity_vector::GravityVector,
    cluster::Cluster,
    mapping_point::MappingPoint
  }
};

/// 将所给的 Cluster 抽象为 GravityVector
pub fn extract_gv(cluster: &Cluster) -> Vec<GravityVector> {
  let avg_cog: f64 = cluster.cal_average_dir();
  let mut mp_list: Vec<MappingPoint> = Vec::new();

  // 将 TrajectoryPoint 映射成 MappingPoint
  for p in cluster.get_cluster() {
    mp_list.push(MappingPoint::map_point(p, avg_cog));
  }

  mp_list.sort();

  let mut gv_list: Vec<GravityVector> = Vec::new();

  let mut count = 0;
  let mut k = 0;
  let mut sum_x = 0.0;
  let mut sum_y = 0.0;
  let mut sum_sog = 0.0;
  let mut sum_cog = 0.0;

  let mut tra_point_map: Vec<&MappingPoint> = Vec::new();

  // 由于 count 肯定小于 len 所以我们才敢直接 unwrap
  while count <= mp_list.len() {
    if count < mp_list.len() && 
      mp_list.get(count).unwrap().get_mappingtude() - mp_list.get(k).unwrap().get_mappingtude() < 0.01
    {
      let from_point = mp_list.get(count).unwrap();

      // 将足够接近的 MappingPoint 合到一个 GravityVector 中
      sum_x = sum_x + from_point.get_longitude();
      sum_y = sum_y + from_point.get_latitude();
      sum_sog = sum_sog + from_point.get_sog();
      sum_cog = sum_cog + from_point.get_cog();

      // @Clone
      // 只读可以采用读引用，没有必要 Clone
      tra_point_map.push(from_point);

      count += 1;
    } else {
      let x = sum_x / ((count - k) as f64);
      let y = sum_y / ((count - k) as f64);
      let sog = sum_sog / ((count - k) as f64);
      let cog = sum_cog / ((count - k) as f64);

      let mut distances: Vec<f64> = Vec::new();
      for tra_point in tra_point_map.iter() {
        let dist = gps_distance(tra_point.get_longitude(), tra_point.get_latitude(), y, x);
        distances.push(dist);
      }

      distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
      let median_distance = quartile(&distances, 50);

      let gv: GravityVector = GravityVector::new(
        median_distance,
        x, y, sog, cog
      );

      gv_list.push(gv);

      sum_x = 0.0;
      sum_y = 0.0;
      sum_cog = 0.0;
      sum_sog = 0.0;
      k = count;
      tra_point_map.clear();

      if count == mp_list.len() { break; }
    }
  }

  gv_list
}

/// 根据所给出的两个经纬度计算实际距离
fn gps_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
  let earth_radius = 3958.75;
  let d_lat = (lat2 - lat1).to_radians();
  let d_lon = (lon2 - lon1).to_radians();
  let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
    + (lat1.to_radians() * lat2.to_radians()).cos()
    * (d_lon / 2.0).sin() * (d_lon / 2.0).sin();
  let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
  let dist = earth_radius * c;

  let meter_conversion = 1609;

  dist * (meter_conversion as f64)
}

/// 返回有序动态数组的第 lower_percent% 个元素
/// 
/// 由于 lower_percent 是代码中自己定的，所以无需考虑大于 100% 的错误情况
fn quartile(values: &Vec<f64>, lower_percent: usize) -> f64 {
  let n: usize;
  if values.len() == 1 { n = 0; }
  else { n = values.len() * lower_percent / 100; }

  values[n]
}