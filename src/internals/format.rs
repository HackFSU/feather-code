//! Traits to encapsulate encoding and decoding

/// Representation of a barcode format
pub trait Format {

    /// Perform format specific validation to verify data integrity
    fn checksum(&self) -> bool;
}


/// Support decoding a particular format to the target type
pub trait Decode<F: Format> where Self: Sized {

    /// Convert a formatted data value
    fn decode(&F) -> Result<Self, FormatErr>;
}


/// Support encoding the target type as a particular format
pub trait Encode<F: Format> {

    /// Convert to a given format
    fn encode(&self) -> Result<F, FormatErr>;
}

/// Describes failure cases for encoding and decoding barcodes
#[derive(Debug,PartialEq)]
pub enum FormatErr {
    /// Barcode is too short or too long
    InvalidLength(usize),
    /// Invalid internal format
    BadFormat(String),
    /// Encode failure
    EncodeErr(String),
    /// Decode failure
    DecodeErr(String),
}
