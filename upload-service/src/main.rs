mod models;
mod parser;
mod pocketbase;

use crate::models::RequestData;
use actix_multipart::Multipart;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, post};
use futures_util::stream::StreamExt as _;
use parser::parse_burp_xml;
use pocketbase::PocketBaseClient;
use serde_json::json;

#[post("/upload")]
async fn upload(mut payload: Multipart, req: HttpRequest) -> impl Responder {
    // print the headers of recieved request
    for (key, value) in req.headers().iter() {
        println!("{}: {:?}", key, value);
    }

    while let Some(Ok(mut field)) = payload.next().await {
        let name = field.name();
        if name == "file" {
            let mut bytes = Vec::new();
            while let Some(Ok(chunk)) = field.next().await {
                bytes.extend_from_slice(&chunk);
            }

            match parse_burp_xml(&bytes) {
                Ok(parsed_json) => {
                    // Upload URLs and Requests to PocketBase
                    let parsed: serde_json::Value = match serde_json::from_str(&parsed_json) {
                        Ok(value) => value,
                        Err(e) => {
                            return HttpResponse::BadRequest()
                                .body(format!("JSON parsing failed: {}", e));
                        }
                    };

                    let empty_vec = vec![];
                    let urls = parsed["urls"].as_array().unwrap_or(&empty_vec);
                    let requests = parsed["requests"].as_array().unwrap_or(&empty_vec);

                    let pb_client = match PocketBaseClient::authenticate(
                        "http://pocketbase-service:8090",
                        "admin@localhost.local",
                        "localadmin",
                    )
                    .await
                    {
                        Ok(client) => client,
                        Err(e) => {
                            return HttpResponse::InternalServerError()
                                .body(format!("Authentication failed: {}", e));
                        }
                    };

                    // Upload all URLs
                    for url in urls {
                        let url_data = url.as_object().unwrap();
                        let url_id = url_data["id"].as_str().unwrap();
                        let result = pb_client
                            .upload_url(
                                url_id,
                                url_data["scheme"].as_str().unwrap(),
                                url_data["netloc"].as_str().unwrap(),
                                url_data["path"].as_str().unwrap(),
                                url_data["ip"].as_str().map(|s| s.to_string()),
                                url_data["port"].as_u64().unwrap() as u16,
                            )
                            .await;
                        if let Err(e) = result {
                            println!("Error uploading URL {}: {}", url_id, e);
                        }
                    }

                    for url in urls {
                        match url.as_object() {
                            Some(url_data) => {
                                let url_id = match url_data["id"].as_str() {
                                    Some(id) => id,
                                    None => {
                                        println!("Error: Missing id field for URL");
                                        continue;
                                    }
                                };

                                let result = pb_client
                                    .upload_url(
                                        url_id,
                                        url_data["scheme"].as_str().unwrap_or(""),
                                        url_data["netloc"].as_str().unwrap_or(""),
                                        url_data["path"].as_str().unwrap_or(""),
                                        url_data["ip"].as_str().map(|s| s.to_string()),
                                        url_data["port"].as_u64().unwrap_or(0) as u16,
                                    )
                                    .await;

                                if let Err(e) = result {
                                    println!("Error uploading URL {}: {}", url_id, e);
                                }
                            }
                            None => println!("Error: Invalid URL object format"),
                        }
                    }

                    // Upload all Requests
                    for request in requests {
                        let request_data = request.as_object().unwrap();
                        let req = RequestData {
                            time: request_data["time"].as_str().map(|s| s.to_string()),
                            method: request_data["method"].as_str().map(|s| s.to_string()),
                            url_parameters: request_data["url_parameters"]
                                .as_str()
                                .map(|s| s.to_string()),
                            response_status: request_data["response_status"]
                                .as_u64()
                                .map(|n| n as u16),
                            request_headers: request_data["request_headers"]
                                .as_object()
                                .map(|obj| {
                                    obj.iter()
                                        .map(|(k, v)| {
                                            (k.clone(), v.as_str().unwrap_or("").to_string())
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            request_body: request_data["request_body"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            response_headers: request_data["response_headers"]
                                .as_object()
                                .map(|obj| {
                                    obj.iter()
                                        .map(|(k, v)| {
                                            (k.clone(), v.as_str().unwrap_or("").to_string())
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            response_body: request_data["response_body"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            response_length: request_data["response_length"]
                                .as_u64()
                                .map(|n| n as u32),
                        };
                        let result = pb_client
                            .upload_request(
                                &request_data["url_id"].as_str().unwrap_or("").to_string(),
                                &req,
                            )
                            .await;
                        if let Err(e) = result {
                            println!("Error uploading request: {}", e);
                        }
                    }

                    // Fetch stats
                    let (url_count, request_count) = pb_client.get_stats().await.unwrap();

                    return HttpResponse::Ok().json(json!({
                        "url_count": url_count,
                        "request_count": request_count,
                    }));
                }
                Err(e) => return HttpResponse::BadRequest().body(format!("Parsing failed: {}", e)),
            }
        }
    }
    HttpResponse::BadRequest().body("No file field found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on http://0.0.0.0:8000");
    HttpServer::new(|| App::new().service(upload))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
