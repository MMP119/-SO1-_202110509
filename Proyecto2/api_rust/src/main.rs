use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct WeatherInput {
    description: String,
    country: String,
    weather: String,
}

async fn input_handler(item: web::Json<WeatherInput>) -> impl Responder {
    println!("input recibido en Rust: {:?}", item);

    //reenviar al servicio en Go
    let client = Client::new();
    let result = client
        .post("http://api-http-service:8081/input") // nombre del service de Go
        .json(&*item)
        .send()
        .await;

    match result {
        Ok(resp) => {
            println!("Respuesta de Go API: {:?}", resp.status());
            HttpResponse::Ok().body("Reenviado correctamente al servicio Go")
        }
        Err(err) => {
            eprintln!("Error al reenviar: {:?}", err);
            HttpResponse::InternalServerError().body("Error al reenviar al servicio Go")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("API Rust escuchando en http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .route("/input", web::post().to(input_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
