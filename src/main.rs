use actix_web::{http::header, App, HttpServer};
use actix_cors::Cors;
use rust_test::funcs::transform_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let port = 8080;
    log::info!("starting HTTP server at http://localhost:{port}");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default() 
                .allowed_methods(vec!["POST"])
                .allowed_headers(vec![header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
            )
            .service(transform_handler)
    })
    .bind(("127.0.0.1", port))?
    .workers(2) // Allow concurrent processing
    .run()
    .await

    // No Cors
    // HttpServer::new(|| App::new().service(transform_handler))
    //     .bind(("127.0.0.1", port))? // localhost
    //     .workers(2) // Allow concurrent processing
    //     .run()
    //     .await
}
