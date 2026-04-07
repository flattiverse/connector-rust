/// Access-control role inside one galaxy ACL.
pub enum GalaxyAclKind {
    /// Controls normal player logins that use the player API key.
    Player = 0x01,

    /// Controls admin logins that use the admin API key.
    Admin = 0x04,
}
