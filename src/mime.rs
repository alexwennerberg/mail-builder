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

use std::{
    borrow::Cow,
    collections::{btree_map::Entry, hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    io::{self, Write},
    iter::FromIterator,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    encoders::{
        base64::base64_encode,
        encode::{get_encoding_type, EncodingType},
        quoted_printable::quoted_printable_encode,
    },
    headers::{
        content_type::ContentType, message_id::MessageId, raw::Raw, text::Text, Header, HeaderType,
    },
};

/// MIME part of an e-mail.
pub struct MimePart<'x> {
    pub headers: BTreeMap<Cow<'x, str>, HeaderType<'x>>,
    pub contents: BodyPart<'x>,
}

pub enum BodyPart<'x> {
    Text(Cow<'x, str>),
    Binary(Cow<'x, [u8]>),
    Multipart(Vec<MimePart<'x>>),
}

impl<'x> From<&'x str> for BodyPart<'x> {
    fn from(value: &'x str) -> Self {
        BodyPart::Text(value.into())
    }
}

impl<'x> From<&'x [u8]> for BodyPart<'x> {
    fn from(value: &'x [u8]) -> Self {
        BodyPart::Binary(value.into())
    }
}

impl<'x> From<String> for BodyPart<'x> {
    fn from(value: String) -> Self {
        BodyPart::Text(value.into())
    }
}

impl<'x> From<Vec<u8>> for BodyPart<'x> {
    fn from(value: Vec<u8>) -> Self {
        BodyPart::Binary(value.into())
    }
}

pub fn make_boundary() -> String {
    // TODO
    String::new()
}

impl<'x> MimePart<'x> {
    /// Create a custom MIME part.
    pub fn new(content_type: ContentType<'x>, contents: BodyPart<'x>) -> Self {
        Self {
            contents,
            headers: BTreeMap::from_iter(vec![("Content-Type".into(), content_type.into())]),
        }
    }

    /// Create a new multipart/* MIME part.
    pub fn new_multipart(
        content_type: impl Into<Cow<'x, str>>,
        contents: Vec<MimePart<'x>>,
    ) -> Self {
        Self {
            contents: BodyPart::Multipart(contents),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new(content_type).into(),
            )]),
        }
    }
    pub fn new_text(contents: impl Into<Cow<'x, str>>) -> Self {
        Self {
            contents: BodyPart::Text(contents.into()),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new("text/plain")
                    .attribute("charset", "utf-8")
                    .into(),
            )]),
        }
    }

    /// Create a new text/plain MIME part with format=flowed
    pub fn new_text_flowed(contents: impl Into<Cow<'x, str>>) -> Self {
        Self {
            contents: BodyPart::Text(contents.into()),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new("text/plain")
                    .attribute("charset", "utf-8")
                    .attribute("format", "flowed")
                    .into(),
            )]),
        }
    }

    /// Create a new text/* MIME part.
    pub fn new_text_other(
        content_type: impl Into<Cow<'x, str>>,
        contents: impl Into<Cow<'x, str>>,
    ) -> Self {
        Self {
            contents: BodyPart::Text(contents.into()),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new(content_type)
                    .attribute("charset", "utf-8")
                    .into(),
            )]),
        }
    }

    /// Create a new text/html MIME part.
    pub fn new_html(contents: impl Into<Cow<'x, str>>) -> Self {
        Self {
            contents: BodyPart::Text(contents.into()),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new("text/html")
                    .attribute("charset", "utf-8")
                    .into(),
            )]),
        }
    }

    /// Create a new binary MIME part.
    pub fn new_binary(c_type: impl Into<Cow<'x, str>>, contents: impl Into<Cow<'x, [u8]>>) -> Self {
        Self {
            contents: BodyPart::Binary(contents.into()),
            headers: BTreeMap::from_iter(vec![(
                "Content-Type".into(),
                ContentType::new(c_type).into(),
            )]),
        }
    }

    /// Set the attachment filename of a MIME part.
    pub fn attachment(mut self, filename: impl Into<Cow<'x, str>>) -> Self {
        self.headers.insert(
            "Content-Disposition".into(),
            ContentType::new("attachment")
                .attribute("filename", filename)
                .into(),
        );
        self
    }

    /// Set the MIME part as inline.
    pub fn inline(mut self) -> Self {
        self.headers.insert(
            "Content-Disposition".into(),
            ContentType::new("inline").into(),
        );
        self
    }

    /// Set the Content-Language header of a MIME part.
    pub fn language(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers
            .insert("Content-Language".into(), Text::new(value).into());
        self
    }

    /// Set the Content-ID header of a MIME part.
    pub fn cid(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers
            .insert("Content-ID".into(), MessageId::new(value).into());
        self
    }

    /// Set the Content-Location header of a MIME part.
    pub fn location(mut self, value: impl Into<Cow<'x, str>>) -> Self {
        self.headers
            .insert("Content-Location".into(), Raw::new(value).into());
        self
    }

    /// Set custom headers of a MIME part.
    pub fn header(
        mut self,
        header: impl Into<Cow<'x, str>>,
        value: impl Into<HeaderType<'x>>,
    ) -> Self {
        self.headers.insert(header.into(), value.into());
        self
    }

    /// Add a body part to a multipart/* MIME part.
    pub fn add_part(&mut self, part: MimePart<'x>) {
        if let BodyPart::Multipart(ref mut parts) = self.contents {
            parts.push(part);
        }
    }

    /// Write the MIME part to a writer.
    pub fn write_part(self, mut output: impl Write) -> io::Result<usize> {
        let mut stack = Vec::new();
        let mut it = vec![self].into_iter();
        let mut boundary: Option<Cow<str>> = None;

        loop {
            while let Some(mut part) = it.next() {
                if let Some(boundary) = boundary.as_ref() {
                    output.write_all(b"\r\n--")?;
                    output.write_all(boundary.as_bytes())?;
                    output.write_all(b"\r\n")?;
                }
                match part.contents {
                    BodyPart::Text(text) => {
                        let mut is_attachment = false;
                        for (header_name, header_value) in &part.headers {
                            output.write_all(header_name.as_bytes())?;
                            output.write_all(b": ")?;
                            if !is_attachment && header_name == "Content-Disposition" {
                                is_attachment = header_value
                                    .as_content_type()
                                    .map(|v| v.is_attachment())
                                    .unwrap_or(false);
                            }
                            header_value.write_header(&mut output, header_name.len() + 2)?;
                        }
                        detect_encoding(text.as_bytes(), &mut output, !is_attachment)?;
                    }
                    BodyPart::Binary(binary) => {
                        let mut is_text = false;
                        let mut is_attachment = false;
                        for (header_name, header_value) in &part.headers {
                            output.write_all(header_name.as_bytes())?;
                            output.write_all(b": ")?;
                            if !is_text && header_name == "Content-Type" {
                                is_text = header_value
                                    .as_content_type()
                                    .map(|v| v.is_text())
                                    .unwrap_or(false);
                            } else if !is_attachment && header_name == "Content-Disposition" {
                                is_attachment = header_value
                                    .as_content_type()
                                    .map(|v| v.is_attachment())
                                    .unwrap_or(false);
                            }
                            header_value.write_header(&mut output, header_name.len() + 2)?;
                        }
                        if !is_text {
                            output.write_all(b"Content-Transfer-Encoding: base64\r\n\r\n")?;
                            base64_encode(binary.as_ref(), &mut output, false)?;
                        } else {
                            detect_encoding(binary.as_ref(), &mut output, !is_attachment)?;
                        }
                    }
                    BodyPart::Multipart(parts) => {
                        if boundary.is_some() {
                            stack.push((it, boundary));
                        }

                        output.write_all(b"Content-Type: ")?;
                        boundary = if let Some(value) = part.headers.remove("Content-Type") {
                            match value {
                                HeaderType::ContentType(mut ct) => {
                                    if let Entry::Vacant(entry) =
                                        ct.attributes.entry("boundary".into())
                                    {
                                        entry.insert(make_boundary().into());
                                    }
                                    ct.write_header(&mut output, 14)?;
                                    ct.attributes.remove("boundary")
                                }
                                HeaderType::Raw(raw) => {
                                    if let Some(pos) = raw.raw.find("boundary=\"") {
                                        if let Some(boundary) = raw.raw[pos..].split('"').nth(1) {
                                            Some(boundary.to_string().into())
                                        } else {
                                            Some(make_boundary().into())
                                        }
                                    } else {
                                        let boundary = make_boundary();
                                        output.write_all(raw.raw.as_bytes())?;
                                        output.write_all(b"; boundary=\"")?;
                                        output.write_all(boundary.as_bytes())?;
                                        output.write_all(b"\"\r\n")?;
                                        Some(boundary.into())
                                    }
                                }
                                _ => panic!("Unsupported Content-Type header value."),
                            }
                        } else {
                            let boundary = make_boundary();
                            ContentType::new("multipart/mixed")
                                .attribute("boundary", &boundary)
                                .write_header(&mut output, 14)?;
                            Some(boundary.into())
                        };

                        for (header_name, header_value) in part.headers {
                            output.write_all(header_name.as_bytes())?;
                            output.write_all(b": ")?;
                            header_value.write_header(&mut output, header_name.len() + 2)?;
                        }
                        output.write_all(b"\r\n")?;
                        it = parts.into_iter();
                    }
                }
            }
            if let Some(boundary) = boundary {
                output.write_all(b"\r\n--")?;
                output.write_all(boundary.as_bytes())?;
                output.write_all(b"--\r\n")?;
            }
            if let Some((prev_it, prev_boundary)) = stack.pop() {
                it = prev_it;
                boundary = prev_boundary;
            } else {
                break;
            }
        }
        Ok(0)
    }
}

fn detect_encoding(input: &[u8], mut output: impl Write, is_body: bool) -> io::Result<()> {
    match get_encoding_type(input, false, is_body) {
        EncodingType::Base64 => {
            output.write_all(b"Content-Transfer-Encoding: base64\r\n\r\n")?;
            base64_encode(input, &mut output, false)?;
        }
        EncodingType::QuotedPrintable(_) => {
            output.write_all(b"Content-Transfer-Encoding: quoted-printable\r\n\r\n")?;
            quoted_printable_encode(input, &mut output, false, is_body)?;
        }
        EncodingType::None => {
            output.write_all(b"Content-Transfer-Encoding: 7bit\r\n\r\n")?;
            if is_body {
                let mut prev_ch = 0;
                for ch in input {
                    if *ch == b'\n' && prev_ch != b'\r' {
                        output.write_all(b"\r")?;
                    }
                    output.write_all(&[*ch])?;
                    prev_ch = *ch;
                }
            } else {
                output.write_all(input)?;
            }
        }
    }
    Ok(())
}
