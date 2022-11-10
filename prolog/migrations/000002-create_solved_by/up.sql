CREATE TABLE solved_by (
	flag_name TEXT NOT NULL,
	team_name TEXT NOT NULL,

	CONSTRAINT pk__solved_by PRIMARY KEY (flag_name, team_name),

	CONSTRAINT fk__solved_by__flag_name FOREIGN KEY (flag_name) REFERENCES manual_flag(name),
	CONSTRAINT fk__solved_by__team_name FOREIGN KEY (team_name) REFERENCES team(name)
);
