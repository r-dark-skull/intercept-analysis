use crate::models::{Request, Url};
use quick_xml::de::from_str;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::str;
use std::{collections::HashMap, error::Error};

use base64::{Engine as _, engine::general_purpose};
use url::Url as UrlParser;

#[derive(Debug, Deserialize)]
struct BurpLog {
    #[serde(rename = "item")]
    pub items: Vec<BurpItem>,
}

#[derive(Debug, Deserialize)]
struct BurpItem {
    pub time: Option<String>,
    pub url: String,
    pub host: HostField,
    pub port: u16,
    // pub protocol: String,
    pub method: Option<String>,
    // pub path: Option<String>,
    #[serde(rename = "request")]
    pub request: Base64Content,
    pub status: Option<u16>,
    #[serde(rename = "responselength")]
    pub response_length: Option<u32>,
    #[serde(rename = "response")]
    pub response: Option<Base64Content>,
}

#[derive(Debug, Deserialize)]
struct HostField {
    #[serde(rename = "@ip")]
    pub ip: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Base64Content {
    #[serde(rename = "$value")]
    pub value: String,
}

pub fn parse_burp_xml(data: &[u8]) -> Result<String, Box<dyn Error>> {
    let xml_str = str::from_utf8(data)?;
    let parsed: BurpLog = from_str(xml_str)?;

    let mut urls = vec![];
    let mut requests = vec![];

    for item in parsed.items {
        // Parse and normalize URL
        let parsed_url = UrlParser::parse(&item.url)?;
        let scheme = parsed_url.scheme().to_string();
        let netloc = parsed_url.host_str().unwrap_or("").to_string();
        let path = parsed_url.path().to_string();
        let query = parsed_url.query().map(str::to_string);

        let url_id = compute_url_id(&scheme, &netloc, &path);

        let url_model = Url {
            id: url_id.clone(),
            scheme,
            netloc,
            path,
            ip: item.host.ip,
            port: item.port,
        };
        urls.push(url_model);

        // Decode and parse request
        let (req_headers, req_body) = decode_and_split(&item.request);
        let (res_headers, res_body) = match item.response {
            Some(resp) => decode_and_split(&resp),
            None => (vec![], "".to_string()),
        };

        let mut request_headers: HashMap<String, String> = HashMap::new();
        let mut response_headers: HashMap<String, String> = HashMap::new();

        for header in req_headers.iter() {
            let parts: Vec<&str> = header.splitn(2, ": ").collect();
            if parts.len() == 2 {
                request_headers.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        for header in res_headers.iter() {
            let parts: Vec<&str> = header.splitn(2, ": ").collect();
            if parts.len() == 2 {
                response_headers.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        let req_model = Request {
            time: item.time,
            url_id: url_id.clone(),
            method: item.method,
            url_parameters: query,
            response_status: item.status,
            request_headers: request_headers,
            request_body: req_body,
            response_headers: response_headers,
            response_body: res_body,
            response_length: item.response_length,
        };

        requests.push(req_model);
    }

    let output = json!({
        "urls": urls,
        "requests": requests
    });

    Ok(serde_json::to_string_pretty(&output)?)
}

fn compute_url_id(scheme: &str, netloc: &str, path: &str) -> String {
    let full = format!("{}://{}{}", scheme, netloc, path);
    let mut hasher = Sha256::new();
    hasher.update(full.as_bytes());
    hex::encode(hasher.finalize())
}

fn decode_and_split(field: &Base64Content) -> (Vec<String>, String) {
    match general_purpose::STANDARD.decode(&field.value) {
        Ok(bytes) => {
            let decoded = String::from_utf8_lossy(&bytes);
            let mut lines = decoded.lines();
            let mut headers = vec![];

            while let Some(line) = lines.next() {
                if line.trim().is_empty() {
                    break;
                }
                headers.push(line.to_string());
            }

            let body: String = lines.collect::<Vec<_>>().join("\n");
            (headers, body)
        }
        Err(_) => (vec!["<invalid base64>".to_string()], "".to_string()),
    }
}
