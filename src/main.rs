mod routes;
mod types;
mod people;
mod utils;
mod db;
mod models;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use std::net::SocketAddr;

use routes::routes::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { 
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = TcpListener::bind(addr).await?;
  
  println!("Listening on http://{}", addr);
  loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    tokio::task::spawn(async move {
      let service = service_fn(routes);

      if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
        println!("Failed to serve connection: {:?}", err);
      }
    });
    
  }
}