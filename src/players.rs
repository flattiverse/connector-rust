use std::convert::TryFrom;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use crate::io::BinaryReader;
use crate::num_traits::FromPrimitive;
use crate::packet::Packet;

#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    name: String,
    online: bool,
    ping: f32,
    account: u32,
    universe: Option<u16>,
    team: Option<u8>,
}

impl Player {
    pub(crate) fn update_ping(&mut self, packet: &Packet) -> Result<(), IoError> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        self.ping = reader.read_single()?;
        Ok(())
    }

    pub(crate) fn update_assignment(&mut self, packet: &Packet) -> Result<(), IoError> {
        if packet.payload.is_none() {
            self.universe = None;
            self.team = None;
        } else {
            let reader = &mut packet.payload() as &mut dyn BinaryReader;
            self.universe = Some(reader.read_uint16()?);
            self.team = Some(reader.read_byte()?);
        }
        Ok(())
    }
}

impl TryFrom<&Packet> for Player {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Player {
            id: i32::from(packet.base_address),
            account: packet.id,
            name: reader.read_string()?,
            online: reader.read_bool()?,
            ping: reader.read_single()?,
            universe: None,
            team: None,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Team {
    id: u8,
    name: String,
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

impl Team {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color_rgb(&self) -> (f32, f32, f32) {
        (self.color_r, self.color_g, self.color_b)
    }

    pub(crate) fn update(&mut self, reader: &mut dyn BinaryReader) -> Result<(), IoError> {
        self.name = reader.read_string()?;
        self.color_r = f32::from(reader.read_byte()?) / 255_f32;
        self.color_g = f32::from(reader.read_byte()?) / 255_f32;
        self.color_b = f32::from(reader.read_byte()?) / 255_f32;
        Ok(())
    }
}

impl TryFrom<&Packet> for Team {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let mut team = Team {
            id: packet.sub_address,
            ..Default::default()
        };
        let reader = &mut packet.payload() as &mut dyn BinaryReader;
        team.update(reader)?;
        Ok(team)
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive, Copy, Clone, PartialOrd, PartialEq)]
pub enum AccountStatus {
    Banned = 0,
    OptIn = 1,
    Normal = 2,
    ReOptIn = 3,
    Vanished = 4,
}

#[derive(Debug, Clone)]
pub struct Account {
    id: u32,
    name: String,
    status: AccountStatus,
    kills: u32,
    deaths: u32,
    email: Option<String>,
    email_new: Option<String>,
}

impl Account {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> AccountStatus {
        self.status
    }

    /// The total kills of this account
    pub fn kills(&self) -> u32 {
        self.kills
    }

    /// The total deaths of ths account
    pub fn deaths(&self) -> u32 {
        self.deaths
    }

    /// The current E-Mail address of this account. This will be `None`
    /// if you do not have the privileges to view it.
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(AsRef::as_ref)
    }

    /// The new E-Mail address of this account, which is only populated after
    /// this account re-oped-in. This will be `None` if you do not have the
    /// privileges to view it.
    pub fn email_new(&self) -> Option<&str> {
        self.email_new.as_ref().map(AsRef::as_ref)
    }

    pub fn check_name(name: &str) -> bool {
        if name.is_empty() || name.len() < 2 || name.len() > 32 {
            return false;
        }

        if [" ", ".", "-", "_"]
            .iter()
            .any(|s| name.starts_with(s) || name.ends_with(s))
        {
            return false;
        }

        for char in name.chars() {
            let dec = char as u32;

            if char >= 'a' && char >= 'z' {
                continue;
            }

            if char >= 'A' && char <= 'Z' {
                continue;
            }

            if char >= '0' && char <= '9' {
                continue;
            }

            if dec >= 192 && dec <= 214 {
                continue;
            }

            if dec >= 216 && dec <= 246 {
                continue;
            }

            if char == ' ' || char == '.' || char == '-' || char == '_' {
                continue;
            }

            return false;
        }

        true
    }
}

impl TryFrom<&Packet> for Account {
    type Error = IoError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        let reader = &mut packet.payload() as &mut dyn BinaryReader;

        Ok(Account {
            id: reader.read_u32()?,
            name: reader.read_string()?,
            status: AccountStatus::from_u8(reader.read_byte()?)
                .ok_or(IoError::from(IoErrorKind::InvalidInput))?,
            kills: reader.read_u32()?,
            deaths: reader.read_u32()?,
            email: reader.read_string_empty_is_none()?,
            email_new: reader.read_string_empty_is_none()?,
        })
    }
}
