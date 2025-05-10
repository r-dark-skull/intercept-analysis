use crate::models::RequestData;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct PocketBaseClient {
    client: Client,
    base_url: String,
    token: String,
}

impl PocketBaseClient {
    // Authenticate using email and password, and store the token
    pub async fn authenticate(
        base_url: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let login_payload = json!({
            "identity": email,
            "password": password
        });

        let res = client
            .post(format!(
                "{}/api/collections/_superusers/auth-with-password",
                base_url
            ))
            .json(&login_payload)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("Authentication failed with status: {}", res.status()).into());
        }

        let body = res.json::<HashMap<String, serde_json::Value>>().await?;
        let token = body
            .get("token")
            .and_then(|v| v.as_str())
            .ok_or("Token not found in response")?
            .to_string();

        Ok(PocketBaseClient {
            client,
            base_url: base_url.to_string(),
            token,
        })
    }

    pub async fn upload_url(
        &self,
        url_id: &str,
        scheme: &str,
        netloc: &str,
        path: &str,
        ip: Option<String>,
        port: u16,
    ) -> Result<(), Box<dyn Error>> {
        let url_data = json!({
            "id": url_id,
            "scheme": scheme,
            "netloc": netloc,
            "path": path,
            "ip": ip,
            "port": port
        });

        let response = self
            .client
            .post(format!("{}/api/collections/url/records", self.base_url))
            .bearer_auth(&self.token)
            .json(&url_data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Error uploading URL: {}", response.status()).into())
        }
    }

    pub async fn upload_request(
        &self,
        url_id: &str,
        request_data: &RequestData,
    ) -> Result<(), Box<dyn Error>> {
        let request_json = json!({
            "time": request_data.time,
            "url_id": url_id,
            "method": request_data.method,
            "url_parameters": request_data.url_parameters,
            "response_status": request_data.response_status,
            "request_headers": request_data.request_headers,
            "request_body": request_data.request_body,
            "response_headers": request_data.response_headers,
            "response_body": request_data.response_body,
            "response_length": request_data.response_length
        });

        let response = self
            .client
            .post(format!(
                "{}/api/collections/intercepted_requests/records",
                self.base_url
            ))
            .bearer_auth(&self.token)
            .json(&request_json)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Error uploading request: {}", response.status()).into())
        }
    }

    pub async fn get_stats(&self) -> Result<(usize, usize), Box<dyn Error>> {
        let url_count = self.get_collection_count("url").await?;
        let request_count = self.get_collection_count("intercepted_requests").await?;
        Ok((url_count, request_count))
    }

    async fn get_collection_count(&self, collection_name: &str) -> Result<usize, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "{}/api/collections/{}/records",
                self.base_url, collection_name
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.json::<serde_json::Value>().await?;
            let count = body["totalItems"].as_u64().unwrap_or(0) as usize;
            Ok(count)
        } else {
            Err(format!(
                "Error fetching {} collection count: {}",
                collection_name,
                response.status()
            )
            .into())
        }
    }
}
