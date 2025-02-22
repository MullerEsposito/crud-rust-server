use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
  pub id: Uuid,
  pub name: String,
  pub age: u8,
}