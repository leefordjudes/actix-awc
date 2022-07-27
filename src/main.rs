use actix_cors::Cors;
use actix_http::HttpMessage;
use actix_web::{middleware::{Logger, NormalizePath, TrailingSlash}, web, App, HttpResponse, HttpServer, HttpRequest};
use futures::{future, Stream};

mod user;
mod error;
use error::Result;
use crate::error::{Error, ErrorKind};

async fn proxy_routes_with_payload(req: HttpRequest, client: web::Data<awc::Client>, payload: web::Json<serde_json::Value>) -> Result<HttpResponse> {
    println!("proxy routes");
    let path = format!("http://localhost:3000{}",req.path());
    let awc_req = client.request_from(path, req.head());
    let mut resp = awc_req.send_json(&data).await.unwrap();
    let x:serde_json::Value = resp.json().await.unwrap();
    Ok(HttpResponse::Ok().json(x))
}

async fn proxy_routes(req: HttpRequest, client: web::Data<awc::Client>) -> Result<HttpResponse> {
    println!("proxy routes");
    let path = format!("http://localhost:3000{}",req.path());
    let awc_req = client.request_from(path, req.head());
    let mut resp = awc_req.send().await.unwrap();
    let x:serde_json::Value = resp.json().await.unwrap();
    Ok(HttpResponse::Ok().json(x))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT is required");
    let server = HttpServer::new(move || {
    let client = awc::Client::new();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/{tail:.*}", web::get().to(proxy_routes))
            .route("/{tail:.*}", web::post().to(proxy_routes_with_payload))
            .route("/{tail:.*}", web::put().to(proxy_routes_with_payload))
    })
        .bind(format!("0.0.0.0:{}", listen_port))?
        .run();
    let print_message = async {
        println!("Server started successfully on port {}", listen_port);
    };
    let _ = future::join(server, print_message).await;
    Ok(())
}
