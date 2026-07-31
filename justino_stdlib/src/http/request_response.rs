//! HTTP Request and Response Data Structures.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(method: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn ok(body: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
        Self {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body: body.into(),
        }
    }

    pub fn json(body: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body: body.into(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            status_text: "Not Found".to_string(),
            headers: HashMap::new(),
            body: "404 Not Found".to_string(),
        }
    }

    pub fn to_http_string(&self) -> String {
        let mut res = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        for (k, v) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", k, v));
        }
        res.push_str(&format!("Content-Length: {}\r\n\r\n", self.body.len()));
        res.push_str(&self.body);
        res
    }
}
