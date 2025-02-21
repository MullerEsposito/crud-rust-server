use crate::types::{Person, PersonDTO, ServiceError};

pub fn create_person_service(person_dto: PersonDTO) -> Result<Person, ServiceError> {   
    let name = person_dto.name.ok_or(ServiceError::InvalidInput("Name is required".to_string()))?;
    let age = person_dto.age.ok_or(ServiceError::InvalidInput("Age is required".to_string()))?;

    let created_person: Person = Person::create(name, age).map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    Ok(created_person)
}