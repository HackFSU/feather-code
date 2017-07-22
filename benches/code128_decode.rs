#![feature(test)]

extern crate feather_code as fc;
extern crate test;

use fc::barcode::code128::Code128;
use fc::barcode::format::Decode;
use fc::barcode::code128::encodings;
use test::Bencher;

#[bench]
fn decode_u8(bencher: &mut Bencher) {
    let buffer = [105, 102, 42, 18, 40, 20, 50, 101, 16, 92, 106];
    let code = Code128(buffer.as_ref());

    bencher.iter(|| {
        code.decode().unwrap();
    })
}

#[bench]
fn decode_vec(bencher: &mut Bencher) {
    let buffer = vec![105, 102, 42, 18, 40, 20, 50, 101, 16, 92, 106];
    let code = Code128(buffer.as_ref());

    bencher.iter(|| {
        code.decode().unwrap();
    })
}

#[bench]
fn decode_pattern(bencher: &mut Bencher) {
    use encodings::Pattern::*;
    let buffer = [C105, C102, C42, C18, C40, C20, C50, C101, C16, C92, C106];
    let code = Code128(buffer.as_ref());

    bencher.iter(|| {
        code.decode().unwrap();
    })
}
