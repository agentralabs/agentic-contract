//! Hardened stdio transport — Content-Length framed JSON-RPC 2.0 (MCP standard).

use std::io::{BufRead, BufReader, Read, Write};

/// Maximum frame size: 8 MiB.
pub const MAX_CONTENT_LENGTH_BYTES: usize = 8 * 1024 * 1024;
/// JSON-RPC stdio framing header marker.
pub const CONTENT_LENGTH_HEADER: &str = "content-length:";
/// Backward-compatible alias for existing transport checks.
pub const MAX_MESSAGE_BYTES: usize = MAX_CONTENT_LENGTH_BYTES;

/// Transport errors.
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    /// Message exceeds maximum size.
    #[error("Message too large: {0} bytes (max {1})")]
    MessageTooLarge(usize, usize),

    /// Invalid UTF-8 in message.
    #[error("Invalid UTF-8 in message")]
    InvalidUtf8,

    /// Invalid JSON-RPC version.
    #[error("Invalid JSON-RPC version: expected \"2.0\", got {0:?}")]
    InvalidJsonRpcVersion(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parse error.
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Hardened stdio transport using Content-Length framing.
pub struct StdioTransport<R: Read, W: Write> {
    reader: BufReader<R>,
    writer: W,
}

impl<R: Read, W: Write> StdioTransport<R, W> {
    /// Create a new transport.
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer,
        }
    }

    /// Read a JSON message — supports both Content-Length framing and plain JSON lines.
    pub fn read_message(&mut self) -> Result<String, TransportError> {
        let mut content_length: Option<usize> = None;

        loop {
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(TransportError::Io(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "EOF while reading",
                )));
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                if let Some(len) = content_length {
                    // Content-Length mode: empty line = end of headers, read body
                    if len > MAX_MESSAGE_BYTES {
                        return Err(TransportError::MessageTooLarge(len, MAX_MESSAGE_BYTES));
                    }
                    let mut body = vec![0u8; len];
                    self.reader.read_exact(&mut body)?;
                    return String::from_utf8(body).map_err(|_| TransportError::InvalidUtf8);
                }
                continue; // Skip empty lines before any content
            }

            // Check for Content-Length header (case-insensitive)
            if let Some((name, value)) = trimmed.split_once(':') {
                let header_name = CONTENT_LENGTH_HEADER.trim_end_matches(':');
                if name.trim().eq_ignore_ascii_case(header_name) {
                    if let Ok(parsed) = value.trim().parse::<usize>() {
                        content_length = Some(parsed);
                        continue;
                    }
                }
                // If we're inside a Content-Length header block, skip other headers
                if content_length.is_some() {
                    continue;
                }
            }

            // Plain JSON line mode — only accept lines that look like JSON objects
            if trimmed.starts_with('{') {
                if trimmed.len() > MAX_MESSAGE_BYTES {
                    return Err(TransportError::MessageTooLarge(trimmed.len(), MAX_MESSAGE_BYTES));
                }
                return Ok(trimmed.to_string());
            }

            // Skip non-JSON, non-header lines (garbage tolerance)
        }
    }

    /// Write a JSON message as a newline-delimited line (raw JSON-RPC).
    /// This is compatible with all MCP clients including Hydra's stdio spawner.
    pub fn write_message(&mut self, content: &str) -> Result<(), TransportError> {
        self.writer.write_all(content.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }
}

/// Validate that a JSON value is a valid JSON-RPC 2.0 request.
pub fn validate_jsonrpc(request: &serde_json::Value) -> Result<(), TransportError> {
    match request.get("jsonrpc").and_then(|v| v.as_str()) {
        Some("2.0") => Ok(()),
        Some(other) => Err(TransportError::InvalidJsonRpcVersion(other.to_string())),
        None => Err(TransportError::InvalidJsonRpcVersion("missing".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_content_length_framed() {
        let input = b"Content-Length: 13\r\n\r\n{\"test\":true}";
        let mut output = Vec::new();

        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"test\":true}");

        transport.write_message("hello").unwrap();
        let written = String::from_utf8(output).unwrap();
        assert_eq!(written, "hello\n");
    }

    #[test]
    fn test_read_plain_json_line() {
        let input = b"{\"jsonrpc\":\"2.0\",\"method\":\"initialize\"}\n";
        let mut output = Vec::new();

        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"jsonrpc\":\"2.0\",\"method\":\"initialize\"}");
    }

    #[test]
    fn test_case_insensitive_content_length() {
        let input = b"content-length: 4\r\n\r\ntest";
        let mut output = Vec::new();
        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "test");
    }

    #[test]
    fn test_non_json_non_header_lines_skipped() {
        // Non-Content-Length header lines that don't start with '{' are skipped.
        // The reader should find the JSON object after the garbage.
        let input = b"No-Header: value\r\n{\"ok\":true}\n";
        let mut output = Vec::new();
        let mut transport = StdioTransport::new(std::io::Cursor::new(input.to_vec()), &mut output);
        let msg = transport.read_message().unwrap();
        assert_eq!(msg, "{\"ok\":true}");
    }

    #[test]
    fn test_validate_jsonrpc() {
        let valid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"2.0","method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&valid).is_ok());

        let invalid: serde_json::Value =
            serde_json::from_str(r#"{"jsonrpc":"1.0","method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&invalid).is_err());

        let missing: serde_json::Value =
            serde_json::from_str(r#"{"method":"test"}"#).unwrap_or_default();
        assert!(validate_jsonrpc(&missing).is_err());
    }
}
