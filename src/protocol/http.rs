use super::parser::{HttpData, PacketParser};
use std::str;

#[derive(Debug)]
pub struct HttpParser;

impl HttpParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_request(&self, data: &[u8]) -> Option<HttpData> {
        let text = str::from_utf8(data).ok()?;
        let first_line = text.lines().next()?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() < 3 {
            return None;
        }

        // Check if this looks like a request (method path version)
        // Valid HTTP methods: GET, POST, PUT, DELETE, HEAD, OPTIONS, PATCH, etc.
        let method = parts[0];
        if !Self::is_valid_method(method) {
            return None;
        }

        let method = Some(method.to_string());
        let path = Some(parts[1].to_string());

        // Parse headers
        let mut host = None;
        let mut content_type = None;

        for line in text.lines().skip(1) {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                let key_lower = key.trim().to_lowercase();
                let value = value.trim().to_string();
                match key_lower.as_str() {
                    "host" => host = Some(value),
                    "content-type" => content_type = Some(value),
                    _ => {}
                }
            }
        }

        Some(HttpData {
            method,
            path,
            status: None,
            host,
            content_type,
        })
    }

    fn is_valid_method(method: &str) -> bool {
        matches!(method.to_uppercase().as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH" | "TRACE" | "CONNECT"
        )
    }

    fn parse_response(&self, data: &[u8]) -> Option<HttpData> {
        let text = str::from_utf8(data).ok()?;
        let first_line = text.lines().next()?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() < 2 {
            return None;
        }

        let status: u16 = parts[1].parse().ok()?;

        // Parse headers
        let mut content_type = None;
        let mut host = None;

        for line in text.lines().skip(1) {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                let key_lower = key.trim().to_lowercase();
                let value = value.trim().to_string();
                match key_lower.as_str() {
                    "content-type" => content_type = Some(value),
                    "host" => host = Some(value),
                    _ => {}
                }
            }
        }

        Some(HttpData {
            method: None,
            path: None,
            status: Some(status),
            host,
            content_type,
        })
    }
}

impl PacketParser for HttpParser {
    type Output = HttpData;
    type Error = HttpError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        // Try to parse as request first
        if let Some(http_data) = self.parse_request(data) {
            return Ok(http_data);
        }

        // Try to parse as response
        if let Some(http_data) = self.parse_response(data) {
            return Ok(http_data);
        }

        Err(HttpError::InvalidFormat)
    }

    fn protocol_name(&self) -> &'static str {
        "HTTP"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Invalid HTTP format")]
    InvalidFormat,
    #[error("UTF-8 decoding error")]
    Utf8Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_parser() {
        let parser = HttpParser::new();
        let data = b"GET /api/users HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\n\r\n";

        let result = parser.parse(data).unwrap();
        assert_eq!(result.method, Some("GET".to_string()));
        assert_eq!(result.path, Some("/api/users".to_string()));
        assert_eq!(result.host, Some("example.com".to_string()));
    }

    #[test]
    fn test_http_response_parser() {
        let parser = HttpParser::new();
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";

        let result = parser.parse(data).unwrap();
        assert_eq!(result.status, Some(200));
        assert_eq!(result.content_type, Some("text/html".to_string()));
    }
}
