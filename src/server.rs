use jiff::Timestamp;
use toasty::{Executor, Model};

use crate::db::Singleton;

#[derive(Debug, Model)]
pub struct Server {
    #[key]
    id: u8,
    profile_path: String,
    server_path: String,
    port: u16,
    cpu_count: u8,
    battleye: bool,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,
}

impl Singleton for Server {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Server::get_by_id(db, Server::SINGLETON_ID).await
    }
}
