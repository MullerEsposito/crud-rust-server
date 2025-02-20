use crate::types::{Database, Person};

use uuid::Uuid;

pub fn get_person_service(db: Database, id: &str) -> Option<Person> {
  let db = db.lock().unwrap();
  let uuid = Uuid::parse_str(id).ok()?;
  let person = db.get(&uuid).cloned();

  person
}