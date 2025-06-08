use std::fmt::Display;

#[derive(Debug)]
pub enum HttpError {
    InvalidVersion(String),
    InvalidMethod(String),
    MissingRequestLine,
    InvalidRequestLine(String),
    InvalidHeader(String),
    IoError(std::io::Error),
}


impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::InvalidVersion(version) => write!(f, "Invalid HTTP version: {}", version),
            HttpError::InvalidMethod(method) => write!(f, "Invalid HTTP method: {}", method),
            HttpError::InvalidRequestLine(line) => write!(f, "Invalid request line: {}", line),
            HttpError::InvalidHeader(header) => write!(f, "Invalid request header: {}", header),
            HttpError::MissingRequestLine => write!(f, "Missing request line"),
            HttpError::IoError(e) => write!(f, "IO error: {}", e)
        }
    }
}

impl std::error::Error for HttpError {
    
}

impl From<std::io::Error> for HttpError {
    fn from(value: std::io::Error) -> Self {
        HttpError::IoError(value)
    }
}