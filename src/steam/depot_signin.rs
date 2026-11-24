use super::{DAYZ_SERVER_APPID, Depot, STEAM_DATA_FOLDER};
use anyhow::{Result, anyhow};
use std::process::Stdio;
use tokio::{
    fs::{canonicalize, create_dir_all},
    io::{AsyncBufReadExt as _, BufReader},
    process::Command,
    sync::broadcast::{self, Receiver, Sender},
};
use tracing::{debug, instrument, trace};

pub struct DepotSignin {
    eventbus: Sender<DepotSigninEvent>,
}

#[derive(Debug, Clone)]
pub enum DepotSigninEvent {
    QR(String),
    Failed { message: String },
    Success { steam_username: String },
}

impl DepotSignin {
    // CLI
    const CLI_QR_READY: &str = "Use the Steam Mobile App to sign in with this QR code:";
    const CLI_QR_LINES: usize = 35;

    const CLI_QR_SUCCESS_USERNAME_START: &str = "Success! Next time you can login with -username ";
    const CLI_QR_SUCCESS_USERNAME_END: &str = " -remember-password instead of -qr.";

    const CLI_QR_SUCCESS_1: &str = "Got AppInfo for 223350";
    const CLI_QR_SUCCESS_2: &str = "Disconnected from Steam";

    const CLI_INIT_FAILED: &str = "Error: InitializeSteam failed";

    pub fn new() -> Self {
        Self { eventbus: broadcast::channel(crate::STD_EVENT_BUFFER).0 }
    }

    pub fn subscribe(&self) -> Receiver<DepotSigninEvent> {
        self.eventbus.subscribe()
    }

    pub fn start_bg(self) {
        tokio::spawn(self.start());
    }

    #[instrument(level = "debug", skip_all, ret, err)]
    pub async fn start(self) -> Result<()> {
        create_dir_all(STEAM_DATA_FOLDER).await?;
        let path = canonicalize(Depot::BINARY_FILEPATH).await?;
        debug!(?path, "Attempting to run command");
        let mut child = Command::new(path)
            .current_dir(STEAM_DATA_FOLDER)
            .arg("-app")
            .arg(DAYZ_SERVER_APPID.to_string())
            .arg("-qr")
            .arg("-remember-password")
            .arg("-manifest-only")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut _stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed depot sign-in (internal error 0a)"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed depot sign-in (internal error 0b)"))?;
        let mut lines = BufReader::new(stdout).lines();

        let mut steam_username: Option<String> = None;
        let mut is_ending_successfully = false;

        while let Some(line) = lines.next_line().await? {
            trace!(?line, "New Line");
            if line.starts_with(Self::CLI_QR_READY) {
                let mut qr = String::new();
                for i in 0..Self::CLI_QR_LINES {
                    #[allow(clippy::manual_range_contains)]
                    if i < 4 || i > Self::CLI_QR_LINES - 3 {
                        lines.next_line().await?;
                        continue;
                    }

                    qr.push_str(
                        &lines.next_line().await?.ok_or_else(|| anyhow!("Failed depot sign-in (internal error 1a)"))?,
                    );
                    qr.push('\n');
                }
                self.eventbus.send(DepotSigninEvent::QR(qr))?;
                continue;
            }

            if line.starts_with(Self::CLI_INIT_FAILED) {
                const MESSAGE: &str = "Didn't sign-in via QR in time";
                self.eventbus.send(DepotSigninEvent::Failed { message: MESSAGE.to_string() })?;
                return Err(anyhow!("Failed steam depot signin: {MESSAGE}"));
            }

            if line.starts_with(Self::CLI_QR_SUCCESS_USERNAME_START) {
                debug!("handling successful signin");
                steam_username = Some(
                    line.replace(Self::CLI_QR_SUCCESS_USERNAME_START, "")
                        .replace(Self::CLI_QR_SUCCESS_USERNAME_END, ""),
                );
                continue;
            }

            if line.starts_with(Self::CLI_QR_SUCCESS_1) {
                is_ending_successfully = true;
            }

            if line.starts_with(Self::CLI_QR_SUCCESS_2) {
                let cmd_success = child.wait().await?.success();
                match (is_ending_successfully && cmd_success, steam_username) {
                    (true, Some(steam_username)) => {
                        debug!("Signed in to steam successfully");
                        self.eventbus.send(DepotSigninEvent::Success { steam_username })?;
                        return Ok(());
                    }
                    (_, _) => {
                        const MESSAGE: &str = "Signin failed, you may not own DayZ";
                        self.eventbus.send(DepotSigninEvent::Failed { message: MESSAGE.to_string() })?;
                        return Err(anyhow!("Failed steam depot signin: {MESSAGE}"));
                    }
                };
            }
        }

        const MESSAGE: &str = "Technical signin failure";
        self.eventbus.send(DepotSigninEvent::Failed { message: MESSAGE.to_string() })?;
        return Err(anyhow!("Failed steam depot signin: {MESSAGE}"));
    }
}
