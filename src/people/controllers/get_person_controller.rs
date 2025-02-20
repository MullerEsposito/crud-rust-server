use hyper::{header, Response, StatusCode};

use crate::{people::services::get_person_service::get_person_service, routes::routes::full, types::{ApiRequest, ApiResponse, Database, Person}};

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub async fn get_person_controller(req: ApiRequest, db: Database) -> ApiResponse {
  let person_id = req.uri().path().trim_start_matches("/people/");
  print!("person_id: {}", person_id);
  let person: Person = get_person_service(db, person_id).unwrap();

  let response = match serde_json::to_string(&person) {
    Ok(json) => Response::builder()
      .header(header::CONTENT_TYPE, "application/json")
      .body(full(json))
      .unwrap(),
    Err(_) => Response::builder()
      .status(StatusCode::INTERNAL_SERVER_ERROR)
      .body(full(INTERNAL_SERVER_ERROR))
      .unwrap(),
  };

  Ok(response)
}