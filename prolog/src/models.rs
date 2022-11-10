use actix_web::web;
use diesel::prelude::*;
use diesel::{insert_into, insert_or_ignore_into};
use serde::Serialize;
use serde_json::Map;

mod schema {
	table! {
		manual_flag (name) {
			name -> Text,
			description -> Text,
			flag -> Text,
		}
	}

	table! {
		team (name) {
			name -> Text,
			points -> Integer,
		}
	}

	table! {
		solved_by (flag_name, team_name) {
			flag_name -> Text,
			team_name -> Text,
		}
	}

	joinable!(solved_by -> manual_flag (flag_name));
	joinable!(solved_by -> team (team_name));

	allow_tables_to_appear_in_same_query!(manual_flag, team, solved_by,);
}

use self::schema::{manual_flag, solved_by, team};
use crate::DbConn;

#[derive(Clone, Identifiable, Queryable, Serialize)]
#[diesel(primary_key(name))]
#[diesel(table_name = manual_flag)]
pub(crate) struct ManualFlag {
	name:        String,
	description: String,
	flag:        String,
}

#[derive(Clone, Identifiable, Queryable, AsChangeset)]
#[diesel(primary_key(name))]
#[diesel(table_name = team)]
pub(crate) struct Team {
	name:   String,
	points: i32,
}

#[derive(Clone, Identifiable, Queryable, Associations)]
#[diesel(primary_key(flag_name, team_name))]
#[diesel(table_name = solved_by)]
#[diesel(belongs_to(ManualFlag, foreign_key = flag_name))]
#[diesel(belongs_to(Team, foreign_key = team_name))]
pub(crate) struct SolvedBy {
	flag_name: String,
	team_name: String,
}

impl ManualFlag {
	/// Check if a flag is correct for a [`ManualFlag`] with a given name
	pub(crate) async fn verify_flag(
		supplied_name: String,
		supplied_flag: String,
		mut conn: DbConn,
	) -> bool {
		let requested_flag: ManualFlag = web::block(move || {
			use self::manual_flag::dsl::*;

			manual_flag.filter(name.eq(supplied_name)).first(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");

		requested_flag.flag == supplied_flag
	}

	/// Get a list of all flags
	pub(crate) async fn get_all(mut conn: DbConn) -> Vec<Self> {
		let challenges = web::block(move || {
			use self::manual_flag::dsl::*;

			manual_flag.load(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");

		challenges
	}
}

#[derive(Insertable)]
#[diesel(table_name = team)]
pub(crate) struct InsertableTeam {
	name: String,
}

impl From<String> for InsertableTeam {
	fn from(value: String) -> Self { Self { name: value } }
}

impl Team {
	/// Update the list of team news with potential new names
	pub(crate) async fn update_team_list(team_names: Vec<InsertableTeam>, mut conn: DbConn) {
		web::block(move || {
			use self::team::dsl::*;

			insert_or_ignore_into(team).values(team_names).execute(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");
	}

	/// Increment the score for a team by a given amount
	pub(crate) async fn incr_team_score(team_name: String, incr: i32, mut conn: DbConn) {
		web::block(move || {
			use self::team::dsl::*;

			let prev: i32 = team
				.filter(name.eq(team_name.clone()))
				.select(points)
				.first(&mut conn)
				.expect("could not get points");

			diesel::update(team)
				.filter(name.eq(team_name))
				.set(points.eq(prev + incr))
				.execute(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");
	}

	/// Get a flat JSON object of all the scores
	pub(crate) async fn get_scores(mut conn: DbConn) -> serde_json::Value {
		let team_vec = web::block(move || {
			use self::team::dsl::*;

			team.load::<Self>(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");

		let mut flat_map = Map::new();
		for team in team_vec {
			flat_map.insert(team.name, team.points.into());
		}

		serde_json::Value::Object(flat_map)
	}
}

impl SolvedBy {
	/// Check if a flag has been solved by a team
	pub(crate) async fn has_been_solved(
		flag_name_: String,
		team_name_: String,
		mut conn: DbConn,
	) -> bool {
		let solved = web::block(move || {
			use diesel::dsl::{exists, select};

			use self::solved_by::dsl::*;

			select(exists(
				solved_by.filter(flag_name.eq(flag_name_)).filter(team_name.eq(team_name_)),
			))
			.get_result(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");

		solved
	}

	pub(crate) async fn set_solved(flag_name_: String, team_name_: String, mut conn: DbConn) {
		web::block(move || {
			use self::solved_by::dsl::*;

			insert_into(solved_by)
				.values((flag_name.eq(flag_name_), team_name.eq(team_name_)))
				.execute(&mut conn)
		})
		.await
		.expect("blocking call failed")
		.expect("db query failed");
	}
}
