use crate::{
  models::{
    gravity_vector::GravityVector,
    cluster::Cluster,
    mapping_point::MappingPoint
  }
};

pub fn extract_gv(cluster: &Cluster) -> Vec<GravityVector> {
  let avg_cog: f64 = cluster.cal_average_dir();
  let mut mp_list: Vec<MappingPoint> = Vec::new();

  for p in cluster.get_cluster() {
    let mp: MappingPoint = MappingPoint::map_point(p, avg_cog);
    mp_list.push(mp);
  }

  mp_list = sort(mp_list);

  let mut gv_list: Vec<GravityVector> = Vec::new();

  let mut count = 0;
  let mut k = 0;
  let mut sum_x = 0.0;
  let mut sum_y = 0.0;
  let mut sum_sog = 0.0;
  let mut sum_cog = 0.0;

  let mut tra_point_map: Vec<MappingPoint> = Vec::new();

  while count <= mp_list.len() {
    if count < mp_list.len() 
      && (mp_list.get(count).unwrap().get_mappingtude() - mp_list.get(k).unwrap().get_mappingtude() < 0.01)
    {
      sum_x = sum_x + mp_list.get(count).unwrap().get_longitude();
      sum_y = sum_y + mp_list.get(count).unwrap().get_latitude();
      sum_sog = sum_sog + mp_list.get(count).unwrap().get_sog();
      sum_cog = sum_cog + mp_list.get(count).unwrap().get_cog();

      tra_point_map.push(mp_list.get(count).unwrap().clone());

      count += 1;
    } else {
      let x = sum_x / ((count - k) as f64);
      let y = sum_y / ((count - k) as f64);
      let sog = sum_sog / ((count - k) as f64);
      let cog = sum_cog / ((count - k) as f64);

      let mut distances: Vec<f64> = Vec::new();
      for tra_point in tra_point_map.iter() {
        let lon = tra_point.get_longitude();
        let lat = tra_point.get_latitude();
        let dist = gps_distance(lat, lon, y, x);
        distances.push(dist);
      }

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

fn sort(mut mp_list: Vec<MappingPoint>) -> Vec<MappingPoint> {
  let mut i = 1;

  while i < mp_list.len() {
    let mut k = i;
    let mp: MappingPoint = mp_list.get(i).unwrap().clone();
    let mut flag: bool = false;

    while mp.get_mappingtude() < mp_list.get(k - 1).unwrap().get_mappingtude() {
      if k == 1 {
        mp_list.remove(i);
        mp_list.insert(0, mp.clone());
        flag = true;
        break;
      }
      k -= 1;
    }

    if flag {
      mp_list.remove(i);
      mp_list.insert(k, mp.clone());
    }

    i += 1;
  }

  mp_list
}

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

fn quartile(values: &Vec<f64>, lower_percent: usize) -> f64 {
  let mut sort_values: Vec<f64> = values.clone();
  sort_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let n: usize;
  if sort_values.len() == 1 { n = 0; }
  else { n = sort_values.len() * lower_percent / 100; }

  sort_values[n]
}