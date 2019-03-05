//! 可合并的簇索引，帮助多线程执行簇合并
use std::collections::HashMap;

/// 实际上，这个数据结构起到的是 Fn(index) -> merge_index 的作用，
/// 输出参数 index 是簇的索引，得到的 merge_index 是与该簇合并的簇索引。
/// 
/// 例如我们已知: Fn(0) -> 0, Fn(1) -> 0, Fn(2) -> 2, Fn(3) -> 0。
/// 那么其实我们就可以知道，簇0，簇1 与簇3 能够合并。
/// 
/// 由于 index 与 merge_index 都是 usize 属性，所以使用 Vec 即可。
pub struct MergeIndexs {
  merge_indexs: Vec<usize>,
}

impl MergeIndexs {
  pub fn new() -> Self {
    Self {
      merge_indexs: Vec::new(),
    }
  }

  /// 得到指定索引的合并索引
  pub fn get(&self, index: usize) -> &usize {
    self.merge_indexs.get(index).unwrap()
  }

  /// 得到指定索引中的最小簇索引
  fn find_min(&self, indexs: &Vec<usize>) -> usize {
    let mut res = <usize>::max_value();
    for index in indexs {
      let val = self.merge_indexs.get(*index).unwrap();
      if *val < res {
        res = *val;
      }
    }
    res
  }

  /// 将指定索引设置为这些索引中的最小值
  /// 
  /// # Ex.
  /// 已知: Fn(1) -> 1, Fn(2) -> 2，且簇3 能够与簇1 ，簇2 合并。
  /// 那么 Fn(1) = Fn(2) = Fn(3) = min(Fn(1), Fn(2), Fn(3))。
  pub fn set_to_min(&mut self, indexs: &Vec<usize>) {
    let min = self.find_min(indexs);
    for index in indexs {
      self.merge_indexs[*index] = min;
    }
    self.push(min);
  }

  pub fn push(&mut self, val: usize) {
    self.merge_indexs.push(val);
  }

  /// 将可合并的簇索引都提取出来
  /// 
  /// # Ex.
  /// 已知 Fn(1) -> 1, Fn(2) -> 1, Fn(3) -> 3。
  /// 那么得到的结果将是 [[1, 2], [3]]。
  pub fn map_indexs(&self) -> Vec<Vec<usize>> {
    // 已知 Fn(a) -> b
    // b_to_index 起到 Fn(b) -> a 的作用
    let mut b_to_index: HashMap<usize, usize> = HashMap::new();
    let mut results: Vec<Vec<usize>> = Vec::new();

    for (a, b) in self.merge_indexs.iter().enumerate() {
      if let Some(index) = b_to_index.get(b) {
        results[*index].push(a);
      } else {
        results.push(vec![a]);
        b_to_index.insert(*b, results.len() - 1);
      }
    }

    results
  }
}
