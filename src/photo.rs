use bson;
use error::WaldoError;
use error::WaldoError::ParseError;
use xmltree::Element;


#[derive(Debug)]
pub struct Photo {
    pub key: String,
    pub last_modified: String,  // TODO: Should be parsed as a DateTime
    pub etag: String,
    pub size: u64,
    pub storage_class: String  // TODO: Should be typed as a StorageClass enum
}


impl Photo {
    pub fn new_many(xml_data: &[u8]) -> Result<Vec<Photo>, WaldoError> {
        match Element::parse(xml_data) {
            Ok(root) => {
                // Parse a Photo element out of the XML
                return Photo::parse_many(root);
            },
            Err(err) => {
                return Err(WaldoError::MalformedError(err));
            }
        }
    }

    /// Parse a single Photo out of an XML element
    pub fn parse_one(mut xml_element: Element) -> Result<Photo, WaldoError> {
        let key = xml_element.take_child("Key").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let last_modified = xml_element.take_child("LastModified").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let etag = xml_element.take_child("ETag").ok_or(ParseError)?.text.ok_or(ParseError)?;
        let size = try!(String::from(xml_element.take_child("Size").ok_or(ParseError)?.text.ok_or(ParseError)?).parse());
        let storage_class = xml_element.take_child("StorageClass").ok_or(ParseError)?.text.ok_or(ParseError)?;

        return Ok(Photo {
            key: String::from(key),
            last_modified: String::from(last_modified),
            etag: String::from(etag),
            size: size,
            storage_class: String::from(storage_class)
        })
    }

    /// Parse multiple Photos out of an XML <Contents> element
    pub fn parse_many(mut xml_element: Element) -> Result<Vec<Photo>, WaldoError> {
        let mut photos = Vec::new();

        loop {
            match xml_element.take_child("Contents") {
                Some(child_xml) => {
                    photos.push(try!(Photo::parse_one(child_xml)));
                },
                None => break
            }
        }

        Ok(photos)
    }

    pub fn to_mongo_document(&self) -> bson::Document {
        let ref key = self.key;
        let ref last_modified = self.last_modified;
        let ref etag = self.etag;
        let size = self.size;
        let ref storage_class = self.storage_class;

        doc! {
            "_id" => key,  // Primary index key
            "key" => key,
            "last_modified" => last_modified,
            "etag" => etag,
            "size" => size,
            "storage_class" => storage_class
        }
    }
}