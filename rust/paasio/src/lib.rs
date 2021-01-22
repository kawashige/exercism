use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    count: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            reader: wrapped,
            count: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.reader.read(buf);
        if let Ok(b) = bytes {
            self.count += 1;
            self.bytes += b;
        }
        bytes
    }
}

pub struct WriteStats<W> {
    writer: W,
    count: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            writer: wrapped,
            count: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.writer.write(buf);
        if let Ok(b) = bytes {
            self.count += 1;
            self.bytes += b;
        }
        bytes
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
