//! Uuid 簇，聚类 uuid 的结果
use uuid::Uuid;

/// 与普通簇不同的是，Uuid 簇只存放 uuid。
/// 到了输出时，再根据 Uuid 簇生成普通簇，将 uuid 映射成点。 
/// 同普通簇，Uuid 簇也只是存放读引用。
pub struct UuidCluster<'a> {
  uuid_cluster: Vec<&'a Uuid>
}

impl<'a> UuidCluster<'a> {
  pub fn new(uuid_cluster: Vec<&'a Uuid>) -> Self {
    Self {
      uuid_cluster
    }
  }

  pub fn get_cluster(&self) -> &Vec<&Uuid> {
    &self.uuid_cluster
  }
}