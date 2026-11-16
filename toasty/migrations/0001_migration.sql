CREATE TABLE "sessions" (
    "id" BLOB NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "token_hash" BLOB NOT NULL,
    "expires_at" TEXT NOT NULL,
    "user_id" BLOB NOT NULL,
    PRIMARY KEY ("id")
);
-- #[toasty::breakpoint]
CREATE INDEX "index_sessions_by_token_hash" ON "sessions" ("token_hash");
-- #[toasty::breakpoint]
CREATE INDEX "index_sessions_by_user_id" ON "sessions" ("user_id");
