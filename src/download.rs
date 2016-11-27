use curl::easy::Easy;
use error::WaldoError;
use photo::PhotoResource;


pub struct Downloader {
    pub uri: String
}


impl Downloader {
    /// Downloads the given resource and passes it fully to the `completion` closure
    pub fn download<F>(&self, resource: Option<String>, mut completion: F) -> Result<(), WaldoError>
        where F: FnMut(&[u8]) -> Result<(), WaldoError> {

        // Fetch the XML from S3
        let mut http_client = Easy::new();

        let url = self.uri.clone() + "/" + &resource.unwrap_or("".to_string());

        http_client.url(&url)?;

        println!("Downloading: {}", url);

        let mut response = Vec::new();

        {
            let mut transfer = http_client.transfer();
            transfer.write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            })?;

            transfer.perform()?;
        }

        completion(&response)
    }

    pub fn download_photo<F>(&self, photo_resource: &PhotoResource, completion: F) -> Result<(), WaldoError>
        where F: FnMut(&[u8]) -> Result<(), WaldoError> {

        self.download(Some(photo_resource.key.clone()), completion)
    }
}