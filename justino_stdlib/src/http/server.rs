//! Async High-Performance Native HTTP Server in Pure Rust.

use crate::error::StdlibError;
use crate::http::request_response::{HttpRequest, HttpResponse};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub type RequestHandler = Arc<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>;

pub struct HttpServer {
    pub port: u16,
    pub running: Arc<AtomicBool>,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn listen<F>(&self, handler: F) -> Result<(), StdlibError>
    where
        F: Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .map_err(|e| StdlibError::HttpError(format!("Failed to bind to {}: {}", addr, e)))?;

        self.running.store(true, Ordering::SeqCst);
        let handler_arc: RequestHandler = Arc::new(handler);
        let running_clone = self.running.clone();

        thread::spawn(move || {
            for stream_res in listener.incoming() {
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
                if let Ok(mut stream) = stream_res {
                    let handler_ref = handler_arc.clone();
                    thread::spawn(move || {
                        let _ = handle_connection(&mut stream, handler_ref);
                    });
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

fn handle_connection(stream: &mut TcpStream, handler: RequestHandler) -> Result<(), StdlibError> {
    let mut buffer = [0u8; 4096];
    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|e| StdlibError::HttpError(format!("Failed to read stream: {}", e)))?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request_raw = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request_raw.lines();

    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    let method = parts.first().copied().unwrap_or("GET");
    let path = parts.get(1).copied().unwrap_or("/");

    let request = HttpRequest::new(method, path);
    let response = handler(request);

    let response_bytes = response.to_http_string();
    stream
        .write_all(response_bytes.as_bytes())
        .map_err(|e| StdlibError::HttpError(format!("Failed to write response: {}", e)))?;

    stream.flush().map_err(|e| StdlibError::HttpError(format!("Failed to flush: {}", e)))?;
    Ok(())
}
