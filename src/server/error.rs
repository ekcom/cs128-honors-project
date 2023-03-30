#[derive(Debug)]
pub enum ServerErrorType {
    BadDirectory,
    ReadFail,
}
const BAD_DIRECTORY: &str = "Could not open directory.";
const READ_FAIL: &str = "Could not open file.";

#[derive(Debug)]
pub struct ServerError {
    pub error_type: ServerErrorType,
    pub error_msg: String,
}

impl ServerError {
    pub fn new(error_type: ServerErrorType) -> ServerError {
        ServerError {
            error_msg: match error_type {
                ServerErrorType::BadDirectory => BAD_DIRECTORY.to_string(),
                ServerErrorType::ReadFail => READ_FAIL.to_string(),
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
impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Error ({}): {}", self.error_type, self.error_msg)
    }
}