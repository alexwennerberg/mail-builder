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

use std::fs::File;

use mail_builder::{headers::url::URL, MessageBuilder};

fn main() {
    // Build a multipart message with text and HTML bodies,
    // inline parts and attachments.
    let mut message = MessageBuilder::new();
    message.from(("John Doe", "john@doe.com").into());

    // To recipients
    message.to(vec![
        ("Antoine de Saint-Exupéry", "antoine@exupery.com").into(),
        ("안녕하세요 세계", "test@test.com").into(),
        ("Xin chào", "addr@addr.com").into(),
    ]
    .into());

    // BCC recipients using grouped addresses
    message.bcc(
        vec![
            (
                "My Group",
                vec![
                    ("ASCII name", "addr1@addr7.com").into(),
                    ("ハロー・ワールド", "addr2@addr6.com").into(),
                    ("áéíóú", "addr3@addr5.com").into(),
                    ("Γειά σου Κόσμε", "addr4@addr4.com").into(),
                ],
            )
                .into(),
            (
                "Another Group",
                vec![
                    ("שלום עולם", "addr5@addr3.com").into(),
                    ("ñandú come ñoquis", "addr6@addr2.com").into(),
                    "addr7@addr1.com".into(),
                ],
            )
                .into(),
        ]
        .into(),
    );

    // Set RFC and custom headers
    message.subject("Testing multipart messages".into());
    message.in_reply_to(vec!["message-id-1", "message-id-2"].into());
    message.header(
        "List-Archive",
        URL::new("http://example.com/archive").into(),
    );

    // Set HTML and plain text bodies
    message.text_body("This is the text body!\n");
    message.html_body("<p>HTML body with <img src=\"my-image\"/>!</p>");

    // Include an embedded image as an inline part
    message.binary_inline("image/png", "my-image", &[0, 1, 2, 3, 4, 5]);

    // Add a text and a binary attachment
    message.text_attachment("text/plain", "my fíle.txt", "Attachment contents go here.");
    message.binary_attachment(
        "text/plain",
        "ハロー・ワールド",
        b"Binary contents go here.",
    );

    // Write the message to a file
    message
        .write_to(File::create("message.eml").unwrap())
        .unwrap();
}