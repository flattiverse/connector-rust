use std::io::Write;

pub trait PacketWriter {
    fn write_4s(&mut self, number: f64, shift: f64);
}

impl<T: Write> PacketWriter for T {
    fn write_4s(&mut self, number: f64, shift: f64) {
        let value = number * shift + 0.5;
        let value = value as i32;
        self.write_all(&value.to_le_bytes()).unwrap();
    }
}
