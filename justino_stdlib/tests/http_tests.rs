use justino_stdlib::error::StdlibError;
use justino_stdlib::http::{HttpClient, HttpRequest, HttpResponse, HttpServer};
use std::thread;
use std::time::Duration;

#[test]
fn test_http_server_and_client_e2e() -> Result<(), StdlibError> {
    let port = 9876;
    let server = HttpServer::new(port);

    server.listen(|req: HttpRequest| {
        if req.path == "/api/status" {
            HttpResponse::json("{\"status\":\"online\"}")
        } else {
            HttpResponse::ok("Welcome to Justino HTTP Server")
        }
    })?;

    thread::sleep(Duration::from_millis(100));

    let res = HttpClient::get(&format!("http://127.0.0.1:{}/api/status", port))?;
    assert_eq!(res.status_code, 200);
    assert!(res.body.contains("online"));

    server.stop();
    Ok(())
}
