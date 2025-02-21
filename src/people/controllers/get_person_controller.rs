use hyper::{header, Response, StatusCode};

use crate::{people::services::get_person_service::get_person_service, routes::routes::full, types::{ApiRequest, ApiResponse, Person}};

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub async fn get_person_controller(req: ApiRequest) -> ApiResponse {
  let person_id = req.uri().path().trim_start_matches("/people/");
  let person: Person = get_person_service(person_id).unwrap();

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