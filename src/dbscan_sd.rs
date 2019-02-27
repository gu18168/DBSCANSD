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

    // 这句话是不是没有意义，因为 point 不可能是 is_visited 
    // is_visited 是在下面被赋值 true 的，所以无意义
    // 且目前 is_visited 这个标识符就是无意义，按顺序全遍历保证了所有点被遍历到
    if point.is_visited && index != len - 1 { index += 1; continue };

    // count 是否可以由 cluster_raw.len() 代替
    // 目前 cluster_raw 的逻辑是有相同的点就不放入，但是时间戳不同就不会认为是相同
    // 所以相同的条件过于苛刻，可以认为都不相同，直接都放入 cluster_raw 中
    // 然后就可以取代 count 的作用
    let mut count = 0;
    // 主要是这一个全遍历然后还得算非常耗时间
    // 是不是可以考虑多线程来运算？将一个点的聚类分给一个线程来进行
    // 当全部点跑完之后，再让主合并聚类结果
    // * 多线程主要的竞争点在于 result_clusters 的共享
    // * 其他的数据由于只是读，所以很安全
    for p in points.iter() {
      if is_density_reachable(p, point, eps, max_spd, max_dir, is_stop_point) {
        count += 1;

        if !cluster_raw.contains(p) {
          // clone 的代价感觉还是比较高的，能否用 uuid 来代替整艘船
          // 然后在输出的时候利用 uuid 得到船真正的信息
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
    
    // 可以移到循环外，因为这一步就是等循环结束了进行
    // 移到循环外，可以减少判断以及对于 index 的维护
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
                // 能不能不要在同一个 Vec 进行操作，感觉很危险，虽然使用了 Cow
                // 思路如下：
                // 1. 分成两个 Vec ，一个是旧的，一个是新的
                // 2. 遍历旧的，然后元素与新的比较，若都不能合并就加入到新的末尾
                // 3. 若能合并，不要直接合并，而是与整个新的比较，获得能合并的索引数组，将这些一起合并成一个
                // 4. 上述步骤能保证，新 Vec 中的元素不能互相合并
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

    println!("{}", index);
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