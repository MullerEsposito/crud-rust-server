use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonDTO {
  pub id: Option<Uuid>,
  pub name: Option<String>,
  pub age: Option<u8>,
}