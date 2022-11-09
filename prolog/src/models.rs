use actix_web::web;
use diesel::insert_or_ignore_into;
use diesel::prelude::*;
use serde::Serialize;
use serde_json::Map;

mod schema {
	table! {
		manual_flag (name) {
			name -> Text,
			flag -> Text,
		}
	}

	table! {
		team (name) {
			name -> Text,
			points -> Integer,
		}
	}
}

use self::schema::{manual_flag, team};
use crate::DbConn;

#[derive(Clone, Identifiable, Queryable, Serialize)]
#[diesel(primary_key(name))]
#[diesel(table_name = manual_flag)]
pub(crate) struct ManualFlag {
	name: String,
	flag: String,
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
}

#[derive(Clone, Identifiable, Queryable)]
#[diesel(primary_key(name))]
#[diesel(table_name = team)]
pub(crate) struct Team {
	name:   String,
	points: i32,
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
