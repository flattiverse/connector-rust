
use std::fmt;
use std::sync::Arc;
use std::sync::Weak;

use crate::Task;
use crate::Error;
use crate::Connector;
use crate::net::BinaryReader;
use crate::net::BinaryWriter;

#[derive(Clone)]
pub struct Polynomial {
    coefficient0: f32,
    coefficient1: f32,
    coefficient2: f32,
    connector: Weak<Connector>,
}

impl Polynomial {
    pub fn new(quad: f32, lin: f32, off: f32) -> Polynomial {
        Polynomial {
            coefficient2: quad,
            coefficient1: lin,
            coefficient0: off,
            connector: Weak::new()
        }
    }

    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader) -> Result<Polynomial, Error> {
        Ok(Polynomial {
            coefficient0: reader.read_single()?,
            coefficient1: reader.read_single()?,
            coefficient2: reader.read_single()?,
            connector: Arc::downgrade(connector),
        })
    }

    pub fn value(&self, position: f32) -> f32 {
        if let Some(connector) = self.connector.upgrade() {
            connector.register_task_quitely_if_unknown(Task::UsedPolynominal);
        }
        self.coefficient2 * position * position
             + self.coefficient1 * position
             + self.coefficient0
    }

    pub fn write(&self, writer: &mut BinaryWriter) -> Result<(), Error> {
        writer.write_f32(self.coefficient2)?;
        writer.write_f32(self.coefficient1)?;
        writer.write_f32(self.coefficient0)?;
        Ok(())
    }
}




impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polynomial: {}, connector.some={}",
               self,
               self.connector.upgrade().is_some()
        )
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut empty= true;
        let mut plus = false;

        if self.coefficient2 != 0f32 {
            write!(f, "{}x^2", self.coefficient2)?;
            empty = false;
            plus  = true;
        }

        if self.coefficient1 != 0f32 {
            if plus {
                write!(f, " + ")?;
            }
            write!(f, "{}x", self.coefficient1)?;
            empty = false;
            plus  = true;
        }

        if self.coefficient0 != 0f32 {
            if plus {
                write!(f, " + ")?;
            }
            write!(f, "{}", self.coefficient0)?;
            empty = false;
        }

        if empty {
            write!(f, "0")?;
        }
        Ok(())
    }
}