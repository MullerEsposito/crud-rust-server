use uuid::Uuid;

use crate::{db::DB, types::Person};

impl Person {
  pub fn create(name: String, age: u8) -> Result<Person, String> {
    let new_person = Person {
        id: Uuid::new_v4(),
        name,
        age,
    };

    let mut db = DB.lock().map_err(|_| "Failed to lock database".to_string())?;
    db.insert(new_person.id, new_person.clone());

    Ok(new_person)
  }

  pub fn get_all() -> Vec<Person> {
    let db = DB.lock().unwrap();
    db.values().cloned().collect()
  }

  pub fn find_by_id(person_id: &Uuid) -> Option<Person> {
    let db = DB.lock().ok()?;
    db.get(person_id).cloned()
  }

  pub fn update_name(person_id: &Uuid, new_name: String) -> Result<(), String> {
    let mut db = DB.lock().map_err(|_| "Failed to lock database".to_string())?;
    if let Some(person) = db.get_mut(person_id) {
      person.name = new_name;
      Ok(())
    } else {
      Err("Person not found".to_string())
    }
  }

  pub fn update_age(person_id: &Uuid, new_age: u8) -> Result<(), String> {
    let mut db = DB.lock().map_err(|_| "Failed to lock database".to_string())?;
    if let Some(person) = db.get_mut(person_id) {
      person.age = new_age;
      Ok(())
    } else {
      Err("Person not found".to_string())
    }
  }
}
