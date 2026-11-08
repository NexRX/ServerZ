use jiff::Timestamp;
use toasty::{Model, stmt::Uuid};

#[derive(Debug, Model)]
pub struct User {
    #[key]
    id: Uuid,
    username: String,
    password_hash: String,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,
}
