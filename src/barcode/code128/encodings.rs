//! Implementation of Code128 barcode encodings

use super::{Encoding, Symbology};

/// Representation of Code128 patterns
///
/// Representation of the symbols used in Code128; depending on the active code
/// set, each symbol maps to one of 3 ASCII values in official Code128.
///
/// Pattern | A     | B     | C
/// :-------|:-----:|:-----:|:------:
/// C0      | space | space | 00
/// C1      | !     | !     | 01
/// C2      | "     | "     | 02
/// C3      | #     | #     | 03
/// C4      | $     | $     | 04
/// C5      | %     | %     | 05
/// C6      | &     | &     | 06
/// C7      | '     | '     | 07
/// C8      | (     | (     | 08
/// C9      | )     | )     | 09
/// C10     | \*    | \*    | 10
/// C11     | +     | +     | 11
/// C12     | ,     | ,     | 12
/// C13     | -     | -     | 13
/// C14     | .     | .     | 14
/// C15     | /     | /     | 15
/// C16     | 0     | 0     | 16
/// C17     | 1     | 1     | 17
/// C18     | 2     | 2     | 18
/// C19     | 3     | 3     | 19
/// C20     | 4     | 4     | 20
/// C21     | 5     | 5     | 21
/// C22     | 6     | 6     | 22
/// C23     | 7     | 7     | 23
/// C24     | 8     | 8     | 24
/// C25     | 9     | 9     | 25
/// C26     | :     | :     | 26
/// C27     | ;     | ;     | 27
/// C28     | <     | <     | 28
/// C29     | =     | =     | 29
/// C30     | >     | >     | 30
/// C31     | ?     | ?     | 31
/// C32     | @     | @     | 32
/// C33     | A     | A     | 33
/// C34     | B     | B     | 34
/// C35     | C     | C     | 35
/// C36     | D     | D     | 36
/// C37     | E     | E     | 37
/// C38     | F     | F     | 38
/// C39     | G     | G     | 39
/// C40     | H     | H     | 40
/// C41     | I     | I     | 41
/// C42     | J     | J     | 42
/// C43     | K     | K     | 43
/// C44     | L     | L     | 44
/// C45     | M     | M     | 45
/// C46     | N     | N     | 46
/// C47     | O     | O     | 47
/// C48     | P     | P     | 48
/// C49     | Q     | Q     | 49
/// C50     | R     | R     | 50
/// C51     | S     | S     | 51
/// C52     | T     | T     | 52
/// C53     | U     | U     | 53
/// C54     | V     | V     | 54
/// C55     | W     | W     | 55
/// C56     | X     | X     | 56
/// C57     | Y     | Y     | 57
/// C58     | Z     | Z     | 58
/// C59     | [     | [     | 59
/// C60     | \\    | \\    | 60
/// C61     | ]     | ]     | 61
/// C62     | ^     | ^     | 62
/// C63     | \_    | \_    | 63
/// C64     | NUL   | `     | 64
/// C65     | SOH   | a     | 65
/// C66     | STX   | b     | 66
/// C67     | ETX   | c     | 67
/// C68     | EOT   | d     | 68
/// C69     | ENQ   | e     | 69
/// C70     | ACK   | f     | 70
/// C71     | BEL   | g     | 71
/// C72     | BS    | h     | 72
/// C73     | HT    | i     | 73
/// C74     | LF    | j     | 74
/// C75     | VT    | k     | 75
/// C76     | FF    | l     | 76
/// C77     | CR    | m     | 77
/// C78     | SO    | n     | 78
/// C79     | SI    | o     | 79
/// C80     | DLE   | p     | 80
/// C81     | DC1   | q     | 81
/// C82     | DC2   | r     | 82
/// C83     | DC3   | s     | 83
/// C84     | DC4   | t     | 84
/// C85     | NAK   | u     | 85
/// C86     | SYN   | v     | 86
/// C87     | ETB   | w     | 87
/// C88     | CAN   | x     | 88
/// C89     | EM    | y     | 89
/// C90     | SUB   | z     | 90
/// C91     | ESC   | {     | 91
/// C92     | FS    | \|    | 92
/// C93     | GS    | }     | 93
/// C94     | RS    | ~     | 94
/// C95     | US    | DEL   | 95
/// C96     | FNC 3 | FNC 3 | 96
/// C97     | FNC 2 | FNC 2 | 97
/// C98     |Shift B|Shift A| 98
/// C99     |Code C |Code C | 99
/// C100    |Code B | FNC 4 | Code B
/// C101    | FNC 4 |Code A | Code A
/// C102    | FNC 1 | FNC 1 | FNC 1
/// C103    |Start A|Start A| Start A
/// C104    |Start B|Start B| Start B
/// C105    |Start C|Start C| Start C
/// C106    | stop  | stop  | stop
#[allow(missing_docs)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Debug,Clone,Copy)]
pub enum Pattern {
    C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10,
    C11, C12, C13, C14, C15, C16, C17, C18, C19,
    C20, C21, C22, C23, C24, C25, C26, C27, C28,
    C29, C30, C31, C32, C33, C34, C35, C36, C37,
    C38, C39, C40, C41, C42, C43, C44, C45, C46,
    C47, C48, C49, C50, C51, C52, C53, C54, C55,
    C56, C57, C58, C59, C60, C61, C62, C63, C64,
    C65, C66, C67, C68, C69, C70, C71, C72, C73,
    C74, C75, C76, C77, C78, C79, C80, C81, C82,
    C83, C84, C85, C86, C87, C88, C89, C90, C91,
    C92, C93, C94, C95, C96, C97, C98, C99, C100,
    C101, C102, C103, C104, C105, C106,
}

// Jump table conversion
impl From<u8> for Pattern {
    fn from(u: u8) -> Pattern {
        use self::Pattern::*;
        match u {
            0 => C0, 1 => C1, 2 => C2, 3 => C3, 4 => C4, 5 => C5, 6 => C6,
            7 => C7, 8 => C8, 9 => C9, 10 => C10, 11 => C11, 12 => C12, 13 => C13,
            14 => C14, 15 => C15, 16 => C16, 17 => C17, 18 => C18, 19 => C19,
            20 => C20, 21 => C21, 22 => C22, 23 => C23, 24 => C24, 25 => C25,
            26 => C26, 27 => C27, 28 => C28, 29 => C29, 30 => C30, 31 => C31,
            32 => C32, 33 => C33, 34 => C34, 35 => C35, 36 => C36, 37 => C37,
            38 => C38, 39 => C39, 40 => C40, 41 => C41, 42 => C42, 43 => C43,
            44 => C44, 45 => C45, 46 => C46, 47 => C47, 48 => C48, 49 => C49,
            50 => C50, 51 => C51, 52 => C52, 53 => C53, 54 => C54, 55 => C55,
            56 => C56, 57 => C57, 58 => C58, 59 => C59, 60 => C60, 61 => C61,
            62 => C62, 63 => C63, 64 => C64, 65 => C65, 66 => C66, 67 => C67,
            68 => C68, 69 => C69, 70 => C70, 71 => C71, 72 => C72, 73 => C73,
            74 => C74, 75 => C75, 76 => C76, 77 => C77, 78 => C78, 79 => C79,
            80 => C80, 81 => C81, 82 => C82, 83 => C83, 84 => C84, 85 => C85,
            86 => C86, 87 => C87, 88 => C88, 89 => C89, 90 => C90, 91 => C91,
            92 => C92, 93 => C93, 94 => C94, 95 => C95, 96 => C96, 97 => C97,
            98 => C98, 99 => C99, 100 => C100, 101 => C101, 102 => C102,
            103 => C103, 104 => C104, 105 => C105, _  => C106,
        }
    }
}

// Simple numerical cast from enum to u8
impl Into<u8> for Pattern {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Encoding for Pattern {
    #[inline]
    fn stop() -> Self {Pattern::C106}

    #[inline]
    fn switch(s: Symbology) -> Self {
        match s {
            Symbology::A => Pattern::C101,
            Symbology::B => Pattern::C100,
            Symbology::C => Pattern::C99,
        }
    }

    #[inline]
    fn start(s: Symbology) -> Self {
        match s {
            Symbology::A => Pattern::C103,
            Symbology::B => Pattern::C104,
            Symbology::C => Pattern::C105,
        }
    }

    #[inline]
    fn shift() -> Self {Pattern::C98}

    #[inline]
    fn fnc1() -> Self {Pattern::C102}

    #[inline]
    fn fnc2() -> Self {Pattern::C97}

    #[inline]
    fn fnc3() -> Self {Pattern::C96}

    #[inline]
    fn fnc4(s: Symbology) -> Option<Self> {
        match s {
            Symbology::A => Some(Pattern::C101),
            Symbology::B => Some(Pattern::C100),
            _ => None,
        }
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        *self as u8
    }
}

impl Encoding for u8 {

    #[inline]
    fn stop() -> Self {106}

    #[inline]
    fn switch(s: Symbology) -> Self {
        match s {
            Symbology::A => 101,
            Symbology::B => 100,
            Symbology::C => 99,
        }
    }

    #[inline]
    fn start(s: Symbology) -> Self {
        match s {
            Symbology::A => 103,
            Symbology::B => 104,
            Symbology::C => 105,
        }
    }

    #[inline]
    fn shift() -> Self {98}

    #[inline]
    fn fnc1() -> Self {102}

    #[inline]
    fn fnc2() -> Self {97}

    #[inline]
    fn fnc3() -> Self {96}

    #[inline]
    fn fnc4(s: Symbology) -> Option<Self> {
        match s {
            Symbology::A => Some(101),
            Symbology::B => Some(100),
            _ => None,
        }
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        *self
    }
}

#[cfg(test)]
mod test {
    quickcheck! {
        fn pattern_from_u8_to_u8(p: u8) -> bool {
            use super::Encoding;
            p == super::Pattern::from(p).as_u8()
        }
    }
}
