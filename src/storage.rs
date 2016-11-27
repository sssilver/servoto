use error::WaldoError;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use photo::PhotoResource;



pub struct Storage {
    collection: Collection
}


impl Storage {
    pub fn new(host: &str, port: u16) -> Result<Storage, WaldoError> {
        let client = Client::connect(host, port)?;
        let collection = client.db("waldo_assignment_areg").collection("photos");

        Ok(Storage {
            collection: collection
        })
    }

    pub fn store_one(&self, photo: PhotoResource) -> Result<(), WaldoError> {
        self.collection.insert_one(photo.to_mongo_document(), None)?;

        Ok(())
    }

    pub fn store_many(&self, photos: Vec<PhotoResource>) -> Result<(), WaldoError> {
        for photo in photos {
            self.store_one(photo)?;  // TODO: Use collection::insert_many() instead!
        }

        Ok(())
    }

    pub fn fetch<'a, 'b>(&'a self, key: &'b str) -> Result<PhotoResource, WaldoError> {
        let photo_document = match self.collection.find_one(Some(doc! { "_id" => key }), None)? {
            Some(photo_document) => photo_document,
            None => return Err(WaldoError::PhotoNotFound(String::from(key)))
        };

        PhotoResource::from_mongo_document(photo_document)
    }
}