ALTER TABLE "steams" RENAME COLUMN "auth_user" TO "steam_username";
-- #[toasty::breakpoint]
ALTER TABLE "steams" DROP COLUMN "auth_password_ciphertext";
-- #[toasty::breakpoint]
ALTER TABLE "steams" DROP COLUMN "auth_password_nonce";
