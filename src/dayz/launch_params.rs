use crate::{DATA_FOLDER, dayz::server::Server, db::Singleton};
use constcat::concat;
use jiff::Timestamp;
use toasty::{Executor, Model};

// pub const DEFAULT_SERVER_QUERY_PORT: u16 = 27016;
pub const DEFAULT_SERVER_PORT: u16 = 2302;
pub const DEFAULT_DO_LOGS: bool = true;
pub const DEFAULT_ADMIN_LOGS: bool = true;
pub const DEFAULT_NET_LOGS: bool = true;
pub const DEFAULT_FREEZE_CHECK: bool = true;
pub const DEFAULT_FILE_PATCHING: bool = true;
pub const DEFAULT_LIMIT_FPS: u8 = 120;

// these are relative to the server
pub const DEFAULT_SERVER_PROFILE_RELATIVE_PATH: &str = "/profile";
pub const DEFAULT_SERVER_CONFIG_RELATIVE_PATH: &str = "/serverDZ.cfg";
pub const DEFAULT_SERVER_MISSION_RELATIVE_PATH: &str = "/mpmissions/dayzOffline.chernarusplus";
pub const DEFAULT_SERVER_BATTLEYE_RELATIVE_PATH: &str = "/battleye";

pub const DEFAULT_SERVER_PATH: &str = concat!(DATA_FOLDER, "/server");
pub const DEFAULT_SERVER_PROFILE_PATH: &str = concat!(DEFAULT_SERVER_PATH, DEFAULT_SERVER_PROFILE_RELATIVE_PATH);
pub const DEFAULT_SERVER_CONFIG_PATH: &str = concat!(DEFAULT_SERVER_PATH, DEFAULT_SERVER_CONFIG_RELATIVE_PATH);
pub const DEFAULT_SERVER_MISSION_PATH: &str = concat!(DEFAULT_SERVER_PATH, DEFAULT_SERVER_MISSION_RELATIVE_PATH);
pub const DEFAULT_SERVER_BATTLEYE_PATH: &str = concat!(DEFAULT_SERVER_PATH, DEFAULT_SERVER_BATTLEYE_RELATIVE_PATH);

/// Modelled after [launch parameters](https://community.bistudio.com/wiki/DayZ:Server_Configuration#Launch_Parameters)
#[derive(Debug, Model)]
pub struct LaunchParams {
    #[key]
    #[default(LaunchParams::SINGLETON_ID)]
    id: u8,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,

    /// Path to the folder containing server profile. By default, server logs are written to the server profile's directory. Logs/dumps/etc will be created there, along with BattlEye/BEC/Rcon related files (i.e. %userProfile%\Documents\DayZServer)
    #[default(DEFAULT_SERVER_PROFILE_PATH.to_string())]
    profile_path: String,
    /// Selects the Server Config File (i.e. `serverDZ.cfg`)
    #[default(DEFAULT_SERVER_CONFIG_PATH.to_string())]
    config_filepath: String,
    /// Defines the mission used by the server
    #[default(DEFAULT_SERVER_MISSION_PATH.to_string())]
    mission_path: String,
    /// Sets a custom path to the BattlEye files
    #[default(DEFAULT_SERVER_BATTLEYE_PATH.to_string())]
    battleye_path: String,
    /// Port to have dedicated server listen on
    #[default(DEFAULT_SERVER_PORT)]
    port: u16,
    /// Sets the number of logical CPU cores to use for parallel tasks processing. It should be less or equal than the numbers of available cores.
    #[default(default_cpus())]
    cpu_count: u8,
    /// Enables all log messages in the server RPT file
    #[default(DEFAULT_DO_LOGS)]
    do_logs: bool,
    /// Enables the admin log
    #[default(DEFAULT_ADMIN_LOGS)]
    admin_logs: bool,
    /// Enables the network traffic logging
    #[default(DEFAULT_NET_LOGS)]
    net_logs: bool,
    /// Stops the server when frozen for more than 5 min and create a dump file
    #[default(DEFAULT_FREEZE_CHECK)]
    freeze_check: bool,
    /// Ensures that only PBOs are loaded and NO unpacked data.
    #[default(DEFAULT_FILE_PATCHING)]
    file_patching: bool,
    /// Limits server FPS to specified value (current max is 200) to lower CPU usage of low population servers.
    #[default(DEFAULT_LIMIT_FPS)]
    limit_fps: u8,
    /// Defines custom root folder for storage location.
    #[default(Option::<String>::None)]
    storage_path: Option<String>,

    #[index]
    server_id: u8,
    #[belongs_to(key = server_id, references = id)]
    server: toasty::Deferred<Server>,
}

impl Singleton for LaunchParams {
    async fn get(db: &mut dyn Executor) -> toasty::Result<Self> {
        Self::get_by_id(db, Self::SINGLETON_ID).await
    }
}

pub fn default_cpus() -> u8 {
    std::thread::available_parallelism().map(|v| v.get() as u8).unwrap_or(1)
}
