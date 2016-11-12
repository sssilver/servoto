#[derive(Debug)]
pub struct Photo {
    pub key: String,
    pub last_modified: String,  // TODO: Should be parsed as a DateTime
    pub etag: String,
    pub size: u64,
    pub storage_class: String  // TODO: Should be typed as a StorageClass enum
}