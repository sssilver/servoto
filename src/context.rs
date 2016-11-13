use curl::easy::Easy;
use error::WaldoError;
use photo::Photo;
use storage::Storage;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};


pub struct Context {
    pub database: Storage
}


impl Context {
    pub fn update_catalog(&self) -> Result<(), WaldoError> {
        // Fetch the XML from S3
        let mut http_client = Easy::new();
        try!(http_client.url("http://s3.amazonaws.com/waldo-recruiting"));

        try!(http_client.write_function(|data| {
            println!("{:?}", data);
            Ok(data.len())
        }));
        try!(http_client.perform());

        println!("{}", http_client.response_code().unwrap());

        Ok(())
    }

    pub fn get_photo(&self, uuid: &str) -> Result<Photo, WaldoError> {
        self.database.fetch(uuid)
    }
}