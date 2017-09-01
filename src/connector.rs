
use std::net::ToSocketAddrs;

use std::error;

use Version;
use Error;
use net::Connection;

pub const PROTOCOL_VERSION  : u32       = 34;
pub const CONNECTOR_VERSION : Version   = Version::new(0, 9, 5, 0);

pub struct Connector {
    connection: Connection
}

impl Connector {
    pub fn new(email: &str, password: &str, compression_enabled: bool) -> Result<Connector, Error> {
        // param check
        if email.len() < 6 || email.len() > 256 || password.is_empty() {
            return Err(Error::EmailAndOrPasswordInvalid);
        }

        let addr = "galaxy.flattiverse.com:22".to_socket_addrs()?.next().unwrap();

        // TODO missing block manager
        // TODO missing addConnectionClosedListener
        // TODO missing addPacketReceivedListener

        // TODO missing
        /*
        this.players            = new UniversalHolder<>(playersArray);
        this.universeGroups     = new UniversalHolder<>(universeGroupsArray);
        this.controllables      = new UniversalHolder<>(controllablesArray);
        this.crystals           = new UniversalHolder<>(crystalsArray);
        */

        let mut connector = Connector {
            connection: Connection::new(&addr)?
        };

        connector.login(email, password, compression_enabled)?;
        Ok(connector)
    }

    fn login(&mut self, email: &str, password: &str, compression_enabled: bool) -> Result<(), Error> {

    }
}