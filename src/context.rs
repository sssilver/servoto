use photo::Photo;
use storage::Storage;


pub struct Context {
    pub database: Storage
}


impl Context {
    pub fn get(&self, uuid: &str) -> Photo {
        self.database.fetch(uuid)
    }
}