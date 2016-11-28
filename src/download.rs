use curl::easy::Easy;
use error::WaldoError;


/// Downloads the given resource and passes it fully to the `completion` closure
pub fn download<F>(url: &str, mut completion: F) -> Result<(), WaldoError>
    where F: FnMut(&[u8]) -> Result<(), WaldoError> {

    // Fetch the XML from S3
    let mut http_client = Easy::new();

    http_client.url(url)?;

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