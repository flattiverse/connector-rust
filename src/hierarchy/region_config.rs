use crate::hierarchy::Region;
use crate::network::PacketWriter;

#[derive(Default)]
pub struct RegionConfig {
    pub name: String,
    pub start_propability: f64,
    pub respawn_prpability: f64,
    pub protected: bool,
}

impl From<&Region> for RegionConfig {
    fn from(region: &Region) -> Self {
        Self {
            name: region.name().to_string(),
            start_propability: region.start_probability(),
            respawn_prpability: region.respawn_probability(),
            protected: region.protected(),
        }
    }
}

impl RegionConfig {
    pub(crate) fn write_to(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_2u(self.start_propability, 100.0);
        writer.write_2u(self.respawn_prpability, 100.0);
        writer.write_boolean(self.protected);
    }
}
