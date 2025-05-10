use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct BurpLog {
    #[serde(rename = "item")]
    pub items: Vec<BurpItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BurpItem {
    pub time: Option<String>,
    pub url: String,

    #[serde(rename = "host")]
    pub host: HostField,

    pub port: u16,
    pub protocol: String,
    pub method: Option<String>,
    pub path: Option<String>,
    pub extension: Option<String>,

    #[serde(rename = "request")]
    pub request: Base64Content,

    pub status: Option<u16>,
    #[serde(rename = "responselength")]
    pub response_length: Option<u32>,
    pub mimetype: Option<String>,

    #[serde(rename = "response")]
    pub response: Option<Base64Content>,

    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostField {
    #[serde(rename = "@ip")]
    pub ip: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Base64Content {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "@base64")]
    pub base64: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Url {
    pub id: String,
    pub scheme: String,
    pub netloc: String,
    pub path: String,
    pub ip: Option<String>,
    pub port: u16,
}

#[derive(Debug, Serialize)]
pub struct Request {
    pub time: Option<String>,
    pub url_id: String,
    pub method: Option<String>,
    pub url_parameters: Option<String>,
    pub response_status: Option<u16>,
    pub request_headers: HashMap<String, String>,
    pub request_body: String,
    pub response_headers: HashMap<String, String>,
    pub response_body: String,
    pub response_length: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestData {
    // pub url_id: String,
    pub time: Option<String>,
    pub method: Option<String>,
    pub url_parameters: Option<String>,
    pub response_status: Option<u16>,
    pub request_headers: HashMap<String, String>, // Change from Vec<String> to HashMap<String, String>
    pub request_body: String,
    pub response_headers: HashMap<String, String>, // Change from Vec<String> to HashMap<String, String>
    pub response_body: String,
    pub response_length: Option<u32>,
}
