use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream);
        });


    }
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next();
    let path = parts.next();
    let version = parts.next();

    if method != Some("GET") {
        return;
    }

    let path = match path {
        Some(p) => p,
        None => return,
    };

    let is_http_1_1 = version == Some("HTTP/1.1");

    let file_path = if path == "/" && is_http_1_1 {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    println!("Requested: {}", file_path);

    let resolved_path = resolve_path(file_path);

    let (status_line, contents) = match resolved_path {
        Some(valid_path) => {
            let data = fs::read_to_string(valid_path).unwrap_or_default();
            ("HTTP/1.1 200 OK", data)
        }
        None => {
            let not_found = format!(
                "<h1>404 Not Found</h1><p>{} doesn't exist!</p>",
                file_path
            );

            let fallback = fs::read_to_string("pub/404.html")
                .unwrap_or(not_found);

            ("HTTP/1.1 404 NOT FOUND", fallback)
        }
    };

    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    thread::sleep(Duration::from_secs(5));
    let _ = stream.write_all(response.as_bytes());
}
// resolving paht
fn resolve_path(path: &str) -> Option<String> {
    let base = Path::new("pub").canonicalize().ok()?;

    let candidate = base.join(path);

    let resolved = candidate.canonicalize().ok();

    if let Some(ref p) = resolved {
        if p.starts_with(&base) && p.is_file() {
            return Some(p.to_string_lossy().to_string());
        }
    }

    if Path::new(path).extension().is_none() {
        let html_candidate = base.join(format!("{path}.html"));
        if let Ok(p) = html_candidate.canonicalize() {
            if p.starts_with(&base) && p.is_file() {

                return Some(p.to_string_lossy().to_string());
            }
        }
    }

    let index_candidate = base.join(path).join("index.html");
    if let Ok(p) = index_candidate.canonicalize() {
        if p.starts_with(&base) && p.is_file() {
            return Some(p.to_string_lossy().to_string());
        }
    }

    None
}