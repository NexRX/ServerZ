DROP TABLE "todos";
-- #[toasty::breakpoint]
CREATE TABLE "servers" (
    "id" INTEGER NOT NULL,
    "profile_path" TEXT NOT NULL,
    "server_path" TEXT NOT NULL,
    "port" INTEGER NOT NULL,
    "cpu_count" INTEGER NOT NULL,
    "battleye" BOOLEAN NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
