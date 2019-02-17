extern crate bencode;
use bencode::{FromBencode, Bencode};
use bencode::util::ByteString;

use super::decodeutil;
use crate::hash::calculate_sha1_bytes;

/**
  * metainfo in the torrent file, we extract them here
  */
#[derive(Debug, PartialEq)]
pub struct MetaInfo {
    pub announce: String,

    // not yet implemented
    // pub announce_list: Option<Vec<Vec<String>>>,
    pub info: Info,
    pub info_hash: Vec<u8>,

    pub created_by: String,
}

impl FromBencode for MetaInfo {
    type Err = decodeutil::Error;

    fn from_bencode(bencode: &Bencode) -> Result<Self, Self::Err> {
        match bencode {
            &Bencode::Dict(ref dt) => {
                let info_raw_bytes = get_field_as_bencoded_bytes!(dt, "info");
                let info_hash = calculate_sha1_bytes(&info_raw_bytes);

               Ok(MetaInfo {
                   announce: get_field!(dt, "announce"),
                   info: get_field!(dt, "info"),
                   info_hash,
                   created_by: get_field_with_default!(dt, "created by", "".to_string()),
               })
            }
            _ => Err(decodeutil::Error::NotADict)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Info {
    pub piece_length: u32, // shared field
    pub pieces: Vec<Vec<u8>>, // shared field
    pub num_pieces: u32, // shared field
    pub name: String, // shared field

    pub length: Option<u64>, // note, this field is not avaiable for multi-file mode

    pub files: Option<Vec<InfoMultiFile>> // only in the multi file
}

impl Info {
    fn is_single_file(&self) -> bool {
        self.length.is_some() && self.files.is_none()
    }

    fn is_multiple_file(&self) -> bool {
        self.length.is_none() && self.files.is_some()
    }
}


impl FromBencode for Info {
    type Err = decodeutil::Error;
    fn from_bencode(bencode: &Bencode) -> Result<Info, decodeutil::Error> {

        match bencode {
            &Bencode::Dict(ref dt) => {
                let pieces:Vec<Vec<u8>> = get_field_as_bytes!(dt, "pieces").chunks(20).map(|v| v.to_owned()).collect();
                Ok(Info {
                    piece_length: get_field!(dt, "piece length"),
                    num_pieces: pieces.len() as u32,
                    pieces,
                    name: get_field!(dt, "name"),
                    length: get_optional_field!(dt, "length"),
                    files: get_optional_field!(dt, "files")
                })
            }
            _ => Err(decodeutil::Error::NotADict)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InfoMultiFile {
    pub length: u64,
    pub path: Vec<String>
}

impl FromBencode for InfoMultiFile {

    type Err = decodeutil::Error;

    fn from_bencode(bencode: &Bencode) -> Result<Self, Self::Err> {
        match bencode {
            &Bencode::Dict(ref dt) => {
                Ok(InfoMultiFile {
                    length: get_field!(dt, "length"),
                    path: get_field!(dt, "path")
                })
            }
            _ => Err(decodeutil::Error::NotADict)
        }
    }
}

// parse a input torrent into readable struct
pub fn parse_metainfo(bytes: &Vec<u8>) -> Result<MetaInfo, decodeutil::Error> {
   let parsed_to_encode = bencode::from_buffer(bytes)?;
    FromBencode::from_bencode(&parsed_to_encode)
}