CREATE TABLE manual_flag (
	name TEXT PRIMARY KEY,
	description TEXT NOT NULL,
	flag TEXT UNIQUE NOT NULL
);
