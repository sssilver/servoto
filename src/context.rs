use curl::easy::Easy;
use error::WaldoError;
use photo::Photo;
use storage::Storage;


pub struct Context {
    pub database: Storage
}


impl Context {
    pub fn update_catalog(&self) -> Result<(), WaldoError> {
        // Fetch the XML from S3
        let mut http_client = Easy::new();
        http_client.url("http://s3.amazonaws.com/waldo-recruiting").unwrap();

        http_client.write_function(|data| {
            println!("{:?}", data);
            Ok(data.len())
        }).unwrap();
        http_client.perform().unwrap();

        println!("{}", http_client.response_code().unwrap());

        Ok(())
    }

    pub fn get_photo(&self, uuid: &str) -> Result<Photo, WaldoError> {
        self.database.fetch(uuid)
    }
}