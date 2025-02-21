use hyper::{header, Response, StatusCode};

use crate::{people::services::get_people_service::get_people_service, routes::routes::full, types::{ApiRequest, ApiResponse, Person}};

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub async fn get_people_controller(_req: ApiRequest) -> ApiResponse {
  let people: Vec<Person> = get_people_service();
  let response = match serde_json::to_string(&people) {
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