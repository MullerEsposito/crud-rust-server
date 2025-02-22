use std::fmt;
use hyper::body::{Bytes, Incoming as IncomingBody};
use hyper::{Request, Response};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::person::Person;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub type Database = Arc<Mutex<HashMap<Uuid, Person>>>;
pub type ApiResponse = Result<Response<BoxBody>>;
pub type ApiRequest = Request<IncomingBody>;


#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    InvalidInput(String),
    DatabaseError(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ServiceError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            ServiceError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

