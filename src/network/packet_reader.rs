use crate::network::PacketHeader;

pub struct PacketReader<'a> {
    header: PacketHeader,
    data: &'a [u8],
    position: usize,
}

impl<'a> PacketReader<'a> {
    #[inline]
    pub fn new(header: PacketHeader, data: &'a [u8]) -> Self {
        Self {
            header,
            data,
            position: 0,
        }
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    #[inline]
    pub fn with_positoin(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    #[inline]
    pub fn header(&self) -> &PacketHeader {
        &self.header
    }

    pub fn read_int32(&mut self) -> i32 {
        debug_assert!(
            self.position + 4 <= self.data.len(),
            "Can't read out of bounds."
        );
        let value = i32::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);
        self.position += 4;
        value
    }

    pub fn read_string(&mut self, length: impl Into<usize>) -> String {
        let length = length.into();
        debug_assert!(
            self.position + length <= self.data.len(),
            "Can't read out of bounds."
        );
        let value = String::from_utf8(self.data[self.position..][..length].to_vec())
            .expect("Invalid UTF-8 data received");
        self.position += length;
        value
    }
}
