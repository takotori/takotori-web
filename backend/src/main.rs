use actix_web::{get, web, App, HttpResponse, HttpServer};

#[get("/users")]
async fn list_users() -> HttpResponse {
    HttpResponse::ImATeapot().json("no coffee for you")
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // let pg_pool = postgres::create_pool();
    // postgres::migrate_up(&pg_pool).await;

    let address = address();
    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(pg_pool.clone()))
            .service(list_users)
    })
    .bind(&address)?
    .run()
    .await
}
