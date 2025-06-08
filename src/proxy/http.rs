use std::{collections::HashMap, str::FromStr};

use super::error::HttpError;

#[derive(Debug)]
enum HttpVersion {
    Http09,
    Http10,
    Http11,
    Http2,
    Http3,
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

#[derive(Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    query: Option<String>,
    version: HttpVersion,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse(buffer: &[u8]) -> Result<Self, HttpError> {
        let request_string = String::from_utf8_lossy(buffer);

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
}
