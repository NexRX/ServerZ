use super::{DAYZ_SERVER_APPID, Depot};
use anyhow::{Result, anyhow};
use std::{default, process::Stdio};
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    process::Command,
    sync::broadcast::{self, Receiver, Sender},
};
use tracing::{instrument, trace};

pub struct DepotSignin {
    eventbus: Sender<DepotSigninEvent>,
}

#[derive(Debug, Clone)]
pub enum DepotSigninEvent {
    QR(String),
    Failed,
}

impl DepotSignin {
    // CLI
    const CLI_QR_READY: &str = "Use the Steam Mobile App to sign in with this QR code:";
    const CLI_QR_LINES: usize = 35;

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
        let cmd = format!("{} -app {DAYZ_SERVER_APPID} -qr", Depot::BINARY_FILEPATH);
        let mut child = Command::new(cmd).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

        let mut _stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed depot sign-in (internal error 0a)"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed depot sign-in (internal error 0b)"))?;
        let mut lines = BufReader::new(stdout).lines();

        while let Some(line) = lines.next_line().await? {
            trace!(?line, "New Line");
            if line.starts_with(Self::CLI_QR_READY) {
                let mut qr = String::new();
                for _ in 0..Self::CLI_QR_LINES {
                    qr.push_str(
                        &lines.next_line().await?.ok_or_else(|| anyhow!("Failed depot sign-in (internal error 1a)"))?,
                    );
                }
                self.eventbus.send(DepotSigninEvent::QR(qr))?;
            }

            if line.starts_with(Self::CLI_INIT_FAILED) {
                self.eventbus.send(DepotSigninEvent::Failed)?;
                return Err(anyhow!("Didn't sign-in via QR in time"));
            }

            // stdin.write_all(b"anonymous\n").await?;
            // stdin.flush().await?;
        }

        todo!()
    }
}

// 66 char long; 37 lines (8 of which are just new lines)

//
//
//
//
//        ██████████████      ██  ██  ██        ██    ██████████████
//        ██          ██  ██  ██    ██  ██████        ██          ██
//        ██  ██████  ██          ██  ████            ██  ██████  ██
//        ██  ██████  ██  ██████  ████    ██████  ██  ██  ██████  ██
//        ██  ██████  ██    ██        ██    ██  ██    ██  ██████  ██
//        ██          ██  ████  ████    ████  ██  ██  ██          ██
//        ██████████████  ██  ██  ██  ██  ██  ██  ██  ██████████████
//                          ██████    ████    ██████
//        ██████████  ████████  ██        ██████  ████  ██  ██  ██
//        ██    ██  ██  ██    ██  ████          ██  ████████      ██
//              ████  ██  ██  ██    ████████  ██  ████  ████
//        ██████    ██    ██      ████          ████    ██████  ██
//        ██      ██████  ██████  ██  ██████████  ██        ████
//          ██████████  ██████                  ██  ████████      ██
//        ██  ██  ██████    ████████    ██████    ██  ██  ██████
//                ████  ████    ██    ██████    ████    ████    ██
//        ██  ██████  ████  ██  ██      ██████████  ██  ██  ████
//        ██    ████          ██  ██████        ██  ████████  ██  ██
//        ██    ████████    ████    ██████        ██  ██      ██
//        ██  ████        ██      ████    ██          ████      ██
//        ██      ██████████      ██  ██████████  ██████████  ██████
//                        ██████                ████      ██████████
//        ██████████████  ██  ██████    ██████  ████  ██  ██████
//        ██          ██    ██  ██    ██      ██  ██      ██    ████
//        ██  ██████  ██  ██    ██      ████████  ██████████  ██
//        ██  ██████  ██  ██      ██████        ████        ████████
//        ██  ██████  ██  ██        ██████      ████  ██  ████████
//        ██          ██  ██  ██  ████  ██    ██  ████  ██  ██  ██
//        ██████████████  ██      ██  ██████████          ██████
//
//
//
//
