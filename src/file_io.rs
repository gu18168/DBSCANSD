//! 执行文件 IO 操作
//! 
//! 1. 读取 csv 文件获得船舶轨迹点数据
//! 2. 将得到的簇信息写入到 csv 文件中
//! 3. 将得到的运动重心向量写入到 csv 文件中
use std::fs::OpenOptions;
use std::io::Write;
use crate::{
  models::{
    trajectory_point::TrajectoryPoint,
    cluster::Cluster,
    gravity_vector::GravityVector,
    point_set::PointSet,
  }
};
use failure::{Error};
use chrono::prelude::*;
use csv::Reader;

/// 从指定的路径中读取 csv 文件，提取其中的船舶轨迹点信息。
/// 
/// 注意，函数根据第二个参数来区分运动点(false) / 静止点(true)。
/// 
/// # Panics
/// 请确保 csv 文件的内容按照以下格式：
/// `MMSI,Time,SOG,Longitude,Latitude,COG` ，
/// 否则将引发 panic! 。如果出现错误，请自行根据错误提示进行修改。
/// 
/// **各列数据类型及其含义:**
/// 1. MMSI: &str - 船舶的 MMSI 号
/// 2. Time: &str - 该 AIS 报文的发送时间，格式为 **%Y%m%d_%H%M%S**
/// 3. SOG: f64 - 船舶 SOG 数据
/// 4. Longitude: f64 - 船舶此时的经度
/// 5. Latitude: f64 - 船舶此时的纬度
/// 6. COG: f64 - 船舶 COG 数据
/// 
/// # Errors
/// 如果对于给定路径打开文件时出现问题，则返回相应的错误。
pub fn read_csv_file(path: &str, is_stop_point: bool) -> Result<PointSet, Error> {
  let mut rdr = Reader::from_path(path)?;

  let mut trajectory_points: Vec<TrajectoryPoint> = Vec::new();

  // 默认已经跳过了第一行，所以无需我们再处理
  for record in rdr.records().filter_map(|result| result.ok()) {
    let mmsi = record.get(0).unwrap();
    let timestamp = time_to_second(record.get(1).unwrap()).expect("Time format must be %Y%m%d_%H%M%S!");
    let sog: f64 = record.get(2).unwrap().parse().expect("SOG must be a f64!");
    let longitude: f64 = record.get(3).unwrap().parse().expect("longitude must be a f64!");
    let latitude: f64 = record.get(4).unwrap().parse().expect("latitude must be a f64!");
    let cog: f64 = record.get(5).unwrap().parse().expect("COG must be a f64!");

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

  Ok(PointSet::new(trajectory_points))
}

/// 将指定格式的时间转化为时间戳
/// 
/// # Errors
/// 如果给定的时间格式不符合要求或者时间不真实（例如 2 月 30 日）将会返回错误
fn time_to_second(time: &str) -> Result<i64, Error> {
  let date = Utc.datetime_from_str(time, "%Y%m%d_%H%M%S")?;
  Ok(date.timestamp())
}

/// 将聚类结果写入到指定的文件路径中。
/// 
/// # Panics
/// 若指定的文件路径不能够进行写操作，或者写操作遇到 
/// [ErrorKind](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variants)
/// 中的其他问题，将会引发 panic!
pub fn write_clusters_to_file(out_path: &str, ppl: &Vec<Cluster>) {
  let mut file = OpenOptions::new().write(true).create(true)
    .open(out_path).expect("File can't write");

  // 写入 csv 文件首行
  file.write_all(b"clusterIndex,Longitude,Latitude,SOG,COG\n")
    .expect("File can't write");

  for (index, cluster) in ppl.iter().enumerate() {
    for p in cluster.get_cluster() {
      let line = index.to_string() + "," + &p.get_longitude().to_string() + "," + &p.get_latitude().to_string() + ","
        + &p.get_sog().to_string() + "," + &p.get_cog().to_string() + "\n";
      file.write_all(line.as_bytes()).expect("File can't write");
    }
  }
}

/// 将运动点簇抽象的 GV 结果写入到指定的文件路径中。
/// 
/// # Panics
/// 若指定的文件路径不能够进行写操作，或者写操作遇到 
/// [ErrorKind](https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html#variants)
/// 中的其他问题，将会引发 panic!
pub fn write_gvs_to_file(out_path: &str, ppl: &Vec<Vec<GravityVector>>) {
  let mut file = OpenOptions::new().write(true).create(true)
    .open(out_path).expect("File can't write");

  // 写入 csv 文件首行
  file.write_all(b"clusterIndex,Longitude,Latitude,SOG,COG,MedianDistance\n")
    .expect("File can't write");

  for (index, pl) in ppl.iter().enumerate() {
    for p in pl {
      let line = index.to_string() + "," + &p.get_longitude().to_string() + "," + &p.get_latitude().to_string() + ","
        + &p.get_sog().to_string() + "," + &p.get_cog().to_string() + "," + &p.get_median_distance().to_string() + "\n";
      file.write_all(line.as_bytes()).expect("File can't write");    
    }
  }
}