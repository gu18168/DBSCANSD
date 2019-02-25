use crate::{
  models::{
    cluster::Cluster,
    trajectory_point::TrajectoryPoint
  },
  dbscan_utility::is_density_reachable
};

pub fn apply_dbscansd(
  points: &mut Vec<TrajectoryPoint>,
  eps: f64,
  min_points: i32,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> Vec<Cluster>
{
  let mut index = 0;
  let mut result_clusters: Vec<Cluster> = Vec::new();
  let len = points.len();

  while index < len {
    let mut cluster_raw: Vec<TrajectoryPoint> = Vec::new();
    let point: &TrajectoryPoint = points.get(index).unwrap();

    if point.is_visited && index != len - 1 { continue };

    let mut count = 0;
    for p in points.iter() {
      if is_density_reachable(p, point, eps, max_spd, max_dir, is_stop_point) {
        count += 1;

        if !cluster_raw.contains(p) {
          cluster_raw.push(p.clone());
        }
      }
    }

    if count >= min_points {
      let point = points.get_mut(index).unwrap();
      point.is_visited = true;
      point.is_core_point = true;

      result_clusters.push(Cluster::new(cluster_raw));
    }
    
    if index == (len - 1) {
      let mut length = result_clusters.len();
      let mut flag = true;
      let mut i = 0;
      let mut j = 0;

      // 感觉非常不优雅，思考一下能不能优化
      while flag {
        flag = false;

        while i < length {
          while j < length {
            if i != j {
              if i == length {
                flag = true;
                continue;
              }

              let to_merge = is_merge(
                result_clusters.get(i).unwrap(), 
                result_clusters.get(j).unwrap()
              );

              if to_merge {
                let c_bef = result_clusters.get(j).unwrap().get_cluster().clone();
                for p in c_bef {
                  let c_aft = result_clusters.get_mut(i).unwrap().get_mut_cluster();
                  if !c_aft.contains(&p) {
                    c_aft.push(p.clone());
                  }
                }
                result_clusters.remove(j);
                j -= 1;
                length -= 1;
              }
            }

            j += 1;
          }
          i += 1;
        }
      }
    }

    index += 1;
  }

  result_clusters
}

fn is_merge(c1: &Cluster, c2: &Cluster) -> bool {
  let points_c1 = c1.get_cluster();
  let points_c2 = c2.get_cluster();

  if points_c1.len() == 0 || points_c2.len() == 0 {
    return false;
  }

  for p in points_c2 {
    if p.is_core_point && points_c1.contains(p) {
      return true;
    }
  }

  false
}