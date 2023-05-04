/// Various errors the the server might throw
#[derive(Debug)]
pub enum ServerErrorType {
    BadDirectory,
    ReadFail,
    BadPort,
//    ConnFail,
}
// Mapping for the error type descriptions:
const BAD_DIRECTORY: &str = "Could not open directory";
const READ_FAIL: &str = "Could not open file";
const BAD_PORT: &str = "Could not open port";
//const CONN_FAIL: &str = "The connection failed";

/// A ServerError, including both a type
/// (to check which error) as well as a message
/// (to give a user-friendly issue as a `&str`)
#[derive(Debug)]
pub struct ServerError {
    pub error_type: ServerErrorType,
    pub error_msg: String,
}

impl ServerError {
    /// Creates a new ServerError of type [error_type](ServerError::error_type),
    /// automatically setting the [error_msg](ServerError::error_msg)
    pub fn new(error_type: ServerErrorType) -> ServerError {
        ServerError {
            error_msg: match error_type {
                ServerErrorType::BadDirectory => BAD_DIRECTORY.to_string(),
                ServerErrorType::ReadFail => READ_FAIL.to_string(),
                ServerErrorType::BadPort => BAD_PORT.to_string(),
//                ServerErrorType::ConnFail => CONN_FAIL.to_string(),
            },
            error_type,
        }
    }
}

impl std::fmt::Display for ServerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ServerErrorType::{:?}", self)
    }
}
/// Override to make the display more user-friendly
impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Error ({}): {}", self.error_type, self.error_msg)
    }
}

/// Returns a readable `&str` based on the HTTP code type.
/// To be used when writing a response in an HTTP message
/// 
/// Response statuses as listed in https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
/// 
/// Note: Not all codes are listed in this map,
/// specifically some 4XX and 5XX errors
/// which just return a general client/server error message.
pub fn http_response_from_code(code: i32) -> String {
    String::from(match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        103 => "Early Hints",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        400..=499 => "Client Error",
        500 => "Internal Server Error",
        // ...
        500..=599 => "Server Error",
        _ => "IDK" // panic?
    })
}