use hyper::{body::Buf, header, Response, StatusCode};
use http_body_util::BodyExt;

use crate::{dto::person_dto::PersonDTO, models::person::Person, people::services::update_person_service::update_person_service, routes::routes::full, types::{ApiRequest, ApiResponse}};

static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub async fn update_person_controller(req: ApiRequest) -> ApiResponse {
  let person_id = req.uri().path().trim_start_matches("/people/").to_string();
  let whole_body = req.collect().await?.aggregate();
  let person_dto: PersonDTO = serde_json::from_reader(whole_body.reader())?;

  let updated_person: Person = update_person_service(person_id, person_dto).unwrap();

  let response = match serde_json::to_string(&updated_person) {
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