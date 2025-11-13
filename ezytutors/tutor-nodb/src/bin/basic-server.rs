use std::io;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // "move" can be omitted here because there's nothing to capture
    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

pub fn general_routes(_cfg: &mut web::ServiceConfig) { // "pub" can be omitted
}
