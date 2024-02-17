use crate::network::PacketWriter;
use crate::Team;

#[derive(Default)]
pub struct TeamConfig {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<&Team> for TeamConfig {
    fn from(team: &Team) -> Self {
        Self {
            name: team.name().to_string(),
            red: team.red(),
            green: team.green(),
            blue: team.blue(),
        }
    }
}

impl TeamConfig {
    #[inline]
    pub(crate) fn write_to(&self, writer: &mut dyn PacketWriter) {
        writer.write_string(&self.name);
        writer.write_byte(self.red);
        writer.write_byte(self.green);
        writer.write_byte(self.blue);
    }
}
