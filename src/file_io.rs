use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use crate::{
  models::{
    trajectory_point::TrajectoryPoint,
    gravity_vector::GravityVector
  }
};
use failure::{Error};
use chrono::prelude::*;

pub fn read_file(path: &str, is_stop_point: bool) -> Result<Vec<TrajectoryPoint>, Error> {
  let file = File::open(path)?;
  let file = BufReader::new(file);

  let mut trajectory_points: Vec<TrajectoryPoint> = Vec::new();

  for line in file.lines().filter_map(|result| result.ok()) {
    // split return Split
    // Split 实现了 Iterator trait
    let mut scanner = line.split(',');
    let is_first_line = scanner.next().unwrap();

    // 跳过 csv 文件的第一行
    if is_first_line == "MMSI" { continue };

    let mmsi = is_first_line;
    let timestamp = time_to_second(scanner.next().unwrap()).expect("Time format has error!");
    let sog_raw = scanner.next().unwrap();
    let longitude: f64 = scanner.next().unwrap().parse().expect("longitude isn't double!");
    let latitude: f64 = scanner.next().unwrap().parse().expect("latitude isn't double!");
    let cog_raw = scanner.next().unwrap();

    let sog: f64 = if sog_raw == "None" { 0.0 } else { sog_raw.parse().expect("SOG isn't double!") };
    let cog: f64 = if cog_raw == "None" { 0.0 } else { cog_raw.parse().expect("COG isn't double!") };

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