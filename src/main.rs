mod routes;
mod types;
mod people;
mod utils;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::SocketAddr;

use routes::routes::routes;
use types::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { 
  let db: Database = Arc::new(Mutex::new(HashMap::new()));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = TcpListener::bind(addr).await?;
  
  println!("Listening on http://{}", addr);
  loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    let db = db.clone();
    tokio::task::spawn(async move {
      let service = service_fn(move |req| routes(req, db.clone()));

      if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
        println!("Failed to serve connection: {:?}", err);
      }
    });
    
  }
}