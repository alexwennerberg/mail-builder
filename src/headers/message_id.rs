use std::borrow::Cow;

use super::Header;

pub struct MessageId<'x> {
    pub id: Vec<Cow<'x, str>>,
}

impl<'x> Header for MessageId<'x> {
    fn write_header(
        &self,
        mut output: impl std::io::Write,
        mut bytes_written: usize,
    ) -> std::io::Result<usize> {
        for (pos, id) in self.id.iter().enumerate() {
            output.write_all(b"<")?;
            output.write_all(id.as_ref().as_bytes())?;
            output.write_all(b">")?;
            bytes_written += id.as_ref().len() + 2;
            if bytes_written >= 76 && pos < self.id.len() - 1 {
                output.write_all(b"\r\n\t")?;
                bytes_written = 0;
            }
        }
        if bytes_written > 0 {
            output.write_all(b"\r\n")?;
        }
        Ok(0)
    }
}
