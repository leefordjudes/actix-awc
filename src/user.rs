use actix_web::{web, HttpResponse, HttpRequest, http::uri};
use serde::{Serialize};

use crate::error::Result;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: u32,
    user_name: String,
}

async fn get_user(req: HttpRequest) -> Result<HttpResponse> {
    let uri = req.uri();
    let path = req.path();
    let headers = req.headers();
    let host = headers.get("host").unwrap().to_str().unwrap().to_string();
    println!("uri: {:?}, path: {:?}, headers: {:?}, host: {:?}", &uri, &path, &headers, &host);
    let mut awc_uri = host.clone();
    awc_uri.push_str(&uri.to_string());
    println!("awc_uri: {}", &awc_uri);
    let client = awc::Client::new();
    let awc_req = client.get(awc_uri);//.append_header(headers.into());
    
    println!("awc Request: {:?}", awc_req);

    println!("get user");
    let user = User {id: 1, user_name: "auditplus".into()};
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(route_path: &str, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(format!("{}/get-user", route_path))
            .route(web::get().to(get_user))
    );
}