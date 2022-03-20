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

//! # mail-builder
//!
//! [![crates.io](https://img.shields.io/crates/v/mail-builder)](https://crates.io/crates/mail-builder)
//! [![build](https://github.com/stalwartlabs/mail-builder/actions/workflows/rust.yml/badge.svg)](https://github.com/stalwartlabs/mail-builder/actions/workflows/rust.yml)
//! [![docs.rs](https://img.shields.io/docsrs/mail-builder)](https://docs.rs/mail-builder)
//! [![crates.io](https://img.shields.io/crates/l/mail-builder)](http://www.apache.org/licenses/LICENSE-2.0)
//!
//! _mail-builder_ is a flexible **e-mail builder library** written in Rust that generates RFC5322 compliant e-mail messages.
//! The library has full MIME support and automatically selects the most optimal encoding for each message body part.
//!
//! Building e-mail messages is straightforward:
//!
//! ```rust
//!     use mail_builder::MessageBuilder;
//!
//!     // Build a simple text message with a single attachment
//!     let mut message = MessageBuilder::new();
//!     message.from(("John Doe", "john@doe.com"));
//!     message.to("jane@doe.com");
//!     message.subject("Hello, world!");
//!     message.text_body("Message contents go here.");
//!     message.binary_attachment("image/png", "image.png", [1, 2, 3, 4].as_ref());
//!
//!     // Write message to memory
//!     let mut output = Vec::new();
//!     message.write_to(&mut output).unwrap();
//! ```
//!
//! More complex messages with grouped addresses, inline parts and
//! multipart/alternative sections can also be easily built:
//!
//! ```rust
//!     use mail_builder::{headers::url::URL, MessageBuilder};
//!     use std::fs::File;
//!
//!     // Build a multipart message with text and HTML bodies,
//!     // inline parts and attachments.
//!     let mut message = MessageBuilder::new();
//!     message.from(("John Doe", "john@doe.com"));
//!
//!     // To recipients
//!     message.to(vec![
//!         ("Antoine de Saint-Exupéry", "antoine@exupery.com"),
//!         ("안녕하세요 세계", "test@test.com"),
//!         ("Xin chào", "addr@addr.com"),
//!     ]);
//!
//!     // BCC recipients using grouped addresses
//!     message.bcc(vec![
//!         (
//!             "My Group",
//!             vec![
//!                 ("ASCII name", "addr1@addr7.com"),
//!                 ("ハロー・ワールド", "addr2@addr6.com"),
//!                 ("áéíóú", "addr3@addr5.com"),
//!                 ("Γειά σου Κόσμε", "addr4@addr4.com"),
//!             ],
//!         ),
//!         (
//!             "Another Group",
//!             vec![
//!                 ("שלום עולם", "addr5@addr3.com"),
//!                 ("ñandú come ñoquis", "addr6@addr2.com"),
//!                 ("Recipient", "addr7@addr1.com"),
//!             ],
//!         ),
//!     ]);
//!
//!     // Set RFC and custom headers
//!     message.subject("Testing multipart messages");
//!     message.in_reply_to(vec!["message-id-1", "message-id-2"]);
//!     message.header("List-Archive", URL::new("http://example.com/archive"));
//!
//!     // Set HTML and plain text bodies
//!     message.text_body("This is the text body!\n");
//!     message.html_body("<p>HTML body with <img src=\"cid:my-image\"/>!</p>");
//!
//!     // Include an embedded image as an inline part
//!     message.binary_inline("image/png", "cid:my-image", [0, 1, 2, 3, 4, 5].as_ref());
//!
//!     // Add a text and a binary attachment
//!     message.text_attachment("text/plain", "my fíle.txt", "Attachment contents go here.");
//!     message.binary_attachment(
//!         "text/plain",
//!         "ハロー・ワールド",
//!         b"Binary contents go here.".as_ref(),
//!     );
//!
//!     // Write the message to a file
//!     message
//!         .write_to(File::create("message.eml").unwrap())
//!         .unwrap();
//! ```
//!
//! Nested MIME body structures can be created using the `body` method:
//!
//! ```rust
//!     use mail_builder::{headers::address::Address, mime::MimePart, MessageBuilder};
//!     use std::fs::File;
//!
//!     // Build a nested multipart message
//!     let mut message = MessageBuilder::new();
//!
//!     message.from(Address::new_address("John Doe".into(), "john@doe.com"));
//!     message.to(Address::new_address("Jane Doe".into(), "jane@doe.com"));
//!     message.subject("Nested multipart message");
//!
//!     // Define the nested MIME body structure
//!     message.body(MimePart::new_multipart(
//!         "multipart/mixed",
//!         vec![
//!             MimePart::new_text("Part A contents go here...").inline(),
//!             MimePart::new_multipart(
//!                 "multipart/mixed",
//!                 vec![
//!                     MimePart::new_multipart(
//!                         "multipart/alternative",
//!                         vec![
//!                             MimePart::new_multipart(
//!                                 "multipart/mixed",
//!                                 vec![
//!                                     MimePart::new_text("Part B contents go here...").inline(),
//!                                     MimePart::new_binary(
//!                                         "image/jpeg",
//!                                         "Part C contents go here...".as_bytes(),
//!                                     )
//!                                     .inline(),
//!                                     MimePart::new_text("Part D contents go here...").inline(),
//!                                 ],
//!                             ),
//!                             MimePart::new_multipart(
//!                                 "multipart/related",
//!                                 vec![
//!                                     MimePart::new_html("Part E contents go here...").inline(),
//!                                     MimePart::new_binary(
//!                                         "image/jpeg",
//!                                         "Part F contents go here...".as_bytes(),
//!                                     ),
//!                                 ],
//!                             ),
//!                         ],
//!                     ),
//!                     MimePart::new_binary("image/jpeg", "Part G contents go here...".as_bytes())
//!                         .attachment("image_G.jpg"),
//!                     MimePart::new_binary(
//!                         "application/x-excel",
//!                         "Part H contents go here...".as_bytes(),
//!                     ),
//!                     MimePart::new_binary(
//!                         "x-message/rfc822",
//!                         "Part J contents go here...".as_bytes(),
//!                     ),
//!                 ],
//!             ),
//!             MimePart::new_text("Part K contents go here...").inline(),
//!         ],
//!     ));
//!
//!     // Write the message to a file
//!     message
//!         .write_to(File::create("nested-message.eml").unwrap())
//!         .unwrap();
//! ```
//!
//! Please note that this library does not support parsing e-mail messages as this functionality is provided separately by the [`mail-parser`](https://crates.io/crates/mail-parser) crate.
//!
//!
//! ## Testing
//!
//! To run the testsuite:
//!
//! ```bash
//!  $ cargo test --all-features
//! ```
//!
//! or, to run the testsuite with MIRI:
//!
//! ```bash
//!  $ cargo +nightly miri test --all-features
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ## Copyright
//!
//! Copyright (C) 2020-2022, Stalwart Labs, Minter Ltd.
//!
//! See [COPYING] for the license.
//!
//! [COPYING]: https://github.com/stalwartlabs/mail-builder/blob/main/COPYING
//!
#[forbid(unsafe_code)]
pub mod encoders;
pub mod headers;
pub mod mime;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    io::{self, Write},
};

use chrono::Local;
use headers::{
    address::Address, date::Date, message_id::MessageId, text::Text, Header, HeaderType,
};
use mime::{make_boundary, MimePart};

/// Builds a RFC5322 compliant MIME email message.
pub struct MessageBuilder<'x> {
    pub headers: BTreeMap<Cow<'x, str>, Vec<HeaderType<'x>>>,
    pub html_body: Option<MimePart<'x>>,
    pub text_body: Option<MimePart<'x>>,
    pub attachments: Option<Vec<MimePart<'x>>>,
    pub body: Option<MimePart<'x>>,
    pub flowed: bool,
}

impl<'x> Default for MessageBuilder<'x> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'x> MessageBuilder<'x> {
    /// Create a new MessageBuilder.
    pub fn new() -> Self {
        MessageBuilder {
            headers: BTreeMap::new(),
            html_body: None,
            text_body: None,
            attachments: None,
            body: None,
            flowed: false,
        }
    }

    /// Set the Message-ID header. If no Message-ID header is set, one will be
    /// generated automatically.
    pub fn message_id(&mut self, value: impl Into<MessageId<'x>>) {
        self.header("Message-ID", value.into());
    }

    /// Set the In-Reply-To header.
    pub fn in_reply_to(&mut self, value: impl Into<MessageId<'x>>) {
        self.header("In-Reply-To", value.into());
    }

    /// Set the References header.
    pub fn references(&mut self, value: impl Into<MessageId<'x>>) {
        self.header("References", value.into());
    }

    /// Set the Sender header.
    pub fn sender(&mut self, value: impl Into<Address<'x>>) {
        self.header("Sender", value.into());
    }

    /// Set the From header.
    pub fn from(&mut self, value: impl Into<Address<'x>>) {
        self.header("From", value.into());
    }

    /// Set the To header.
    pub fn to(&mut self, value: impl Into<Address<'x>>) {
        self.header("To", value.into());
    }

    /// Set the Cc header.
    pub fn cc(&mut self, value: impl Into<Address<'x>>) {
        self.header("Cc", value.into());
    }

    /// Set the Bcc header.
    pub fn bcc(&mut self, value: impl Into<Address<'x>>) {
        self.header("Bcc", value.into());
    }

    /// Set the Reply-To header.
    pub fn reply_to(&mut self, value: impl Into<Address<'x>>) {
        self.header("Reply-To", value.into());
    }

    /// Set the Subject header.
    pub fn subject(&mut self, value: impl Into<Text<'x>>) {
        self.header("Subject", value.into());
    }

    /// Set the Date header. If no Date header is set, one will be generated
    /// automatically.
    pub fn date(&mut self, value: impl Into<Date>) {
        self.header("Date", value.into());
    }

    /// Add a custom header.
    pub fn header(&mut self, header: impl Into<Cow<'x, str>>, value: impl Into<HeaderType<'x>>) {
        self.headers
            .entry(header.into())
            .or_insert_with(Vec::new)
            .push(value.into());
    }

    /// Mark body as format=flowed
    pub fn format_flowed(&mut self) {
        self.flowed = true
    }

    /// Set the plain text body of the message. Note that only one plain text body
    /// per message can be set using this function.
    /// To build more complex MIME body structures, use the `body` method instead.
    pub fn text_body(&mut self, value: impl Into<Cow<'x, str>>) {
        if self.flowed {
            self.text_body = Some(MimePart::new_text_flowed(value));
        } else {
            self.text_body = Some(MimePart::new_text(value));
        }
    }

    /// Set the HTML body of the message. Note that only one HTML body
    /// per message can be set using this function.
    /// To build more complex MIME body structures, use the `body` method instead.
    pub fn html_body(&mut self, value: impl Into<Cow<'x, str>>) {
        self.html_body = Some(MimePart::new_html(value));
    }

    /// Add a binary attachment to the message.
    pub fn binary_attachment(
        &mut self,
        content_type: impl Into<Cow<'x, str>>,
        filename: impl Into<Cow<'x, str>>,
        value: impl Into<Cow<'x, [u8]>>,
    ) {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(MimePart::new_binary(content_type, value).attachment(filename));
    }

    /// Add a text attachment to the message.
    pub fn text_attachment(
        &mut self,
        content_type: impl Into<Cow<'x, str>>,
        filename: impl Into<Cow<'x, str>>,
        value: impl Into<Cow<'x, str>>,
    ) {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(MimePart::new_text_other(content_type, value).attachment(filename));
    }

    /// Add an inline binary to the message.
    pub fn binary_inline(
        &mut self,
        content_type: impl Into<Cow<'x, str>>,
        cid: impl Into<Cow<'x, str>>,
        value: impl Into<Cow<'x, [u8]>>,
    ) {
        self.attachments
            .get_or_insert_with(Vec::new)
            .push(MimePart::new_binary(content_type, value).inline().cid(cid));
    }

    /// Set a custom MIME body structure.
    pub fn body(&mut self, value: MimePart<'x>) {
        self.body = Some(value);
    }

    /// Build the message.
    pub fn write_to(self, mut output: impl Write) -> io::Result<()> {
        let mut has_date = false;
        let mut has_message_id = false;

        for (header_name, header_values) in &self.headers {
            if !has_date && header_name == "Date" {
                has_date = true;
            } else if !has_message_id && header_name == "Message-ID" {
                has_message_id = true;
            }

            for header_value in header_values {
                output.write_all(header_name.as_bytes())?;
                output.write_all(b": ")?;
                header_value.write_header(&mut output, header_name.len() + 2)?;
            }
        }

        if !has_message_id {
            output.write_all(b"Message-ID: <")?;
            output.write_all(make_boundary().as_bytes())?;
            output.write_all(b">\r\n")?;
        }

        if !has_date {
            output.write_all(b"Date: ")?;
            output.write_all(Local::now().to_rfc2822().as_bytes())?;
            output.write_all(b"\r\n")?;
        }

        (if let Some(body) = self.body {
            body
        } else {
            match (self.text_body, self.html_body, self.attachments) {
                (Some(text), Some(html), Some(attachments)) => {
                    let mut parts = Vec::with_capacity(attachments.len() + 1);
                    parts.push(MimePart::new_multipart(
                        "multipart/alternative",
                        vec![text, html],
                    ));
                    parts.extend(attachments);

                    MimePart::new_multipart("multipart/mixed", parts)
                }
                (Some(text), Some(html), None) => {
                    MimePart::new_multipart("multipart/alternative", vec![text, html])
                }
                (Some(text), None, Some(attachments)) => {
                    let mut parts = Vec::with_capacity(attachments.len() + 1);
                    parts.push(text);
                    parts.extend(attachments);
                    MimePart::new_multipart("multipart/mixed", parts)
                }
                (Some(text), None, None) => text,
                (None, Some(html), Some(attachments)) => {
                    let mut parts = Vec::with_capacity(attachments.len() + 1);
                    parts.push(html);
                    parts.extend(attachments);
                    MimePart::new_multipart("multipart/mixed", parts)
                }
                (None, Some(html), None) => html,
                (None, None, Some(attachments)) => {
                    MimePart::new_multipart("multipart/mixed", attachments)
                }
                (None, None, None) => MimePart::new_text("\n"),
            }
        })
        .write_part(output)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use mail_parser::Message;

    use crate::{
        headers::{address::Address, url::URL},
        mime::MimePart,
        MessageBuilder,
    };

    #[test]
    fn build_nested_message() {
        let mut message = MessageBuilder::new();

        message.from(Address::new_address("John Doe".into(), "john@doe.com"));
        message.to(Address::new_address("Jane Doe".into(), "jane@doe.com"));
        message.subject("RFC 8621 Section 4.1.4 test");

        message.body(MimePart::new_multipart(
            "multipart/mixed",
            vec![
                MimePart::new_text("Part A contents go here...").inline(),
                MimePart::new_multipart(
                    "multipart/mixed",
                    vec![
                        MimePart::new_multipart(
                            "multipart/alternative",
                            vec![
                                MimePart::new_multipart(
                                    "multipart/mixed",
                                    vec![
                                        MimePart::new_text("Part B contents go here...").inline(),
                                        MimePart::new_binary(
                                            "image/jpeg",
                                            "Part C contents go here...".as_bytes(),
                                        )
                                        .inline(),
                                        MimePart::new_text("Part D contents go here...").inline(),
                                    ],
                                ),
                                MimePart::new_multipart(
                                    "multipart/related",
                                    vec![
                                        MimePart::new_html("Part E contents go here...").inline(),
                                        MimePart::new_binary(
                                            "image/jpeg",
                                            "Part F contents go here...".as_bytes(),
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        MimePart::new_binary("image/jpeg", "Part G contents go here...".as_bytes())
                            .attachment("image_G.jpg"),
                        MimePart::new_binary(
                            "application/x-excel",
                            "Part H contents go here...".as_bytes(),
                        ),
                        MimePart::new_binary(
                            "x-message/rfc822",
                            "Part J contents go here...".as_bytes(),
                        ),
                    ],
                ),
                MimePart::new_text("Part K contents go here...").inline(),
            ],
        ));

        let mut output = Vec::new();
        message.write_to(&mut output).unwrap();
        Message::parse(&output).unwrap();
        //fs::write("test.yaml", &serde_yaml::to_string(&message).unwrap()).unwrap();
    }

    #[test]
    fn build_message() {
        let mut message = MessageBuilder::new();
        message.from(("John Doe", "john@doe.com"));
        message.to(vec![
            ("Antoine de Saint-Exupéry", "antoine@exupery.com"),
            ("안녕하세요 세계", "test@test.com"),
            ("Xin chào", "addr@addr.com"),
        ]);
        message.bcc(vec![
            (
                "Привет, мир",
                vec![
                    ("ASCII recipient", "addr1@addr7.com"),
                    ("ハロー・ワールド", "addr2@addr6.com"),
                    ("áéíóú", "addr3@addr5.com"),
                    ("Γειά σου Κόσμε", "addr4@addr4.com"),
                ],
            ),
            (
                "Hello world",
                vec![
                    ("שלום עולם", "addr5@addr3.com"),
                    ("¡El ñandú comió ñoquis!", "addr6@addr2.com"),
                    ("Recipient", "addr7@addr1.com"),
                ],
            ),
        ]);
        message.header("List-Archive", URL::new("http://example.com/archive"));
        message.subject("Hello world!");

        message.text_body("Hello, world!\n".repeat(20));
        message.html_body("<p>¡Hola Mundo!</p>".repeat(20));
        message.binary_inline("image/png", "cid:image", [0, 1, 2, 3, 4, 5].as_ref());
        message.text_attachment("text/plain", "my fíle.txt", "안녕하세요 세계".repeat(20));
        message.binary_attachment(
            "text/plain",
            "ハロー・ワールド",
            "ハロー・ワールド".repeat(20).into_bytes(),
        );

        let mut output = Vec::new();
        message.write_to(&mut output).unwrap();
        Message::parse(&output).unwrap();
    }
}
