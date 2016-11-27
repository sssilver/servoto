use error::WaldoError;
use error::WaldoError::ParseError;
use rexiv2::Metadata;
use std::collections::HashMap;
use std::str::FromStr;
use storage_class::StorageClass;
use xmltree::Element;


#[derive(Debug)]
pub struct PhotoResource {
    pub key: String,
    pub last_modified: String,  // TODO: Should be parsed as a DateTime
    pub etag: String,
    pub size: u64,
    pub storage_class: StorageClass
}


impl PhotoResource {
    pub fn new_many(xml_data: &[u8]) -> Result<Vec<PhotoResource>, WaldoError> {
        match Element::parse(xml_data) {
            Ok(root) => {
                // Parse a Photo element out of the XML
                return PhotoResource::parse_many(root);
            },
            Err(err) => {
                return Err(WaldoError::MalformedError(err));
            }
        }
    }

    /// Parse a single Photo out of an XML element
    pub fn parse_one(mut xml_element: Element) -> Result<PhotoResource, WaldoError> {
        let key = xml_element.take_child("Key").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let last_modified = xml_element.take_child("LastModified").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let etag = xml_element.take_child("ETag").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let size = String::from(xml_element.take_child("Size").ok_or(ParseError)?.text.ok_or(ParseError)?).parse()?;
        let storage_class = StorageClass::from_str(&xml_element.take_child("StorageClass").ok_or(ParseError)?.text.ok_or(ParseError)?)?;

        return Ok(PhotoResource {
            key: key,
            last_modified: last_modified,
            etag: etag,
            size: size,
            storage_class: storage_class
        })
    }

    /// Parse multiple Photos out of an XML <Contents> element
    pub fn parse_many(mut xml_element: Element) -> Result<Vec<PhotoResource>, WaldoError> {
        let mut photos = Vec::new();

        loop {
            match xml_element.take_child("Contents") {
                Some(child_xml) => {
                    photos.push(PhotoResource::parse_one(child_xml)?);
                },
                None => break
            }
        }

        Ok(photos)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Photo {
    pub key: String,
    pub exif_tags: HashMap<String, String>
}


impl Photo {
    pub fn new(key: &str, data: &[u8]) -> Result<Photo, WaldoError> {
        // Parse EXIF
        let meta = Metadata::new_from_buffer(data)?;
        let mut exif_tags = HashMap::new();

        let tag_names = meta.get_exif_tags()?;

        for tag_name in tag_names {
            exif_tags.insert(tag_name.to_string(), meta.get_tag_string(&tag_name)?);
        }

        println!("EXIF: {:?}", exif_tags);

        Ok(Photo {
            key: key.to_string(),
            exif_tags: exif_tags
        })
    }
}