use http_body_util::BodyExt;
use hyper::{body::Buf, header, Response, StatusCode};

use crate::{dto::person_dto::PersonDTO, people::services::create_person_service::create_person_service, routes::routes::full, types::{ApiRequest, ApiResponse}};

pub async fn create_person_controller(req: ApiRequest) -> ApiResponse {
  let whole_body = req.collect().await?.aggregate();
  let person_dto: PersonDTO = serde_json::from_reader(whole_body.reader())?;
  
  match create_person_service(person_dto) {
    Ok(person_dto) => {
      let json = serde_json::to_string(&person_dto)?;
      let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
      Ok(response)
    },
    Err(e) => {
      let response = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(serde_json::to_string(&e.to_string())?))?;
      Ok(response)
    }
  }
}