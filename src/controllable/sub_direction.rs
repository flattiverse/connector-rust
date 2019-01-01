
use crate::Error;
use crate::Vector;

use crate::net::BinaryReader;
use crate::net::BinaryWriter;

#[derive(Clone, Debug)]
pub struct SubDirection {
    time: u16,
    movement: Vector
}

impl SubDirection {
    pub fn new(time: u16, movement: Vector) -> SubDirection {
        SubDirection { time, movement }
    }

    pub fn from_reader(reader: &mut BinaryReader) -> Result<SubDirection, Error> {
        Ok(SubDirection {
            time:       reader.read_u16()?,
            movement:   Vector::from_reader(reader)?,
        })
    }

    pub fn time(&self) -> u16 {
        self.time
    }

    pub fn movement(&self) -> &Vector {
        &self.movement
    }

    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), Error> {
        writer.write_u16(self.time)?;
        self.movement.write(writer)?;
        Ok(())
    }
}