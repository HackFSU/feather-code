//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![deny(missing_docs)]

//! # Feather Code
//!
//! Custom visual encoding format extended from code128 barcode encoding.

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod barcode;
