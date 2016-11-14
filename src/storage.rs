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
    }

    pub fn store_many(&self, photos: Vec<Photo>) -> Result<(), WaldoError> {
        for photo in photos {
            try!(self.store_one(photo));  // TODO: Use collection::insert_many() instead!
        }

        Ok(())
    }

    pub fn fetch<'a, 'b>(&'a self, key: &'b str) -> Result<Photo, WaldoError> {
        let photo_document = match try!(self.collection.find_one(Some(doc! { "_id" => key }), None)) {
            Some(photo_document) => photo_document,
            None => return Err(WaldoError::PhotoNotFound(String::from(key)))
        };

        Photo::from_mongo_document(photo_document)
    }
}