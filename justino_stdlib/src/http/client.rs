//! Native HTTP Client in Pure Rust.

use crate::error::StdlibError;
use crate::http::request_response::HttpResponse;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct HttpClient;

impl HttpClient {
    pub fn get(url: &str) -> Result<HttpResponse, StdlibError> {
        Self::request("GET", url, "", HashMap::new())
    }

    pub fn post(url: &str, body: &str) -> Result<HttpResponse, StdlibError> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self::request("POST", url, body, headers)
    }

    pub fn request(
        method: &str,
        url: &str,
        body: &str,
        headers: HashMap<String, String>,
    ) -> Result<HttpResponse, StdlibError> {
        let (host, path, port) = parse_url(url)?;
        let addr = format!("{}:{}", host, port);

        let mut stream = TcpStream::connect(&addr)
            .map_err(|e| StdlibError::HttpError(format!("Failed to connect to {}: {}", addr, e)))?;

        let mut req_str = format!("{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n", method, path, host);
        for (k, v) in &headers {
            req_str.push_str(&format!("{}: {}\r\n", k, v));
        }

        if !body.is_empty() {
            req_str.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
            req_str.push_str(body);
        } else {
            req_str.push_str("\r\n");
        }

        stream
            .write_all(req_str.as_bytes())
            .map_err(|e| StdlibError::HttpError(format!("Failed to send HTTP request: {}", e)))?;

        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .map_err(|e| StdlibError::HttpError(format!("Failed to read HTTP response: {}", e)))?;

        let response_str = String::from_utf8_lossy(&buffer);
        parse_response(&response_str)
    }
}

fn parse_url(url: &str) -> Result<(String, String, u16), StdlibError> {
    let clean = url.trim_start_matches("http://").trim_start_matches("https://");
    let parts: Vec<&str> = clean.splitn(2, '/').collect();

    let host_port = parts[0];
    let path = if parts.len() > 1 {
        format!("/{}", parts[1])
    } else {
        "/".to_string()
    };

    let hp_parts: Vec<&str> = host_port.splitn(2, ':').collect();
    let host = hp_parts[0].to_string();
    let port = if hp_parts.len() > 1 {
        hp_parts[1].parse::<u16>().unwrap_or(80)
    } else {
        80
    };

    Ok((host, path, port))
}

fn parse_response(raw: &str) -> Result<HttpResponse, StdlibError> {
    let parts: Vec<&str> = raw.splitn(2, "\r\n\r\n").collect();
    let header_part = parts[0];
    let body_part = parts.get(1).copied().unwrap_or("");

    let mut lines = header_part.lines();
    let status_line = lines.next().unwrap_or("");
    let status_parts: Vec<&str> = status_line.split_whitespace().collect();

    let status_code = status_parts.get(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(200);
    let status_text = status_parts.get(2..).map(|p| p.join(" ")).unwrap_or_else(|| "OK".to_string());

    let mut headers = HashMap::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    Ok(HttpResponse {
        status_code,
        status_text,
        headers,
        body: body_part.to_string(),
    })
}
