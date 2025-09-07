use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    if let Some(peer_addr) = req.connection_info().peer_addr() {
        format!("{}", peer_addr)
    } else {
        "Could not determine client IP".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:6666")?
    .run()
    .await
}
