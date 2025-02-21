use tauri_plugin_http::reqwest::header::HeaderMap;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_http::reqwest;
use serde_json::Value;
use std::error::Error;

pub enum Method {
    GET,
    POST,
}

pub struct RequestInit {
    method: Method,
    header: Option<Value>,
    body: Option<Value>,
}

impl RequestInit {
    pub fn new(init: Value) -> Result<Self, Box<dyn Error>> {
        if !init.is_object() {
            return Err("init must be a JSON object".into());
        }

        let method = match init.get("method").and_then(|v| v.as_str()) {
            Some("GET") => Method::GET,
            Some("POST") => Method::POST,
            _ => return Err("Invalid or missing method".into()),
        };

        let header = init.get("header").cloned();
        let body = init.get("body").cloned();

        Ok(RequestInit {
            method,
            header,
            body,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub status: u16,
    pub headers: Value,
    pub body: String,
}

#[tauri::command]
pub async fn fetch(url: &str, init: Value) -> Result<Response, String> {
    let request_init = match RequestInit::new(init) {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    let client = Client::new();

    let mut headers = HeaderMap::new();
    if let Some(header_value) = request_init.header {
        if let Some(obj) = header_value.as_object() {
            for (key, value) in obj {
                if let Some(value_str) = value.as_str() {
                    headers.insert(
                        match reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                            Ok(value) => value,
                            Err(err) => {
                                return Err(format!("InvalidHeaderName:{}", err.to_string()))
                            }
                        },
                        match reqwest::header::HeaderValue::from_str(value_str) {
                            Ok(value) => value,
                            Err(err) => {
                                return Err(format!("InvalidHeaderValue:{}", err.to_string()))
                            }
                        },
                    );
                }
            }
        }
    }

    let response = match request_init.method {
        Method::GET => match client.get(url).headers(headers).send().await {
            Ok(value) => value,
            Err(err) => return Err(err.to_string()),
        },
        Method::POST => {
            let mut request_builder = client.post(url).headers(headers);
            if let Some(body_value) = request_init.body {
                request_builder = request_builder.body(body_value.to_string());
            }
            match request_builder.send().await {
                Ok(value) => value,
                Err(err) => return Err(err.to_string()),
            }
        }
    };

    let status = response.status().as_u16();

    let headers_json: Value = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                Value::String(value.to_str().unwrap_or("").to_string()),
            )
        })
        .collect();

    let body = match response.text().await {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    Ok(Response {
        status,
        headers: headers_json,
        body,
    })
}
