mod owner;
mod server;

use crate::{
    components::prelude::*, dayz::server::ensure_server_created, user::ensure_owner_created,
};
use constcat::concat;
use derive_more::Display;
use topcoat::{
    Result,
    context::Cx,
    router::{
        Slot,
        error::{RouterErrorExt, redirect},
        layout, page,
        request::uri,
    },
    view::{View, component, view},
};
use tracing::instrument;

pub const SETUP_ROOT: &str = "/setup";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, Default)]
pub(in super::setup) enum SetupStep {
    #[default]
    Owner,
    Server,
    Done,
}

impl SetupStep {
    #[allow(dead_code)]
    pub const TOTAL: u8 = 3;

    pub const fn number(self) -> u8 {
        match self {
            SetupStep::Owner => 1,
            SetupStep::Server => 2,
            SetupStep::Done => 3,
        }
    }

    #[instrument(level = "trace", skip_all, ret)]
    pub fn from_context(cx: &Cx) -> Option<Self> {
        let path = uri(cx).path();
        if path.starts_with(Self::Owner.to_uri()) {
            Some(Self::Owner)
        } else if path.starts_with(Self::Server.to_uri()) {
            Some(Self::Server)
        } else if path.starts_with(Self::Done.to_uri()) {
            Some(Self::Done)
        } else {
            None
        }
    }

    pub const fn to_uri(self) -> &'static str {
        match self {
            Self::Owner => concat!(SETUP_ROOT, "/owner"),
            Self::Server => concat!(SETUP_ROOT, "/server"),
            Self::Done => concat!(SETUP_ROOT, "/done"),
        }
    }
}

#[layout]
async fn setup_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let current_step = SetupStep::from_context(cx).unwrap_or_default();

    let setup_title = match current_step {
        SetupStep::Owner => "Owner Account",
        SetupStep::Server => "Server Config",
        SetupStep::Done => "Setup Finished",
    };
    let setup_description = match current_step {
        SetupStep::Owner => {
            "Enter the details for your main administration account, this will have full permissions to manager your ServerZ instance"
        }
        SetupStep::Server => {
            "Enter and confirm the setup details for your DayZ server as this will define where your server is created and persisted to."
        }
        SetupStep::Done => "All done, Enjoy!",
    };

    Ok(view! {
        <div
            class="setup-shell relative isolate flex min-h-screen flex-row justify-center gap-10 overflow-hidden p-12"
        >
            bg_grid()
            <div class="relative z-10 flex-1 flex">
                <h1 class="text-5xl text-base-content/90">
                    "Server"
                    <span class="text-primary align-text-bottom">"Z"</span>
                    " Setup"
                </h1>
            </div>
            <div class="relative z-10 flex-1 flex justify-center">
                <div
                    class="w-140 bg-base-200 border-base-300 border min-h-140 my-auto p-6 gap-4 flex flex-col rounded-xl"
                >
                    <div class="flex justify-between">
                        <h2 class="text-3xl font-medium">(setup_title)</h2>
                        step_display(current_step: current_step)
                    </div>
                    <p class="text-xs font-light text-base-content/75">
                        (setup_description)
                    </p>
                    divider()
                    (slot)
                </div>
            </div>
        </div>
    })
}

#[page]
pub async fn setup(cx: &Cx) -> Result<()> {
    ensure_owner_created(cx)
        .await
        .ok_or_redirect(SetupStep::Owner.to_uri())?;
    ensure_server_created(cx)
        .await
        .ok_or_redirect(SetupStep::Server.to_uri())?;
    Err(redirect("/").into())
}

#[component]
async fn step_display(cx: &Cx, current_step: SetupStep) -> Result<impl View> {
    Ok(view! {
        tooltip(
            alignment: TooltipAlignment::Start,
            text: "Setup Progres".to_string(),
            <div class="flex flex-col items-center">
                steps(
                    <div class="contents">
                        step(
                            content: "1",
                            color: match current_step.number() {
                                SetupStep::TOTAL => StepColor::Success,
                                _ => StepColor::Accent,
                            }
                        )
                        step(
                            content: "2",
                            color: match current_step.number() {
                                2 => StepColor::Accent,
                                SetupStep::TOTAL => StepColor::Success,
                                _ => StepColor::default(),
                            }
                        )
                        step(
                            content: "3",
                            color: match current_step.number() {
                                SetupStep::TOTAL => StepColor::Success,
                                _ => StepColor::default(),
                            }
                        )
                    </div>
                )
            </div>
        )
    })
}
