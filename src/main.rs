use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct IpInfo {
    client_ip: String,
    forwarded_ip: Option<String>,
    user_agent: Option<String>,
}

async fn get_ip_info(req: HttpRequest) -> impl Responder {
    let client_ip = req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let forwarded_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .filter(|s| !s.is_empty());

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|hv| hv.to_str().ok())
        .map(String::from);

    let ip_info = IpInfo {
        client_ip,
        forwarded_ip,
        user_agent,
    };

    web::Json(ip_info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:4013");

    HttpServer::new(|| {
        App::new()
            .route("/ipinfo", web::get().to(get_ip_info))
    })
    .bind("127.0.0.1:4013")?
    .run()
    .await
}
