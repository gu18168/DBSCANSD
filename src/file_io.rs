use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::{
  models::{
    trajectory_point::TrajectoryPoint,
    gravity_vector::GravityVector
  }
};
use failure::{Error};
use chrono::prelude::*;

pub fn readFile(path: &str, is_stop_point: bool) -> Result<Vec<TrajectoryPoint>, Error> {
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