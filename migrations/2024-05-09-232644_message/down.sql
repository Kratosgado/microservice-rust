-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP COLUMN "username";
ALTER TABLE "messages" ADD COLUMN "username" VARCHAR(255) NOT NULL;

