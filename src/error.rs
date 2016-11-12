use std::error;
use std::fmt;
use redis::RedisError;


#[derive(Debug)]
pub enum WaldoError {
    StorageError(RedisError)
}


impl fmt::Display for WaldoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WaldoError::StorageError(ref err) => write!(f, "Storage error: {}", err),
        }
    }
}


impl error::Error for WaldoError {
    fn description(&self) -> &str {
        match *self {
            WaldoError::StorageError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WaldoError::StorageError(ref err) => Some(err),
        }
    }
}


impl From<RedisError> for WaldoError {
    fn from(err: RedisError) -> WaldoError {
        WaldoError::StorageError(err)
    }
}

