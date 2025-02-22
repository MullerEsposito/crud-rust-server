use uuid::Uuid;

use crate::models::person::Person;

pub fn get_person_service(id: &str) -> Option<Person> {
  let uuid = Uuid::parse_str(id).ok()?;
  let person = Person::find_by_id(&uuid);

  person
}