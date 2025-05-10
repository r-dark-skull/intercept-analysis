mod models;
mod parser;

use actix_multipart::Multipart;
use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use futures_util::stream::StreamExt as _;
use parser::parse_burp_xml;

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    while let Some(Ok(mut field)) = payload.next().await {
        let name = field.name();
        if name == "file" {
            let mut bytes = Vec::new();
            while let Some(Ok(chunk)) = field.next().await {
                bytes.extend_from_slice(&chunk);
            }

            match parse_burp_xml(&bytes) {
                Ok(json_output) => {
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json_output);
                }
                Err(e) => return HttpResponse::BadRequest().body(format!("Parsing failed: {}", e)),
            }
        }
    }
    HttpResponse::BadRequest().body("No file field found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on http://127.0.0.1:8000");
    HttpServer::new(|| App::new().service(upload))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
