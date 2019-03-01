use std::fs::OpenOptions;
use std::io::Write;
use crate::{
  models::{
    trajectory_point::TrajectoryPoint,
    gravity_vector::GravityVector
  }
};
use failure::{Error};
use chrono::prelude::*;
use csv::Reader;

pub fn read_csv_file(path: &str, is_stop_point: bool) -> Result<Vec<TrajectoryPoint>, Error> {
  let mut rdr = Reader::from_path(path)?;

  let mut trajectory_points: Vec<TrajectoryPoint> = Vec::new();

  for record in rdr.records().filter_map(|result| result.ok()) {
    // 默认已经跳过了第一行，所以无需我们再处理

    let mmsi = record.get(0).unwrap();
    let timestamp = time_to_second(record.get(1).unwrap()).expect("Time format has error!");
    let sog: f64 = record.get(2).unwrap().parse().expect("SOG isn't double!");
    let longitude: f64 = record.get(3).unwrap().parse().expect("longitude isn't double!");
    let latitude: f64 = record.get(4).unwrap().parse().expect("latitude isn't double!");
    let cog: f64 = record.get(5).unwrap().parse().expect("COG isn't double!");

    // 暂停点但是速度太快 & 移动点但是速度太慢
    if !is_stop_point && sog <= 0.5 { continue };
    if is_stop_point && sog > 0.5 { continue };

    let trajectory_point = TrajectoryPoint::new(
      mmsi,
      timestamp,
      longitude,
      latitude,
      sog,
      cog,
    );

    trajectory_points.push(trajectory_point);
  }

  Ok(trajectory_points)
}

fn time_to_second(time: &str) -> Result<i64, Error> {
  let date = Utc.datetime_from_str(time, "%Y%m%d_%H%M%S")?;
  Ok(date.timestamp())
}

// 存在一个问题，如果是多次写入就完蛋了
// 但是如果直接覆盖，会使原功能失效
pub fn write_cluster_to_file(out_path: &str, ppl: &Vec<TrajectoryPoint>, index: i32) {
  let mut file = OpenOptions::new().append(true).create(true)
    .open(out_path).expect("File doesn't open and write");

  if file.metadata().unwrap().len() == 0 {
    file.write_all(b"clusterIndex,Longitude,Latitude,SOG,COG\n")
      .expect("Write first line error");
  }

  for p in ppl {
    let line = index.to_string() + "," + &p.get_longitude().to_string() + "," + &p.get_latitude().to_string() + ","
      + &p.get_sog().to_string() + "," + &p.get_cog().to_string() + "\n";
    file.write_all(line.as_bytes()).expect("File write error");
  }
}

pub fn write_gv_to_file(out_path: &str, ppl: &Vec<GravityVector>, index: i32) {
  let mut file = OpenOptions::new().append(true).create(true)
    .open(out_path).expect("File doesn't open and write");

  if file.metadata().unwrap().len() == 0 {
    file.write_all(b"clusterIndex,Longitude,Latitude,SOG,COG,MedianDistance\n")
      .expect("Write first line error");
  }

  for p in ppl {
    let line = index.to_string() + "," + &p.get_longitude().to_string() + "," + &p.get_latitude().to_string() + ","
      + &p.get_sog().to_string() + "," + &p.get_cog().to_string() + "," + &p.get_median_distance().to_string() + "\n";
    file.write_all(line.as_bytes()).expect("File write error");    
  }
}