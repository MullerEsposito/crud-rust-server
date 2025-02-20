use hyper::Method;

use crate::{people::controllers::{create_person_controller::create_person_controller, get_people_controller::get_people_controller}, types::{ApiRequest, ApiResponse, Database}};

use super::routes::notfound_response;

pub async fn people_routes(req: ApiRequest, db: Database) -> ApiResponse {
  let method = req.method();
  let path = req.uri().path();

  match (method, path) {
      (&Method::GET, "/people") => get_people_controller(req, db).await,
      (&Method::POST, "/people") => create_person_controller(req, db).await,
      // (&Method::GET, "/people/{id}") => get_person_controller(req),
      // (&Method::PUT, "/people/{id}") => update_person_controller(req),
      // (&Method::DELETE, "/people/{id}") => delete_person_controller(req),
      _ => notfound_response().await,
  }
}