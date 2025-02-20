use crate::types::{Database, Person};

pub fn get_people_service(db: Database) -> Vec<Person> {
  let db = db.lock().unwrap();
  let people: Vec<Person> = db.values().cloned().collect();
  people
}