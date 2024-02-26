use crate::network::{PacketReader, PacketWriter};
use crate::utils::check_name_or_err_64;
use crate::{GameError, TeamId};

#[derive(Debug, Clone)]
pub struct RegionConfig {
    pub name: String,
    pub start_propability: f64,
    pub respawn_propability: f64,
    pub protected: bool,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub team: u32,
}

impl From<&mut dyn PacketReader> for RegionConfig {
    fn from(reader: &mut dyn PacketReader) -> Self {
        let mut this = Self {
            name: String::default(),
            start_propability: 0.0,
            respawn_propability: 0.0,
            protected: false,
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            team: 0,
        };
        this.read(reader);
        this
    }
}

impl RegionConfig {
    pub(crate) fn read(&mut self, reader: &mut dyn PacketReader) {
        self.name = reader.read_string();
        self.start_propability = reader.read_double();
        self.respawn_propability = reader.read_double();
        self.protected = reader.read_boolean();
        self.left = reader.read_double();
        self.top = reader.read_double();
        self.right = reader.read_double();
        self.bottom = reader.read_double();
        self.team = reader.read_uint32();
    }

    pub(crate) fn write(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_double(self.start_propability);
        writer.write_double(self.respawn_propability);
        writer.write_boolean(self.protected);
        writer.write_double(self.left);
        writer.write_double(self.top);
        writer.write_double(self.right);
        writer.write_double(self.bottom);
        writer.write_uint32(self.team)
    }

    /// Extracts the [`TeamId`]s from the `teams` bit-field.
    pub fn teams(&self) -> impl Iterator<Item = TeamId> {
        const MASK: u32 = 1;
        let team = self.team;
        (0..u32::BITS as u8).flat_map(move |bit| {
            let mask = MASK << bit;
            if team & mask == mask {
                Some(TeamId(bit))
            } else {
                None
            }
        })
    }

    /// The name of the configured [`crate::hierarchy::Region`].
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), GameError> {
        let name = name.into();
        self.name = check_name_or_err_64(name)?;
        Ok(())
    }

    #[inline]
    pub fn name_valid(&self) -> bool {
        check_name_or_err_64(&self.name).is_ok()
    }
}
