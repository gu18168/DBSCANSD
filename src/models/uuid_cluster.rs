use uuid::Uuid;

pub struct UuidCluster {
  uuid_cluster: Vec<Uuid>
}

impl UuidCluster {
  pub fn new(uuid_cluster: Vec<Uuid>) -> Self {
    Self {
      uuid_cluster
    }
  }

  pub fn get_cluster(&self) -> &Vec<Uuid> {
    &self.uuid_cluster
  }
}