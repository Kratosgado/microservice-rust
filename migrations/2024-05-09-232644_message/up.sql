-- Your SQL goes here
ALTER TABLE "messages" DROP COLUMN "username";
ALTER TABLE "messages" ADD COLUMN "username" VARCHAR NOT NULL;

