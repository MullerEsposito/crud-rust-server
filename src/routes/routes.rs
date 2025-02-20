use http_body_util::{Full, BodyExt};
use hyper::{body::Bytes, Response, StatusCode};
use crate::types::{ApiRequest, ApiResponse, BoxBody, Database};

use super::people_routes::people_routes;

pub async fn routes(req: ApiRequest, db: Database) -> ApiResponse {
  let path = req.uri().path();
   
  if path.starts_with("/people") {
    return people_routes(req, db).await;
  }
  
  notfound_response().await  
}

static NOTFOUND: &[u8] = b"Oops! Not Found";

pub async fn notfound_response() -> ApiResponse {  
  Ok(Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(full(NOTFOUND))
    .unwrap())
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
  Full::new(chunk.into())
  .map_err(|never| match never {})
  .boxed()
}