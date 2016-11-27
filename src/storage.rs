use bson;
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
        let client = Client::connect(host, port)?;
        let collection = client.db("waldo_assignment_areg").collection("photos");

        Ok(Storage {
            collection: collection
        })
    }

    pub fn store_one(&self, photo: Photo) -> Result<(), WaldoError> {
        let serialized = match bson::to_bson(&photo) {
            Ok(serialized) => serialized,
            Err(err) => {
                println!("Error: {}", err);
                return Err(WaldoError::ParseError);
            }
        };

        if let bson::Bson::Document(doc) = serialized { self.collection.insert_one(doc, None)?; } else {
            return Err(WaldoError::ParseError);
        }

        Ok(())
    }

    pub fn fetch<'a, 'b>(&'a self, key: &'b str) -> Result<Photo, WaldoError> {
        let photo_document = match self.collection.find_one(Some(doc! { "_id" => key }), None)? {
            Some(photo_document) => photo_document,
            None => return Err(WaldoError::PhotoNotFound(String::from(key)))
        };

        match bson::from_bson(bson::Bson::Document(photo_document)) {
            Ok(bson) => Ok(bson),
            Err(_) => Err(WaldoError::ParseError)
        }
    }
}