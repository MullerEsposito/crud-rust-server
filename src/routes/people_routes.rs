use hyper::Method;

use crate::{people::controllers::{create_person_controller::create_person_controller, get_people_controller::get_people_controller, get_person_controller::get_person_controller, update_person_controller::update_person_controller}, types::{ApiRequest, ApiResponse}};

use super::routes::notfound_response;

pub async fn people_routes(req: ApiRequest) -> ApiResponse {
  let method = req.method();
  let path = req.uri().path();

  match (method, path) {
    (&Method::GET, path) if path.starts_with("/people/") => {
      get_person_controller(req).await
    }, 
    (&Method::PUT, path) if path.starts_with("/people/") => {
      update_person_controller(req).await
    },
    (&Method::GET, "/people") => get_people_controller(req).await,
    (&Method::POST, "/people") => create_person_controller(req).await,
    // (&Method::DELETE, "/people/{id}") => delete_person_controller(req),
    _ => notfound_response().await,
  }
}