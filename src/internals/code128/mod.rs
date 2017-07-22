//! Barcode 128 standard as data representation
//!
//! ![barcode example](https://upload.wikimedia.org/wikipedia/commons/4/41/9494-RI476394652CH.jpg)
//!
//! Encapsulates logic for encoding and decoding barcodes using the [`Format`], [`Decode`], and
//! [`Encode`] traits.
//!
//! [`Format`]: ../format/trait.Format.html
//! [`Encode`]: ../format/trait.Encode.html
//! [`Decode`]: ../format/trait.Decode.html

use std::fmt::Debug;
use super::format;
use super::format::{Format, Decode};
pub mod encodings;

/// Code128 alphabets (symbologies) which specify how [patterns][`Encoding`] map to characters
///
/// [`Encoding`]: trait.Encoding.html
#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub enum Symbology {
    /// (ASCII 00 to 95) A-Z, 0-9, and special characters
    A = 103,
    /// (ASCII 32-127) a-z, A-Z, and 0-9
    B = 104,
    /// High density, number pair encoding
    C = 105,
}

/// Interface for types which represent Code128 encodings
///
/// Code128 encodings are patterns whch reprsent numerical values from 0 to 106.  Each encoding
/// maps to a differet subset of ascii values depending on the [`Symbology`]:
///
///  u8  | [`A`] | [`B`] | [`C`]
/// :----|:-----:|:-----:|:------:
///  0   | space | space | 00
///  1   | !     | !     | 01
///  2   | "     | "     | 02
///  3   | #     | #     | 03
///  4   | $     | $     | 04
///  5   | %     | %     | 05
///  6   | &     | &     | 06
///  7   | '     | '     | 07
///  8   | (     | (     | 08
///  9   | )     | )     | 09
///  10  | \*    | \*    | 10
///  11  | +     | +     | 11
///  12  | ,     | ,     | 12
///  13  | -     | -     | 13
///  14  | .     | .     | 14
///  15  | /     | /     | 15
///  16  | 0     | 0     | 16
///  17  | 1     | 1     | 17
///  18  | 2     | 2     | 18
///  19  | 3     | 3     | 19
///  20  | 4     | 4     | 20
///  21  | 5     | 5     | 21
///  22  | 6     | 6     | 22
///  23  | 7     | 7     | 23
///  24  | 8     | 8     | 24
///  25  | 9     | 9     | 25
///  26  | :     | :     | 26
///  27  | ;     | ;     | 27
///  28  | <     | <     | 28
///  29  | =     | =     | 29
///  30  | >     | >     | 30
///  31  | ?     | ?     | 31
///  32  | @     | @     | 32
///  33  | A     | A     | 33
///  34  | B     | B     | 34
///  35  | C     | C     | 35
///  36  | D     | D     | 36
///  37  | E     | E     | 37
///  38  | F     | F     | 38
///  39  | G     | G     | 39
///  40  | H     | H     | 40
///  41  | I     | I     | 41
///  42  | J     | J     | 42
///  43  | K     | K     | 43
///  44  | L     | L     | 44
///  45  | M     | M     | 45
///  46  | N     | N     | 46
///  47  | O     | O     | 47
///  48  | P     | P     | 48
///  49  | Q     | Q     | 49
///  50  | R     | R     | 50
///  51  | S     | S     | 51
///  52  | T     | T     | 52
///  53  | U     | U     | 53
///  54  | V     | V     | 54
///  55  | W     | W     | 55
///  56  | X     | X     | 56
///  57  | Y     | Y     | 57
///  58  | Z     | Z     | 58
///  59  | [     | [     | 59
///  60  | \\    | \\    | 60
///  61  | ]     | ]     | 61
///  62  | ^     | ^     | 62
///  63  | \_    | \_    | 63
///  64  | NUL   | `     | 64
///  65  | SOH   | a     | 65
///  66  | STX   | b     | 66
///  67  | ETX   | c     | 67
///  68  | EOT   | d     | 68
///  69  | ENQ   | e     | 69
///  70  | ACK   | f     | 70
///  71  | BEL   | g     | 71
///  72  | BS    | h     | 72
///  73  | HT    | i     | 73
///  74  | LF    | j     | 74
///  75  | VT    | k     | 75
///  76  | FF    | l     | 76
///  77  | CR    | m     | 77
///  78  | SO    | n     | 78
///  79  | SI    | o     | 79
///  80  | DLE   | p     | 80
///  81  | DC1   | q     | 81
///  82  | DC2   | r     | 82
///  83  | DC3   | s     | 83
///  84  | DC4   | t     | 84
///  85  | NAK   | u     | 85
///  86  | SYN   | v     | 86
///  87  | ETB   | w     | 87
///  88  | CAN   | x     | 88
///  89  | EM    | y     | 89
///  90  | SUB   | z     | 90
///  91  | ESC   | {     | 91
///  92  | FS    | \|    | 92
///  93  | GS    | }     | 93
///  94  | RS    | ~     | 94
///  95  | US    | DEL   | 95
///  96  | FNC 3 | FNC 3 | 96
///  97  | FNC 2 | FNC 2 | 97
///  98  |Shift B|Shift A| 98
///  99  |Code C |Code C | 99
///  100 |Code B | FNC 4 | Code B
///  101 | FNC 4 |Code A | Code A
///  102 | FNC 1 | FNC 1 | FNC 1
///  103 |Start A|Start A| Start A
///  104 |Start B|Start B| Start B
///  105 |Start C|Start C| Start C
///  106 | stop  | stop  | stop
///
/// [`A`]: enum.Symbology.html#variant.A
/// [`B`]: enum.Symbology.html#variant.B
/// [`C`]: enum.Symbology.html#variant.C
pub trait Encoding: From<u8> + Into<u8> + PartialOrd {
    /// Convert an encoding to its string representation in a given symbology
    fn repr(&self, Symbology) -> String;

    /// Get the stop value in the particular encoding format
    ///
    /// Correspond to numerical values such that:
    ///
    ///  u8  | [`A`] | [`B`] | [`C`]
    /// :----|:-----:|:-----:|:------:
    ///  106 | stop  | stop  | stop
    ///
    /// [`A`]: enum.Symbology.html#variant.A
    /// [`B`]: enum.Symbology.html#variant.B
    /// [`C`]: enum.Symbology.html#variant.C
    fn stop() -> Self;

    /// Switch symbol for a given symbology
    ///
    /// Correspond to numerical values such that:
    ///
    ///  u8  | [`A`] | [`B`] | [`C`]
    /// :----|:-----:|:-----:|:------:
    ///  99  |Code C |Code C |  ...
    ///  100 |Code B |  ...  | Code B
    ///  101 |  ...  |Code A | Code A
    ///
    /// # Examples
    ///
    /// ```
    /// # use feather_code::barcode::code128::Encoding;
    /// # use feather_code::barcode::code128::Symbology;
    ///
    /// assert_eq!(u8::switch(Symbology::A), 101);
    /// assert_eq!(u8::switch(Symbology::B), 100);
    /// assert_eq!(u8::switch(Symbology::C), 99);
    /// ```
    ///
    /// [`A`]: enum.Symbology.html#variant.A
    /// [`B`]: enum.Symbology.html#variant.B
    /// [`C`]: enum.Symbology.html#variant.C
    fn switch(Symbology) -> Self;

    /// Start symbol for a given symbology
    ///
    /// Correspond to numerical values such that
    ///
    ///  u8  | [`A`] | [`B`] | [`C`]
    /// :----|:-----:|:-----:|:------:
    ///  103 |Start A|Start A| Start A
    ///  104 |Start B|Start B| Start B
    ///  105 |Start C|Start C| Start C
    ///
    /// [`A`]: enum.Symbology.html#variant.A
    /// [`B`]: enum.Symbology.html#variant.B
    /// [`C`]: enum.Symbology.html#variant.C
    fn start(Symbology) -> Self;

    /// Shift code wich indicates that the next encoding uses the shifted symbology
    ///
    /// When in symbology A or B, the shift code indicates the next encoding will use the other
    /// symbology to parse.  If we have an encoded value which encodes a lot of lowercase
    /// characters, we may use Symbology [`B`].  If we need to add a single special character to
    /// the sequence though, we can shift into symbology [`A`] for a single encoding to add the
    /// special character.
    ///
    /// Correspond to numerical values such that
    ///
    ///  u8  | [`A`] | [`B`]
    /// :----|:-----:|:-----:
    ///  98  |Shift B|Shift A
    ///
    /// [`A`]: enum.Symbology.html#variant.A
    /// [`B`]: enum.Symbology.html#variant.B
    fn shift() -> Self;

    /// Function 1 encoding, indicates special behaviour, ignored in the spec
    fn fnc1() -> Self;

    /// Reserved encoding for function 2, currently in the spec but not used
    fn fnc2() -> Self;

    /// Reserved encoding for function 3, currently in the spec but not used
    fn fnc3() -> Self;

    /// Reserved encoding for function 4, currently in the spec but not used
    ///
    /// Correspond to numerical values such that:
    ///
    ///  u8  | [`A`] | [`B`]
    /// :----|:-----:|:-----:
    ///  100 |  ...  | FNC 4
    ///  101 | FNC 4 |  ...
    ///
    /// [`A`]: enum.Symbology.html#variant.A
    /// [`B`]: enum.Symbology.html#variant.B
    fn fnc4(Symbology) -> Option<Self>;

    /// Representation as a u8 for non-copy types to calculate checksum
    fn as_u8(&self) -> u8;
}

/// [Code128][wiki] barcode format
///
/// # [`Symbology`]
///
/// Code128 encodes the full 128 characters of ASCII using three different symbologies which
/// represent different subsets of ASCII.
///
/// - [`A`]: A-Z, 0-9, and special characters (ASCII 00 to 95)
/// - [`B`]: a-z, A-Z, and 0-9 (ASCII 32-127)
/// - [`C`]: high density, number pair encoding
///
/// The 107 different patterns map differently to ASCII characters for each
/// symbology, see documentation for `Pattern` or references for more details.
///
/// # References
/// - [Code 128 on Wikipedia][wiki]
///
/// [wiki]: https://en.wikipedia.org/wiki/Code_128
/// [`Symbology`]: enum.Symbology.html
/// [`A`]: enum.Symbology.html#variant.A
/// [`B`]: enum.Symbology.html#variant.B
/// [`C`]: enum.Symbology.html#variant.C
#[derive(PartialEq,Eq,Debug)]
pub struct Code128<'a, E>(pub &'a [E]) where E: 'a + Encoding;

impl<'a, E> Code128<'a, E> where E: 'a + Encoding {
    /// If the encoding has valid format, returns the parts of the datum
    ///
    /// A valid [`Code128`] datum has a start encoding which determines the proper start symbology,
    /// a series of encodings which represent data, a checksum digit to ensure that the data is
    /// properly formatted, and a stop encoding which tells the barcode scanner to stop scanning.
    fn data(&self) -> Option<(Symbology, &'a [E], &E)> {
        let (start, rest) = match self.0.split_first() {
            Some(x) => x,
            None => return None,
        };
        let rest = match rest.split_last() {
            Some((stop,x)) if *stop == E::stop() => x,
            _ => return None,
        };
        let (check, symbols) = match rest.split_last() {
            Some(x) => x,
            None => return None,
        };

        match start {
            _ if *start == E::start(Symbology::A) => Some((Symbology::A, symbols, &check)),
            _ if *start == E::start(Symbology::B) => Some((Symbology::B, symbols, &check)),
            _ if *start == E::start(Symbology::C) => Some((Symbology::C, symbols, &check)),
            _ => None,
        }
    }
}

impl<'a, E: 'a + Encoding> Format for Code128<'a, E> {
    /// Verify data integrity, returning true if data is properly formatted, has the
    /// proper length, and has the right modulo 103 checksum digit.
    ///
    /// # Example
    ///
    /// ```
    /// # use feather_code::barcode::code128::Code128;
    /// # use feather_code::barcode::format::Format;
    ///
    /// assert!(Code128(&[103, 48, 42, 42, 17, 18, 19, 35, 54, 106]).checksum());
    /// ```
    fn checksum(&self) -> bool {
        // split slice into respective parts, and return false indicating invalid
        // data if the formatted data isn't the proper length
        let (start, symbols, check) = match self.data() {
            Some(x) => x,
            None => return false,
        };

        // Sum raw numerical values from each symbol multiplied by its position
        let sum: u64 = {
            let mut pos: u64 = 0;
            symbols.iter()
                .fold(0, |sum, pat| {
                    let raw: u64 = pat.as_u8() as u64;
                    pos += 1;
                    sum + raw * pos
                })
        } + start as u64;

        // Checksum is the remainder after dividing the raw code sum by 103
        E::from((sum % 103) as u8) == *check
    }
}

impl<'a, E> Decode<String> for Code128<'a, E> where E: 'a + Encoding + Debug {

    /// Convert Code128 barcode representation to a string
    ///
    /// In case of badly formatted data, will return a [`format::Result`] detailing the issue.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use feather_code::internals::format::Decode;
    /// # use feather_code::internals::code128::Code128;
    /// # use feather_code::internals::code128::encodings::Pattern::*;
    /// let buffer = [C105, C102, C42, C18, C40, C20, C50, C101, C16, C92, C106];
    /// let country_code: String = Code128(buffer.as_ref()).decode().unwrap();
    ///
    /// assert_eq!(country_code, "42184020500".to_string());
    /// ```
    fn decode(&self) -> format::Result<String> {
        use super::format::Error::*;
        use super::code128::Symbology::*;

        if self.0.len() < 4 { return Err(InvalidLength(self.0.len())) }

        let mut decoded: String = "".to_string();
        // Grab start code or return with error in case of bad format
        let (state, symbols, _) = match self.data() {
            Some(x) => x,
            _ => return Err(BadFormat("unrecognized format".into())),
        };

        enum Parser { A, B, C, ShiftA, ShiftB }

        let mut state = match state {
            A => Parser::A,
            B => Parser::B,
            C => Parser::C,
        };

        // Use finite state machine to parse Code128 to a String
        'parser: for e in symbols.iter() {
            state = match state {
                Parser::A => {
                    match e.as_u8() {
                        n if n < 98 => {decoded.push_str(&e.repr(A)); Parser::A},
                        100 => Parser::B, // Switch to symbology B
                        99 => Parser::C, // Switch to symbology C
                        98 => Parser::ShiftB, // shift code
                        106 => break 'parser,
                        102 | 97 | 96 | 101 => Parser::A, // function 1, 2, 3, 4, disabled
                        _ => return Err(DecodeErr(format!("unrecognized encoding {:?}", *e))),
                    }
                },
                Parser::B => {
                    match e.as_u8() {
                        n if n < 98 => {decoded.push_str(&e.repr(B)); Parser::B},
                        101 => Parser::B, // Switch to symbology A
                        99 => Parser::B, // Switch to symbology C
                        106 => break 'parser,
                        98 => Parser::ShiftA, // shift code
                        102 | 97 | 96 | 100 => Parser::B, // function 1, 2, 3, 4, disabled
                        _ => return Err(DecodeErr(format!("unrecognized encoding {:?}", *e))),
                    }
                },
                Parser::C => {
                    match e.as_u8() {
                        n if n < 100 => {decoded.push_str(&e.repr(C)); Parser::C}
                        100 => Parser::B, // Switch to symbology B
                        101 => Parser::A, // Switch to symbology A
                        106 => break 'parser,
                        102 => Parser::C, // function 1, disabled
                        _ => return Err(DecodeErr(format!("unexpected encoding {:?}", *e))),
                    }
                },
                Parser::ShiftA => {
                    match e.as_u8() {
                        n if n < 98 => {decoded.push_str(&e.repr(A)); Parser::B},
                        _ => return Err(DecodeErr(format!("unexpected shifted encoding {:?}", *e))),
                    }
                },
                Parser::ShiftB => {
                    match e.as_u8() {
                        n if n < 98 => {decoded.push_str(&e.repr(B)); Parser::A},
                        _ => return Err(DecodeErr(format!("unexpected shifted encoding {:?}", *e))),
                    }
                },
            };
        };
        Ok(decoded)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn checksum() {
        use internals::format::Format;
        use internals::code128::encodings::Pattern::*;
        use internals::code128::Code128;

        assert!(Code128(&[C103, C48, C42, C42, C17, C18, C19, C35, C54, C106]).checksum());

        assert!(Code128(&[C105, C102, C42, C18, C40, C20, C50, C101, C16, C92, C106]).checksum());
        assert!(Code128(&[105, 102, 42, 18, 40, 20, 50, 101, 16, 92, 106]).checksum());
    }

    #[test]
    fn decode() {
        use internals::code128::Code128;
        use internals::format::Decode;

        let pjj123_c = [103, 48, 42, 42, 17, 18, 19, 35, 54, 106];

        assert_eq!(Code128(&pjj123_c).decode().unwrap(), "PJJ123C".to_string());

        let country_code = [105, 102, 42, 18, 40, 20, 50, 101, 16, 92, 106];

        assert_eq!(Code128(&country_code).decode().unwrap(), "42184020500".to_string());

        let hello_world = [104, 40, 69, 76, 76, 79, 0, 55, 79, 82, 76, 68, 43, 106];

        assert_eq!(Code128(&hello_world).decode().unwrap(), "Hello World".to_string());

        let shift_codes = [103, 51, 40, 98, 73, 38, 52, 100, 98, 1, 34, 106];

        assert_eq!(Code128(&shift_codes).decode().unwrap(), "SHiFT!".to_string())
    }

    #[test]
    fn split_data() {
        use internals::code128::Code128;
        use internals::code128::encodings::Pattern::*;
        use internals::code128::Symbology::*;

        let symbols = [C103, C48, C42, C42, C17, C18, C19, C35, C54, C106];
        let code = Code128(&symbols);

        assert_eq!(code.data(), Some((A, [C48, C42, C42, C17, C18, C19, C35].as_ref(), &C54)));

        let symbols = [C103];
        let code = Code128(&symbols);

        assert_eq!(code.data(), None);

        let symbols = [C103, C106];
        let code = Code128(&symbols);

        assert_eq!(code.data(), None);

        let symbols = [C103, C48, C15, C106];
        let code = Code128(&symbols);

        assert_eq!(code.data(), Some((A, [C48].as_ref(), &C15)));
    }

    quickcheck! {
        fn short_data_is_invalid(symbols: Vec<u8>) -> bool {
            use internals::code128::Code128;

            if symbols.len() < 4 {
                Code128(symbols.as_ref()).data() == None
            } else {
                true
            }
        }
    }
}
