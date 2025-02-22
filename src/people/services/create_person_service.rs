use crate::{dto::person_dto::PersonDTO, models::person::Person, types::ServiceError};

pub fn create_person_service(person_dto: PersonDTO) -> Result<PersonDTO, ServiceError> {   
    person_dto.name.clone().ok_or(ServiceError::InvalidInput("Name is required".to_string()))?;
    person_dto.age.clone().ok_or(ServiceError::InvalidInput("Age is required".to_string()))?;

    let created_person: PersonDTO = Person::create(person_dto).map_err(|e| ServiceError::InvalidInput(e.to_string()))?;

    Ok(created_person)
}