use crate::app::setup::SetupStep;
use crate::components::prelude::*;
use crate::crypto::Secret;
use crate::dayz::launch_params::{
    DEFAULT_ADMIN_LOGS, DEFAULT_DO_LOGS, DEFAULT_LIMIT_FPS, DEFAULT_NET_LOGS, DEFAULT_SERVER_PATH,
    DEFAULT_SERVER_PORT, default_cpus,
};
use crate::dayz::server::{ensure_server_created, ensure_server_not_created};
use crate::ensure;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use topcoat::router::error::{RouterErrorExt as _, SeeOther, see_other};
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page, route},
    view::{View, attributes, view},
};
use tracing::instrument;

#[derive(Debug, JsonSchema, Validate, FormValidation, Serialize, Deserialize)]
struct ServerCreation {
    #[garde(length(min = 1))]
    #[form(value = DEFAULT_SERVER_PATH)]
    server_path: String,
    #[garde(range(min = 1, max = 64))]
    #[form(value = default_cpus())]
    cpu_cores: u8,
    #[garde(range(min = 1))]
    #[form(value = DEFAULT_SERVER_PORT)]
    port: u16,
    #[garde(range(max = 200))]
    #[form(value = DEFAULT_LIMIT_FPS)]
    limit_fps: u8,
    #[garde(skip)]
    #[form(value = DEFAULT_DO_LOGS)]
    do_logs: bool,
    #[garde(skip)]
    #[form(value = DEFAULT_ADMIN_LOGS)]
    admin_logs: bool,
    #[garde(skip)]
    #[form(value = DEFAULT_NET_LOGS)]
    net_logs: bool,
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/create-server")]
async fn create_owner(cx: &Cx, Form(input): Form<ServerCreation>) -> Result<SeeOther> {
    ensure!(
        ensure_server_created(cx).await.is_err(),
        "Server has already been created"
    )?;
    input.validate()?;

    Ok(see_other(SetupStep::Done.to_uri()))
}

#[page]
pub async fn setup_owner(cx: &Cx) -> Result<impl View> {
    ensure_server_not_created(cx)
        .await
        .ok_or_redirect(SetupStep::Done.to_uri())?;

    Ok(view! {
        <form
            method="POST"
            action="/api/setup/create-server"
            class="flex flex-col flex-1 items-center h-full"
        >
            <div class="flex flex-1 flex-wrap w-full gap-4 justify-evenly">
                input(
                    label: Some("Server Path".into()),
                    label_for: Some(ServerCreationValidation::ServerPath.name().into()),
                    inner_attrs: ServerCreationValidation::ServerPath.attrs(cx),
                    attrs: attributes!(class="w-full")
                )
                input(
                    label: Some("CPU Cores".into()),
                    label_for: Some(ServerCreationValidation::CpuCores.name().into()),
                    inner_attrs: ServerCreationValidation::CpuCores.attrs(cx),
                    attrs: attributes!(class="w-40")
                )
                input(
                    label: Some("Port".into()),
                    label_for: Some(ServerCreationValidation::Port.name().into()),
                    inner_attrs: ServerCreationValidation::Port.attrs(cx),
                    attrs: attributes!(class="w-38")
                )
                input(
                    label: Some("Server FPS Limit".into()),
                    label_for: Some(ServerCreationValidation::LimitFps.name().into()),
                    inner_attrs: ServerCreationValidation::LimitFps.attrs(cx),
                    attrs: attributes!(class="w-38")
                )
                fieldset(
                    legend: Some("Logging Options".into()),
                    class: "border border-base-300 rounded-box p-4",
                    checkbox(
                        label: Some("General (Do)".into()),
                        attrs: ServerCreationValidation::DoLogs.attrs(cx)
                    )
                    checkbox(
                        label: Some("Admin".into()),
                        attrs: ServerCreationValidation::AdminLogs.attrs(cx)
                    )
                    checkbox(
                        label: Some("Network".into()),
                        attrs: ServerCreationValidation::NetLogs.attrs(cx)
                    )
                )
            </div>
            button(
                attrs: attributes!(type="submit" class="mt-auto ml-auto self-end"),
                "Next"
                <div class="iconify solar--arrow-right-bold-duotone" />
            )
        </form>
    })
}
