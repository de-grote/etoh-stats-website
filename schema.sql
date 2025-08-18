CREATE TABLE IF NOT EXISTS "User" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id")
);
CREATE UNIQUE INDEX "username" ON "User" (
	"name"
);
CREATE TABLE IF NOT EXISTS "DifficultyRequirement" (
	"id"	INTEGER NOT NULL UNIQUE,
	"difficulty"	TEXT NOT NULL,
	"amount"	INTEGER NOT NULL,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "Realm" (
	"name"	TEXT NOT NULL UNIQUE,
	"tower_points"	INTEGER NOT NULL,
	"difficulty_requirement1"	INTEGER,
	"difficulty_requirement2"	INTEGER,
	PRIMARY KEY("name"),
	FOREIGN KEY("difficulty_requirement1") REFERENCES "DifficultyRequirement"("id"),
	FOREIGN KEY("difficulty_requirement2") REFERENCES "DifficultyRequirement"("id")
);
CREATE TABLE IF NOT EXISTS "TowerCompletion" (
	"user_id"	INTEGER NOT NULL,
	"badge_id"	INTEGER NOT NULL,
	"time"	DATETIME NOT NULL,
	PRIMARY KEY("user_id","badge_id"),
	FOREIGN KEY("badge_id") REFERENCES "Tower"("badge_id"),
	FOREIGN KEY("user_id") REFERENCES "User"("id")
);
CREATE TABLE IF NOT EXISTS "Tower" (
	"badge_id"	INTEGER NOT NULL UNIQUE,
	"full_name"	TEXT NOT NULL,
	"acronym"	TEXT NOT NULL,
	"difficulty"	REAL NOT NULL,
	"realm"	TEXT NOT NULL,
	"legacy_badge_id"	INTEGER,
	PRIMARY KEY("badge_id")
);
