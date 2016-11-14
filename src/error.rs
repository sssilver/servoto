use curl;
use redis::RedisError;
use std::error;
use std::fmt;
use std::num;
use xmltree;


#[derive(Debug)]
pub enum WaldoError {
    StorageError(RedisError),
    PhotoNotFound(&'static str),
    NetworkError,
    MalformedError(xmltree::ParseError),
    ParseError
}


impl fmt::Display for WaldoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WaldoError::StorageError(ref err) => write!(f, "Storage error: {}", err),
            WaldoError::PhotoNotFound(ref err) => write!(f, "Photo not found: {}", err),
            WaldoError::NetworkError => write!(f, "Network error"),
            WaldoError::MalformedError(ref err) => write!(f, "XML parsing error: {}", err),
            WaldoError::ParseError => write!(f, "Parse error"),
        }
    }
}


impl error::Error for WaldoError {
    fn description(&self) -> &str {
        match *self {
            WaldoError::StorageError(ref err) => err.description(),
            WaldoError::PhotoNotFound(ref err) => err,
            WaldoError::NetworkError => "Network error",
            WaldoError::MalformedError(ref err) => err.description(),
            WaldoError::ParseError => "Parse error"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WaldoError::StorageError(ref err) => Some(err),
            WaldoError::PhotoNotFound(_) => None,
            WaldoError::NetworkError => None,
            WaldoError::MalformedError(ref err) => Some(err),
            WaldoError::ParseError => None,
        }
    }
}


impl From<RedisError> for WaldoError {
    fn from(err: RedisError) -> WaldoError {
        WaldoError::StorageError(err)
    }
}


impl From<curl::Error> for WaldoError {
    fn from(err: curl::Error) -> WaldoError {
        WaldoError::NetworkError
    }
}


impl From<xmltree::ParseError> for WaldoError {
    fn from(err: xmltree::ParseError) -> WaldoError {
        WaldoError::MalformedError(err)
    }
}


impl From<num::ParseIntError> for WaldoError {
    fn from(err: num::ParseIntError) -> WaldoError {
        WaldoError::ParseError
    }
}