//! Abstract representation of barcode, or other, data encoding formats
//!
//! [`Decode`] and [`Encode`] must be idempotent.
use std::result;

/// Representation of a barcode format
pub trait Format {

    /// Perform format specific validation to verify data integrity
    fn checksum(&self) -> bool;
}

/// Specialized result type for errors in barcode conversions
pub type Result<T> = result::Result<T, Error>;

/// Describes failure cases for encoding and decoding barcodes
#[derive(Debug,PartialEq)]
pub enum Error {
    /// Barcode is too short or too long
    InvalidLength(usize),
    /// Invalid internal format
    BadFormat(String),
    /// Encode failure
    EncodeErr(String),
    /// Decode failure
    DecodeErr(String),
}

/// Support decoding a particular format to the target type
pub trait Decode<T> where Self: Format {

    /// Convert a formatted data value
    fn decode(&self) -> Result<T>;
}


/// Support encoding the target type as a particular format
pub trait Encode<F: Format> {

    /// Convert to a given format
    fn encode(&self) -> Result<F>;
}
