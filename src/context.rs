use download::Downloader;
use error::WaldoError;
use photo::{Photo, PhotoResource};
use storage::Storage;


pub struct Context {
    pub database: Storage
}


impl Context {
    pub fn update_catalog(&self) -> Result<(), WaldoError> {
        // Fetch the XML from S3
        let downloader = Downloader {
            uri: "http://s3.amazonaws.com/waldo-recruiting".to_string()
        };

        downloader.download(Some("".to_string()), |response| -> Result<(), WaldoError> {
            // Parse all the photo resources
            let photo_resources = PhotoResource::new_many(&response)?;

            println!("{:?}", photo_resources);
            println!("Total: {} photos", photo_resources.len());

            // Download all of them
            for photo_resource in photo_resources {
                if let Err(err) = downloader.download_photo(&photo_resource, |response| -> Result<(), WaldoError> {
                    // Photo data is downloaded; parse the photo
                    let photo = Photo::new(&photo_resource.key, response)?;

                    // ...and shove it into our storage
                    self.database.store_one(photo)?;

                    Ok(())
                }) {
                    println!("Error parsing or downloading photo {}: {}", photo_resource.key, err);
                    continue;
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    pub fn get_photo(&self, uuid: &str) -> Result<Photo, WaldoError> {
        self.database.fetch(uuid)
    }
}