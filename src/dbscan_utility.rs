use crate::{
  models::{
    trajectory_point::TrajectoryPoint
  }
};

fn cal_distance_bwt_two_points(p1: &TrajectoryPoint, p2: &TrajectoryPoint) -> f64 {
  let dx: f64 = p1.get_longitude() - p2.get_longitude();
  let dy: f64 = p1.get_latitude() - p2.get_latitude();

  let distance = (dx * dx + dy * dy).sqrt();

  distance
}

pub fn is_density_reachable(
  p1: &TrajectoryPoint, 
  p2: &TrajectoryPoint,
  eps: f64,
  max_spd: f64,
  max_dir: f64,
  is_stop_point: bool) -> bool 
{
  if cal_distance_bwt_two_points(p1, p2) <= eps {
    if is_stop_point { return true; }

    if (p1.get_cog() - p2.get_cog()).abs() < max_dir {
      if (p1.get_sog() - p2.get_sog()).abs() < max_spd {
        return true;
      }
    }
  }

  false
}