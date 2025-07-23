use std::fs::{self, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::thread;
use tiny_http::{Server, Response, Request, Header};

use crate::paths::platform_paths::LOG_PATH;

pub fn start_warning_server() {
    // Warning page server (port 80)
    thread::spawn(|| {
        let server = Server::http("127.0.0.1:80").expect("Failed to bind to port 80");
        println!("GambleGuard warning server running on http://127.0.0.1");

        for request in server.incoming_requests() {
            // Extract domain from Host header
            let domain = request.headers()
                .iter()
                .find(|h| h.field.equiv("Host"))
                .map(|h| h.value.as_str())
                .unwrap_or("Unknown")
                .to_string();

            let response = Response::from_string(
                r#"
                    <html>
                      <head><title>Access Blocked</title></head>
                      <body style="text-align:center;font-family:sans-serif;margin-top:10%;">
                        <h1>Gambling Site Blocked</h1>
                        <p>This site is blocked by GambleGuard to protect users from gambling harm.</p>
                      </body>
                    </html>
                "#,
            )
            .with_header(Header::from_bytes("Content-Type", "text/html").unwrap());

            let _ = request.respond(response);

            let log_msg = format!("Blocked gambling site: {}", domain);
            let _ = append_log(&log_msg);
        }
    });

    thread::spawn(|| {
        let server = Server::http("127.0.0.1:7878").expect("Failed to bind to port 7878");
        println!("GambleGuard dashboard API running on http://127.0.0.1:7878");

        for request in server.incoming_requests() {
            handle_dashboard_request(request);
        }
    });
}

/// Handles /logs endpoint 
fn handle_dashboard_request(request: Request) {
    match request.url() {
        "/logs" => {
            let logs = read_logs();
            let response = Response::from_string(logs)
                .with_header(Header::from_bytes("Content-Type", "text/plain").unwrap());
            let _ = request.respond(response);
        }
        _ => {
            let response = Response::from_string("404 Not Found")
                .with_status_code(404);
            let _ = request.respond(response);
        }
    }
}

/// Appends a message to the platform-specific log file
pub fn append_log(message: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "[{}] {}", timestamp, message)?;
    Ok(())
}

/// Reads the log file and returns it as a string
fn read_logs() -> String {
    match fs::File::open(LOG_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader.lines()
                .filter_map(|line| line.ok())
                .collect::<Vec<String>>()
                .join("\n")
        }
        Err(_) => String::from("No logs yet."),
    }
}
