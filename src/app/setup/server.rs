use super::SetupStep;
use crate::anyhow;
use crate::components::prelude::*;
use crate::dayz::Server;
use crate::dayz::{
    DEFAULT_ADMIN_LOGS, DEFAULT_DO_LOGS, DEFAULT_LIMIT_FPS, DEFAULT_NET_LOGS, DEFAULT_SERVER_BATTLEYE_RELATIVE_PATH,
    DEFAULT_SERVER_CONFIG_RELATIVE_PATH, DEFAULT_SERVER_MISSION_RELATIVE_PATH, DEFAULT_SERVER_PATH,
    DEFAULT_SERVER_PORT, DEFAULT_SERVER_PROFILE_RELATIVE_PATH, default_cpus,
};
use crate::db::db;
use crate::guard::Guard;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use server_z_macros::FormValidation;
use std::fs;
use topcoat::router::Route as _;
use topcoat::router::error::{SeeOther, see_other};
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
    #[serde(default)]
    #[form(value = DEFAULT_DO_LOGS)]
    do_logs: bool,
    #[garde(skip)]
    #[serde(default)]
    #[form(value = DEFAULT_ADMIN_LOGS)]
    admin_logs: bool,
    #[garde(skip)]
    #[serde(default)]
    #[form(value = DEFAULT_NET_LOGS)]
    net_logs: bool,
}

#[instrument(level = "info", skip(cx), ret, err)]
#[route(POST "/api/setup/create-server")]
async fn create_server(cx: &Cx, Form(input): Form<ServerCreation>) -> Result<SeeOther> {
    Server::ensure_unavailable(cx).await?;
    input.validate()?;

    fs::create_dir_all(&input.server_path).map_err(|err| anyhow!("Failed to create given server path: {err:#?}"))?;
    // TODO: Check we have read/write/execute permissions in this folder

    let server_path = input.server_path.as_str();
    toasty::create!(Server { launch_params: {
        profile_path: format!("{server_path}{DEFAULT_SERVER_PROFILE_RELATIVE_PATH}"),
        config_filepath: format!("{server_path}{DEFAULT_SERVER_CONFIG_RELATIVE_PATH}"),
        mission_path: format!("{server_path}{DEFAULT_SERVER_MISSION_RELATIVE_PATH}"),
        battleye_path: format!("{server_path}{DEFAULT_SERVER_BATTLEYE_RELATIVE_PATH}"),
        port: input.port,
        cpu_count: input.cpu_cores,
        do_logs: input.do_logs,
        admin_logs: input.admin_logs,
        net_logs: input.net_logs,
        limit_fps: input.limit_fps
    } })
    .exec(&mut db(cx))
    .await?;

    Ok(see_other(SetupStep::Server.next().to_uri()))
}

#[page]
pub async fn setup_server(cx: &Cx) -> Result<impl View> {
    Server::ensure_unavailable(cx).await?;

    Ok(view! {
        <form
            method="POST"
            action=(create_server.path().to_string())
            class="flex flex-col flex-1 items-center h-full"
        >
            <div class="flex flex-1 flex-wrap w-full gap-4 justify-evenly">
                input(
                    label: Some("Server Path"),
                    label_for: Some(ServerCreationValidation::ServerPath.name()),
                    inner_attrs: ServerCreationValidation::ServerPath.attrs(cx),
                    attrs: attributes!(class="w-full")
                )
                input(
                    label: Some("CPU Cores"),
                    label_for: Some(ServerCreationValidation::CpuCores.name()),
                    inner_attrs: ServerCreationValidation::CpuCores.attrs(cx),
                    attrs: attributes!(class="w-40")
                )
                input(
                    label: Some("Port"),
                    label_for: Some(ServerCreationValidation::Port.name()),
                    inner_attrs: ServerCreationValidation::Port.attrs(cx),
                    attrs: attributes!(class="w-38")
                )
                input(
                    label: Some("Server FPS Limit"),
                    label_for: Some(ServerCreationValidation::LimitFps.name()),
                    inner_attrs: ServerCreationValidation::LimitFps.attrs(cx),
                    attrs: attributes!(class="w-38")
                )
                fieldset(
                    legend: Some("Enabled Logging"),
                    class: "border border-base-300 rounded-box p-4",
                    checkbox(
                        label: "General (Do)",
                        attrs: ServerCreationValidation::DoLogs.attrs(cx)
                    )
                    checkbox(
                        label: "Admin",
                        attrs: ServerCreationValidation::AdminLogs.attrs(cx)
                    )
                    checkbox(
                        label: "Network",
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
