use uuid::Uuid;

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