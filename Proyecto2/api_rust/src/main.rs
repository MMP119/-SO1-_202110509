use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct WeatherInput {
    description: String,
    country: String,
    weather: String,
}

async fn input_handler(item: web::Json<WeatherInput>) -> impl Responder {
    println!("input: {:?}", item);
    HttpResponse::Ok().json(item.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("iniciando en http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/input", web::post().to(input_handler)) // se define la ruta /input
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
