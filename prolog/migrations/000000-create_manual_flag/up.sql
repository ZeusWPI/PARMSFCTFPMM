CREATE TABLE manual_flag (
	name TEXT PRIMARY KEY,
	description TEXT NOT NULL,
	points INTEGER NOT NULL,
	flag TEXT UNIQUE NOT NULL
);
