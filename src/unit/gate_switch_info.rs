
use Error;
use UniversalEnumerable;
use net::BinaryReader;

pub struct GateSwitchInfo {
    name:   String,
    state:  bool
}

impl GateSwitchInfo {
    pub fn from_reader(reader: &mut BinaryReader) -> Result<GateSwitchInfo, Error> {
        Ok(GateSwitchInfo {
            name:   reader.read_string()?,
            state:  reader.read_bool()?,
        })
    }

    pub fn state(&self) -> bool {
        self.state
    }
}

impl UniversalEnumerable for GateSwitchInfo {
    fn name(&self) -> &str {
        &self.name
    }
}