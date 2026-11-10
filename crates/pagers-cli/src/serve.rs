//! `pagers serve`: build the site, then serve the output directory over
//! HTTP. A small std-only static file server — no framework needed.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

const CONTENT_TYPES: &[(&str, &str)] = &[
    ("html", "text/html; charset=utf-8"),
    ("css", "text/css; charset=utf-8"),
    ("js", "text/javascript; charset=utf-8"),
    ("json", "application/json; charset=utf-8"),
    ("xml", "application/xml; charset=utf-8"),
    ("svg", "image/svg+xml"),
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("ico", "image/x-icon"),
    ("md", "text/markdown; charset=utf-8"),
    ("txt", "text/plain; charset=utf-8"),
    ("woff2", "font/woff2"),
    ("wasm", "application/wasm"),
];

/// Serves `dir` on `host:port` until interrupted — the plain static file
/// server behind `pagers landing preview`.
pub fn serve_dir(dir: &str, host: &str, port: u16) -> Result<(), String> {
    let root = PathBuf::from(dir);
    if !root.is_dir() {
        return Err(format!("directory {dir} does not exist"));
    }
    let listener =
        TcpListener::bind((host, port)).map_err(|e| format!("bind {host}:{port}: {e}"))?;
    eprintln!("Serving {} at http://{host}:{port}/", root.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = root.clone();
                std::thread::spawn(move || {
                    let _ = handle(stream, &root, "");
                });
            }
            Err(e) => eprintln!("accept: {e}"),
        }
    }
    Ok(())
}

/// Serves `dir` on `127.0.0.1:port` until interrupted. `base_path` is the
/// site's configured base path (e.g. `/docs`): request paths are stripped of
/// it so prefixed links resolve to the unprefixed files on disk.
pub fn serve(dir: &str, port: u16, base_path: &str) -> Result<(), String> {
    let root = PathBuf::from(dir);
    if !root.is_dir() {
        return Err(format!("output directory {dir} does not exist"));
    }
    let prefix = base_path.trim_matches('/').to_string();
    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("bind 127.0.0.1:{port}: {e}"))?;
    eprintln!(
        "Serving {} at http://127.0.0.1:{port}{base_path}/",
        root.display()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = root.clone();
                let prefix = prefix.clone();
                std::thread::spawn(move || {
                    let _ = handle(stream, &root, &prefix);
                });
            }
            Err(e) => eprintln!("accept: {e}"),
        }
    }
    Ok(())
}

fn handle(mut stream: TcpStream, root: &Path, prefix: &str) -> std::io::Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    // Reject traversal outside the root, then strip the site base path.
    let mut rel = path.split(['?', '#']).next().unwrap_or("/");
    rel = rel.trim_start_matches('/');
    if rel.contains("..") {
        return respond(&mut stream, 403, "text/plain", b"Forbidden");
    }
    if !prefix.is_empty() {
        if rel == prefix {
            rel = "";
        } else if rel.len() > prefix.len()
            && rel.starts_with(prefix)
            && rel.as_bytes()[prefix.len()] == b'/'
        {
            rel = &rel[prefix.len() + 1..];
        }
    }
    let rel = rel.trim_start_matches('/');

    let mut file = root.join(rel);
    if file.is_dir() {
        file.push("index.html");
    }
    if rel.is_empty() {
        file = root.join("index.html");
    } else if !file.exists() && !rel.ends_with('/') {
        // Pages publish at trailing-slash URLs: redirect /guide to /guide/.
        let dir_index = root.join(rel).join("index.html");
        if dir_index.exists() {
            let location = format!("/{rel}/");
            let head = format!(
                "HTTP/1.1 301 Moved Permanently\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            return stream.write_all(head.as_bytes());
        }
    }

    match std::fs::read(&file) {
        Ok(bytes) => {
            let mime = content_type(&file);
            respond(&mut stream, 200, mime, &bytes)
        }
        Err(_) => respond(&mut stream, 404, "text/plain", b"Not found"),
    }
}

fn content_type(path: &Path) -> &'static str {
    path.extension()
        .and_then(|e| e.to_str())
        .and_then(|e| {
            CONTENT_TYPES
                .iter()
                .find(|(ext, _)| ext.eq_ignore_ascii_case(e))
                .map(|(_, mime)| *mime)
        })
        .unwrap_or("application/octet-stream")
}

fn respond(stream: &mut TcpStream, status: u16, mime: &str, body: &[u8]) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        403 => "Forbidden",
        _ => "Not Found",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)
}
