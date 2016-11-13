use std::error;
use std::fmt;
use redis::RedisError;


#[derive(Debug)]
pub enum WaldoError {
    StorageError(RedisError),
    PhotoNotFound(&'static str)
}


impl fmt::Display for WaldoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WaldoError::StorageError(ref err) => write!(f, "Storage error: {}", err),
            WaldoError::PhotoNotFound(ref err) => write!(f, "Photo not found: {}", err)
        }
    }
}


impl error::Error for WaldoError {
    fn description(&self) -> &str {
        match *self {
            WaldoError::StorageError(ref err) => err.description(),
            WaldoError::PhotoNotFound(ref err) => err
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WaldoError::StorageError(ref err) => Some(err),
            WaldoError::PhotoNotFound(_) => None
        }
    }
}


impl From<RedisError> for WaldoError {
    fn from(err: RedisError) -> WaldoError {
        WaldoError::StorageError(err)
    }
}

