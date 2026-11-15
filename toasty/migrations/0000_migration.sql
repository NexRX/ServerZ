CREATE TABLE "launch_params" (
    "id" INTEGER NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "profile_path" TEXT NOT NULL,
    "config_filepath" TEXT NOT NULL,
    "mission_path" TEXT NOT NULL,
    "battleye_path" TEXT NOT NULL,
    "port" INTEGER NOT NULL,
    "cpu_count" INTEGER NOT NULL,
    "do_logs" BOOLEAN NOT NULL,
    "admin_logs" BOOLEAN NOT NULL,
    "net_logs" BOOLEAN NOT NULL,
    "freeze_check" BOOLEAN NOT NULL,
    "file_patching" BOOLEAN NOT NULL,
    "limit_fps" INTEGER NOT NULL,
    "storage_path" TEXT,
    "server_id" INTEGER NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_launch_params_by_server_id" ON "launch_params" ("server_id");
-- #[toasty::breakpoint]
CREATE TABLE "mods" (
    "id" INTEGER NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "server_only" BOOLEAN NOT NULL,
    "server_id" INTEGER NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_mods_by_server_id" ON "mods" ("server_id");
-- #[toasty::breakpoint]
CREATE TABLE "servers" (
    "id" INTEGER NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE TABLE "users" (
    "id" BLOB NOT NULL,
    "username" TEXT NOT NULL,
    "password_hash" TEXT NOT NULL,
    "role" TEXT NOT NULL CHECK ("role" IN ('owner', 'member', 'guest')),
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
