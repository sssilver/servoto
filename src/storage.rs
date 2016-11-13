use redis;

use photo::Photo;
use error::WaldoError;


pub struct Storage {
    conn: redis::Connection
}


impl Storage {
    pub fn new(connection_string: &str) -> Result<Storage, WaldoError> {
        let client = try!(redis::Client::open(connection_string));

        let conn = try!(client.get_connection());

        Ok(Storage {
            conn: conn
        })
    }

    pub fn store(photo: Photo) {

    }

    pub fn fetch(&self, key: &str) -> Result<Photo, WaldoError> {
        Ok(Photo {
            key: String::from(key),
            last_modified: String::from("last_modified"),  // TODO: Should be parsed as a DateTime
            etag: String::from("etag"),
            size: 1234,
            storage_class: String::from("storage_class")  // TODO: Should be typed as a StorageClass enum
        })
    }
}