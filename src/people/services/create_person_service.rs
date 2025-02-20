use uuid::Uuid;

use crate::types::{Database, Person, PersonDTO};

pub fn create_person_service(person_dto: PersonDTO, db: Database) -> Person {    
    let mut person = Person {
        id: Uuid::new_v4(),
        name: person_dto.name,
        age: person_dto.age,
    };
    person.id = Uuid::new_v4();

    let mut db = db.lock().unwrap();
    db.insert(person.id, person.clone());

    person
}