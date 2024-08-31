use std::io::Write;

use sha1::{digest::Update, Sha1};

/// a wrapper over a writer but computing the hash while writing
pub(crate) struct HashWriter<W> {
    pub(crate) writer: W,
    pub(crate) hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
