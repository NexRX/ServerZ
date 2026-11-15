use crate::dayz::server::Server;
use jiff::Timestamp;
use toasty::Model;

/// Modelled after [launch parameters](https://community.bistudio.com/wiki/DayZ:Server_Configuration#Launch_Parameters)
#[derive(Debug, Model)]
pub struct Mod {
    /// Is usually a workshop id
    #[key]
    id: u64,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,

    #[default(false)]
    server_only: bool,

    #[index]
    server_id: u8,
    #[belongs_to(key = server_id, references = id)]
    server: toasty::Deferred<Server>,
}
