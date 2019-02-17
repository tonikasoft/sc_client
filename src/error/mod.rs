use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ScClientError {
    pub cause: String,
}

impl ScClientError {
    pub fn new(cause: &str) -> Self {
        ScClientError {
            cause: cause.to_string(),
        }
    }
}

impl fmt::Display for ScClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl error::Error for ScClientError {
    fn description(&self) -> &str {
        "sc_client error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<std::io::Error> for ScClientError {
    fn from(error: std::io::Error) -> Self {
        ScClientError::new(&format!("{}", error))
    }
}
