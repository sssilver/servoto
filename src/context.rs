use error::WaldoError;
use process::Processor;
use photo::Photo;
use std::sync::Arc;
use storage::Storage;


pub struct Context {
    pub database: Arc<Storage>,
    pub processor: Processor
}


impl Context {
    pub fn update_catalog(&mut self) -> Result<(), WaldoError> {
        self.processor.process("http://s3.amazonaws.com/waldo-recruiting".to_string())
    }

    pub fn get_photo(&self, uuid: &str) -> Result<Photo, WaldoError> {
        self.database.fetch(uuid)
    }
}
