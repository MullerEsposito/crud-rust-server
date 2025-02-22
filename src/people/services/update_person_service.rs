use uuid::Uuid;

use crate::{dto::person_dto::PersonDTO, models::person::Person, types::ServiceError};

pub fn update_person_service(id: String, person_dto: PersonDTO) -> Result<Person, ServiceError> {
  let uuid = Uuid::parse_str(&id).unwrap();

  if Person::find_by_id(&uuid).is_none() {
    return Err(ServiceError::NotFound("Person not found".to_string()));
  }

  if let Some(name) = person_dto.name {
    if let Err(e) = Person::update_name(&uuid, name) {
        return Err(ServiceError::DatabaseError(e.to_string()));
    }
  }

  if let Some(age) = person_dto.age {
    if let Err(e) = Person::update_age(&uuid, age) {
        return Err(ServiceError::DatabaseError(e.to_string()));
    }}

  let updated_person = Person::find_by_id(&uuid).unwrap();

  Ok(updated_person)
}