//! Compile-time CBOR serialization helpers.

/// Helper structure for compile-time CBOR serialization.
pub struct ConstCborWriter<const N: usize> {
    /// Internal buffer containing serialized bytes.
    pub buf: [u8; N],
    /// Current write length.
    pub len: usize,
}

impl<const N: usize> ConstCborWriter<N> {
    /// Creates a new ConstCborWriter.
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    /// Writes a map header with a given number of fields.
    pub const fn write_map_header(mut self, size: u8) -> Self {
        self.buf[self.len] = 0xa0 + size;
        self.len += 1;
        self
    }

    /// Writes an array header with a given number of elements.
    pub const fn write_array_header(mut self, size: u8) -> Self {
        self.buf[self.len] = 0x80 + size;
        self.len += 1;
        self
    }

    /// Writes a u32 key.
    pub const fn write_key(self, key: u32) -> Self {
        self.write_u32(key)
    }

    /// Writes a u32 value in CBOR format.
    pub const fn write_u32(mut self, val: u32) -> Self {
        if val <= 23 {
            self.buf[self.len] = val as u8;
            self.len += 1;
        } else if val <= 255 {
            self.buf[self.len] = 0x18;
            self.buf[self.len + 1] = val as u8;
            self.len += 2;
        } else if val <= 65535 {
            self.buf[self.len] = 0x19;
            self.buf[self.len + 1] = (val >> 8) as u8;
            self.buf[self.len + 2] = val as u8;
            self.len += 3;
        } else {
            self.buf[self.len] = 0x1a;
            self.buf[self.len + 1] = (val >> 24) as u8;
            self.buf[self.len + 2] = (val >> 16) as u8;
            self.buf[self.len + 3] = (val >> 8) as u8;
            self.buf[self.len + 4] = val as u8;
            self.len += 5;
        }
        self
    }

    /// Writes a string slice as a CBOR string (major type 3).
    pub const fn write_str(mut self, s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if len <= 23 {
            self.buf[self.len] = 0x60 + len as u8;
            self.len += 1;
        } else if len <= 255 {
            self.buf[self.len] = 0x78;
            self.buf[self.len + 1] = len as u8;
            self.len += 2;
        } else {
            self.buf[self.len] = 0x79;
            self.buf[self.len + 1] = (len >> 8) as u8;
            self.buf[self.len + 2] = len as u8;
            self.len += 3;
        }
        let mut i = 0;
        while i < len {
            self.buf[self.len] = bytes[i];
            self.len += 1;
            i += 1;
        }
        self
    }
}

impl<const N: usize> Default for ConstCborWriter<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to extract exactly the active serialized bytes into a fixed-size array of length M.
pub const fn extract_bytes<const N: usize, const M: usize>(buf: [u8; N]) -> [u8; M] {
    let mut out = [0; M];
    let mut i = 0;
    while i < M {
        out[i] = buf[i];
        i += 1;
    }
    out
}
