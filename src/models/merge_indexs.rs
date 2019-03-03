use std::collections::HashMap;

pub struct MergeIndexs {
  merge_indexs: Vec<usize>,
}

impl MergeIndexs {
  pub fn new() -> Self {
    Self {
      merge_indexs: Vec::new(),
    }
  }

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

  pub fn map_indexs(&self) -> Vec<Vec<usize>> {
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
