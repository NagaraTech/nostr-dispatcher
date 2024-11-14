-- Your SQL goes here
-- CREATE DATABASE dispatcher

CREATE TABLE "message"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"from" VARCHAR NOT NULL,
	"to" VARCHAR NOT NULL,
	"action" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"info" JSON NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "record"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"event_id" VARCHAR NOT NULL,
	"relay" VARCHAR NOT NULL,
	"message_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"info" JSON NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);