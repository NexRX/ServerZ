mod owner;
mod server;

use crate::{components::prelude::*, server::ensure_server_created, user::ensure_owner_created};
use constcat::concat;
use derive_more::Display;
use topcoat::{
    Result,
    context::Cx,
    router::{Slot, error::RouterErrorExt, layout, page, request::uri},
    view::{View, component, view},
};

pub const SETUP_ROOT: &str = "/setup";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
enum SetupStep {
    Owner,
    Server,
}

impl SetupStep {
    pub const TOTAL: u8 = 2;

    pub fn number(self) -> u8 {
        match self {
            SetupStep::Owner => 1,
            SetupStep::Server => 2,
        }
    }

    pub fn from_context(cx: &Cx) -> Option<Self> {
        let path = uri(cx).path();
        if path.starts_with(Self::Owner.to_uri()) {
            Some(Self::Owner)
        } else if path.starts_with(Self::Server.to_uri()) {
            Some(Self::Server)
        } else {
            None
        }
    }

    pub const fn to_uri(self) -> &'static str {
        match self {
            Self::Owner => concat!(SETUP_ROOT, "/owner"),
            Self::Server => concat!(SETUP_ROOT, "/server"),
        }
    }
}

#[layout]
async fn setup_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let current_step = SetupStep::from_context(cx).ok_or_redirect(SETUP_ROOT)?;
    let setup_title = match current_step {
        SetupStep::Owner => "Owner Account",
        SetupStep::Server => "Server Config",
    };
    let setup_description = match current_step {
        SetupStep::Owner => {
            "Enter the details for your main administration account, this will have full permissions to manager your ServerZ instance"
        }
        SetupStep::Server => {
            "Enter and confirm the setup details for your DayZ server as this will define where your server is created and persisted to."
        }
    };

    Ok(view! {
        <div class="flex flex-row justify-center gap-10 p-12 min-h-full">
            <div class="flex-1 flex">
                <h1 class="text-5xl">
                    "Server"
                    <span class="text-primary">"Z"</span>
                    " Setup"
                </h1>
            </div>
            <div class="flex-1 flex justify-center">
                <div
                    class="w-140 bg-base-200 border-base-300 border min-h-140 my-auto p-6 gap-4 flex flex-col"
                >
                    <div class="flex justify-between">
                        <h2 class="text-3xl font-medium">(setup_title)</h2>
                        step_display(current_step: current_step)
                    </div>
                    <p class="text-xs font-light">(setup_description)</p>
                    divider()
                    (slot)
                </div>
            </div>
        </div>
    })
}

#[page]
pub async fn setup(cx: &Cx) -> Result<impl View> {
    ensure_owner_created(cx)
        .await
        .ok_or_redirect(SetupStep::Owner.to_uri())?;
    ensure_server_created(cx)
        .await
        .ok_or_redirect(SetupStep::Server.to_uri())?;

    Ok(view! { <main>"Setup Complete"</main> })
}

#[component]
async fn step_display(current_step: SetupStep) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col items-center">
            <span class="font-bold text-sm">"Progress"</span>
            steps(
                <div class="contents">
                    step(
                        content: "1",
                        color: match current_step.number() {
                            1 => StepColor::Accent,
                            _ => StepColor::Success,
                        }
                    )
                    step(
                        content: "2",
                        color: match current_step.number() {
                            2 => StepColor::Accent,
                            3 => StepColor::Success,
                            _ => StepColor::default(),
                        }
                    )
                </div>
            )
        </div>
    })
}
