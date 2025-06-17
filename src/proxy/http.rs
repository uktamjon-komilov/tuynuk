use std::{collections::HashMap, fmt::Display, str::FromStr};

use clap::Error;
use tokio::net::TcpStream;

use super::error::HttpError;

#[derive(Debug)]
pub enum HttpVersion {
    Http09,
    Http10,
    Http11,
    Http2,
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version_str = match self {
            HttpVersion::Http09 => "HTTP/0.9",
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        };

        write!(f, "{}", version_str)
    }
}

impl FromStr for HttpVersion {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/0.9" => Ok(HttpVersion::Http09),
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(HttpError::InvalidVersion(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl FromStr for HttpMethod {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(HttpMethod::GET),
            "post" => Ok(HttpMethod::POST),
            "put" => Ok(HttpMethod::PUT),
            "patch" => Ok(HttpMethod::PATCH),
            "delete" => Ok(HttpMethod::DELETE),
            "head" => Ok(HttpMethod::HEAD),
            "options" => Ok(HttpMethod::OPTIONS),
            "trace" => Ok(HttpMethod::TRACE),
            "connect" => Ok(HttpMethod::CONNECT),
            _ => Err(HttpError::InvalidMethod(s.to_string())),
        }
    }
}

impl Into<&'static str> for HttpMethod {
    fn into(self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query: Option<String>,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

const HOP_BY_HOP_HEADERS: [&str; 8] = [
    "Connection",
    "Keep-Alive",
    "Proxy-Authenticate",
    "Proxy-Authorization",
    "TE",
    "Trailer",
    "Transfer-Encoding",
    "Upgrade",
];

impl HttpRequest {
    pub fn parse(buffer: &[u8]) -> Result<Self, HttpError> {
        let request_string = String::from_utf8_lossy(buffer);

        println!("{}", &request_string);

        let mut lines = request_string.split("\r\n");

        let request_line = lines.next().ok_or(HttpError::MissingRequestLine)?;
        let (method, path, query, version) = Self::parse_request_line(request_line)?;

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body_start = false;
        let mut body_lines = Vec::new();

        for line in lines {
            if body_start {
                body_lines.push(line);
            } else if line.is_empty() {
                body_start = true;
            } else {
                let (key, value) = Self::parse_header_line(line)?;
                headers.insert(key, value);
            }
        }

        let body = body_lines.join("\r\n").into_bytes();

        Ok(HttpRequest {
            method: method,
            path: path.to_string(),
            query: query,
            version: version,
            headers: headers,
            body: body,
        })
    }

    pub fn parse_request_line(
        line: &str,
    ) -> Result<(HttpMethod, String, Option<String>, HttpVersion), HttpError> {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        if parts.len() < 3 {
            return Err(HttpError::InvalidRequestLine(line.to_string()));
        }

        let method = parts[0].parse()?;
        let version = parts[2].parse()?;
        let fullpath = parts[1];

        let (path, query) = if let Some(query_start) = fullpath.find("?") {
            (
                fullpath[..query_start].to_string(),
                Some(fullpath[query_start + 1..].to_string()),
            )
        } else {
            (fullpath.to_string(), None)
        };

        return Ok((method, path, query, version));
    }

    pub fn parse_header_line(line: &str) -> Result<(String, String), HttpError> {
        let (key, value) = line
            .split_once(":")
            .ok_or_else(|| HttpError::InvalidHeader(line.to_string()))?;

        Ok((key.trim().to_string(), value.trim().to_string()))
    }

    pub fn to_backend_request(self) -> Vec<u8> {
        let mut request_string = String::new();

        let method: &str = self.method.into();
        let fullpath: String = match self.query {
            Some(query_string) => format!("{}?{}", self.path, query_string),
            None => self.path,
        };
        let request_line = format!("{} {} {}", method, fullpath, self.version);

        let new_headers: HashMap<_, _> = self
            .headers
            .into_iter()
            .filter(|(key, _)| !HOP_BY_HOP_HEADERS.contains(&key.as_ref()))
            .collect();

        let mut header_lines = String::new();

        new_headers.iter().for_each(|(key, value)| {
            let header_line = format!("{}: {}\r\n", key, value);
            header_lines.push_str(header_line.as_str());
        });

        request_string.push_str(&request_line);
        request_string.push_str(header_lines.as_str());
        request_string.push_str("\r\n");

        let mut result = request_string.into_bytes();

        result.extend(self.body);

        return result;
    }

    // Establishes TCP connection to backend server (127.0.0.1:3000), sends the HTTP request bytes,
    // waits for the complete response, then returns all response bytes (headers + body).
    // Returns IO error if connection fails, send fails, or response reading fails.
    async fn forward_to_backend(request_bytes: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
        Ok(vec![])
    }

    // Reads from client TCP stream until we have a complete HTTP request.
    // Must handle partial reads and continue until we find "\r\n\r\n" (end of headers),
    // then read any body content based on Content-Length header if present.
    // Returns the complete raw HTTP request as bytes (request line + headers + body).
    async fn read_http_request(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
        Ok(vec![])
    }

    // Reads from backend TCP stream until we have a complete HTTP response.
    // Similar to read_http_request but for responses: reads status line + headers until "\r\n\r\n",
    // then reads body content based on Content-Length or Transfer-Encoding headers.
    // Returns the complete raw HTTP response as bytes (status + headers + body).
    async fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
        Ok(vec![])
    }

    // Writes the complete response bytes to the client TCP stream.
    // Ensures all bytes are sent before returning. Used to send backend response
    // back to original client, or to send error responses (like 500) when backend fails.
    async fn send_response(
        stream: &mut TcpStream,
        response_bytes: Vec<u8>,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
