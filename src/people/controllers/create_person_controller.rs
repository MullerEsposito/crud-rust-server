use http_body_util::BodyExt;
use hyper::{body::Buf, header, Response, StatusCode};

use crate::{people::services::create_person_service::create_person_service, routes::routes::full, types::{ApiRequest, ApiResponse, Database, PersonDTO}};

pub async fn create_person_controller(req: ApiRequest, db: Database) -> ApiResponse {
  let whole_body = req.collect().await?.aggregate();
  let person_dto: PersonDTO = serde_json::from_reader(whole_body.reader())?;
  
  let person = create_person_service(person_dto, db.clone());

  let json = serde_json::to_string(&person)?;
  let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
  Ok(response)
}