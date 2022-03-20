/*
 * Copyright Stalwart Labs, Minter Ltd. See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::io::{self, Write};

use super::Header;

/// RFC5322 Date header
pub struct Date {
    pub date: i64,
}

impl Date {
    /// Create a new Date header from a `chrono` timestamp.
    pub fn new(date: i64) -> Self {
        Self { date }
    }
}

impl Header for Date {
    fn write_header(&self, mut output: impl Write, _bytes_written: usize) -> io::Result<usize> {
        output.write_all(b"\r\n")?;
        Ok(0)
    }
}
