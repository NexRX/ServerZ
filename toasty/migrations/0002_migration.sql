CREATE TABLE "steams" (
    "id" INTEGER NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL,
    "auth_user" TEXT NOT NULL,
    "auth_password_ciphertext" BLOB NOT NULL,
    "auth_password_nonce" BLOB NOT NULL,
    PRIMARY KEY ("id")
);
