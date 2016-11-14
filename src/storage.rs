use error::WaldoError;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use photo::Photo;



pub struct Storage {
    collection: Collection
}


impl Storage {
    pub fn new(host: &str, port: u16) -> Result<Storage, WaldoError> {
        let client = try!(Client::connect(host, port));
        let collection = client.db("waldo").collection("photos");

        Ok(Storage {
            collection: collection
        })
    }

    pub fn store_one(&self, photo: Photo) -> Result<(), WaldoError> {
        try!(self.collection.insert_one(photo.to_mongo_document(), None));

        Ok(())
        //self.conn.set(photo.key, photo);
    }

    pub fn store_many(&self, photos: Vec<Photo>) -> Result<(), WaldoError> {
        for photo in photos {
            try!(self.store_one(photo));
        }

        Ok(())
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