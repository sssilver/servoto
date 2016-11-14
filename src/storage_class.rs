use bson;
use error::WaldoError;
use std::str::FromStr;


#[derive(Debug)]
pub enum StorageClass {
    Standard
}


impl FromStr for StorageClass {
    type Err = WaldoError;

    fn from_str(s: &str) -> Result<StorageClass, WaldoError> {
        match s {
            "STANDARD" => Ok(StorageClass::Standard),
            _ => Err(WaldoError::ParseError)
        }
    }
}

impl<'a> From<&'a StorageClass> for bson::Bson {
    fn from(storage_class: &'a StorageClass) -> bson::Bson {
        match *storage_class {
            StorageClass::Standard => bson::Bson::String(String::from("STANDARD"))
        }
    }
}
