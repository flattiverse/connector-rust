pub(crate) mod id {
    /// Issued if the client wants to join the universe
    ///
    /// data: base_address contains the universe id, sub_address contains the team_id
    pub(crate) const C2S_UNIVERSE_JOIN: u8 = 0x1A;

    /// Issued if the client wants to leave the universe
    ///
    /// data: base_address contains the universe id
    pub(crate) const C2S_UNIVERSE_PART: u8 = 0x1B;

    /// Issued if the client wants to know more details of an account
    ///
    /// data: [`Account`] in payload
    pub(crate) const C2S_QUERY_ACCOUNT: u8 = 0x40;

    /// Issued if the client wants to know more details of many accounts using pattern matching
    ///
    /// data: a list of matching account ids
    pub(crate) const C2S_QUERY_ACCOUNTS: u8 = 0x41;

    /// Issued to inform the connector to forget a certain player
    ///
    /// data: base_address contains the player id
    pub(crate) const S2C_PLAYER_REMOVED: u8 = 0x0A;

    /// Issued to inform the connector about a new (== yet unknown) player
    ///
    /// data: base_address contains the player id, data contains the player data
    pub(crate) const S2C_NEW_PLAYER: u8 = 0x0B;

    /// Issued when a player (id) is moved. The old id won't be used for this
    /// player anymore from this point onward.
    ///
    /// data: base_address contains the new position, data contains u16 with old position
    pub(crate) const S2C_PLAYER_DEFRAGMENTED: u8 = 0x0C;

    /// Issued regularly to update the ping value of a player
    ///
    /// data: new ping value
    pub(crate) const S2C_PLAYER_PING_UPDATE: u8 = 0x0D;

    /// Issued whenever a player joined or left an universe
    ///
    /// data: empty data if the player left an universe, universe and team id if the player joined
    /// an universe instead
    pub(crate) const S2C_PLAYER_ASSIGNMENT_UPDATE: u8 = 0x0E;

    /// Issued after the login has completed. This marks also that the client
    /// has received all necessary information about `Universe`s and thereof.
    ///
    /// data: none
    pub(crate) const S2C_LOGIN_RESPONSE: u8 = 0x0F;

    /// Issued whenever a universe definition has been created, updated or when a
    /// universe has been deleted.
    ///
    /// data: nothing for a deleted universe, universe-data for an updated or new universe
    pub(crate) const S2C_UNIVERSE_META_INFO_UPDATED: u8 = 0x10;

    /// Issued whenever a team definition has been created, updated or when a team has
    /// been deleted.
    ///
    /// data: nothing for a deleted team, team-data for an updated or newly created team
    pub(crate) const S2C_UNIVERSE_TEAM_META_INFO_UPDATE: u8 = 0x11;

    /// Issued whenever a galaxy definition has been created, updated or
    /// when a galaxy has been deleted.
    ///
    /// data: nothing for a deleted galaxy, galaxy-data for an updated or newly created galaxy
    pub(crate) const S2C_UNIVERSE_GALAXY_META_INFO_UPDATE: u8 = 0x12;

    /// Issued whenever the system definitions have been changed.
    ///
    /// data: List of Systems for a universe, replaces current known list of Systems
    pub(crate) const S2C_UNIVERSE_SYSTEM_META_INFO_UPDATE: u8 = 0x13;

    /// Issued whenever a session (request) experiences an error. Thus this command
    /// can only be read when the session is not zero.
    ///
    /// data: helper contains the error type, further info is error specific
    pub(crate) const S2C_SESSION_EXCEPTION: u8 = 0xFF;
}
