use crate::models::person::Person;

pub fn get_people_service() -> Vec<Person> {
  let people: Vec<Person> = Person::get_all();
  people
}