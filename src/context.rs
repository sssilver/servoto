use curl::easy::{Easy, WriteError};
use error::WaldoError;
use photo::Photo;
use storage::Storage;

use std::fs::File;
use std::io::BufReader;
use std::str::from_utf8;


pub struct Context {
    pub database: Storage
}


impl Context {
    pub fn update_catalog(&self) -> Result<(), WaldoError> {
        // Fetch the XML from S3
        let mut http_client = Easy::new();
        try!(http_client.url("http://s3.amazonaws.com/waldo-recruiting"));

        let mut response = Vec::new();

        {
            let mut transfer = http_client.transfer();
            transfer.write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        let photos = Photo::new_many(&response);

        println!("{:?}", photos);
        println!("Total: {} photos", photos.unwrap().len());

        Ok(())
    }

    pub fn get_photo(&self, uuid: &str) -> Result<Photo, WaldoError> {
        self.database.fetch(uuid)
    }
}