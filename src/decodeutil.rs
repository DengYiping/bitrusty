#![macro_use]

use bencode;
use std::{convert, io};
use bencode::VecFromBencodeError;


#[macro_export]
macro_rules! get_field_with_default {
    ($m:expr, $field:expr, $default:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(a) => (FromBencode::from_bencode(a))?,
            None => $default
        }
    )
}

#[macro_export]
macro_rules! get_field {
    ($m:expr, $field:expr) => (
        get_field_with_default!($m, $field, return Err(decodeutil::Error::DoesntContain($field)))
    )
}

#[macro_export]
macro_rules! get_optional_field {
    ($m:expr, $field:expr) => (
        get_field_with_default!($m, $field, None)
    )
}

#[macro_export]
macro_rules! get_raw_field {
    ($m:expr, $field:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(a) => a,
            None => return Err(decodeutil::Error::DoesntContain($field))
        }
    )
}

#[macro_export]
macro_rules! get_field_as_bencoded_bytes {
    ($m:expr, $field:expr) => (
        get_raw_field!($m, $field).to_bytes()?
    )
}

#[macro_export]
macro_rules! get_field_as_bytes {
    ($m:expr, $field:expr) => (
        match get_raw_field!($m, $field) {
            &Bencode::ByteString(ref v) => v.clone(),
            _ => return Err(decodeutil::Error::NotAByteString)
        }
    )
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DecodingError(bencode::streaming::Error),
    NotADict,
    NotAByteString,
    DoesntContain(&'static str),
    NotANumber(bencode::NumFromBencodeError),
    NotAString(bencode::StringFromBencodeError),
    NotAVec
}


impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl convert::From<Error> for io::Error {
    fn from(err: Error) -> Self {
        println!("error detail = {:?}", err);
        io::Error::new(io::ErrorKind::InvalidData, "invalid data to use bencode")
    }
}

impl convert::From<bencode::streaming::Error> for Error {
    fn from(err: bencode::streaming::Error) -> Error {
        Error::DecodingError(err)
    }
}

impl convert::From<bencode::NumFromBencodeError> for Error {
    fn from(err: bencode::NumFromBencodeError) -> Error {
        Error::NotANumber(err)
    }
}

impl convert::From<bencode::StringFromBencodeError> for Error {
    fn from(err: bencode::StringFromBencodeError) -> Error {
        Error::NotAString(err)
    }
}

impl convert::From<bencode::VecFromBencodeError<Error>> for Error {
    fn from(err: VecFromBencodeError<Error>) -> Self {
        Error::NotAVec
    }
}

impl convert::From<bencode::VecFromBencodeError<bencode::StringFromBencodeError>> for Error {
    fn from(err: VecFromBencodeError<bencode::StringFromBencodeError>) -> Self {
        Error::NotAVec
    }
}