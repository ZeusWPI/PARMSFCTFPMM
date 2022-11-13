#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;

use std::env;

use actix_files::Files;
use actix_web::middleware::{Compress, NormalizePath};
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use diesel::backend::Backend;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use handlebars::Handlebars;
use r2d2::{Pool, PooledConnection};

mod middleware;
mod models;

use models::{ManualFlag, SolvedBy, Team};
use serde::Deserialize;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DbPool = Pool<ConnectionManager<SqliteConnection>>;
type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

fn run_migrations<DB: Backend>(conn: &mut impl MigrationHarness<DB>) {
	conn.run_pending_migrations(MIGRATIONS).expect("could not run migrations");
}

#[get("/")]
async fn show_index(hb: web::Data<Handlebars<'_>>, db_pool: web::Data<DbPool>) -> HttpResponse {
	let db_conn = db_pool.get().expect("could not get database connection");
	let challenges = ManualFlag::get_all(db_conn).await;

	let body =
		hb.render("index", &json!({ "challenges": challenges })).expect("could not render index");

	HttpResponse::Ok().body(body)
}

#[get("/scores")]
async fn get_scores(db_pool: web::Data<DbPool>) -> HttpResponse {
	let db_conn = db_pool.get().expect("could not get database connection");

	let scores = Team::get_scores(db_conn).await;

	HttpResponse::Ok().json(scores)
}

#[derive(Deserialize)]
struct TeamNameQuery {
	team_name: String,
}

#[post("/verify/{name}/{flag}")]
async fn verify_flag(
	info: web::Path<(String, String)>,
	query: web::Query<TeamNameQuery>,
	db_pool: web::Data<DbPool>,
) -> HttpResponse {
	let (flag_name, flag) = info.into_inner();
	let team_name = query.into_inner().team_name;

	let db_conn = db_pool.get().expect("could not get database connection");
	if !(Team::exists(team_name.clone(), db_conn).await) {
		return HttpResponse::BadRequest().finish();
	}

	let db_conn = db_pool.get().expect("could not get database connection");
	let solved = SolvedBy::has_been_solved(flag_name.clone(), team_name.clone(), db_conn).await;

	if solved {
		return HttpResponse::Forbidden().finish();
	}

	let db_conn = db_pool.get().expect("could not get database connection");
	let points_opt = ManualFlag::verify_flag(flag_name.clone(), flag, db_conn).await;

	let Some(points) = points_opt else {
		return HttpResponse::Ok().json(json!({ "correct": false }));
	};

	let db_conn = db_pool.get().expect("could not get database connection");
	Team::incr_team_score(team_name.clone(), points, db_conn).await;

	let db_conn = db_pool.get().expect("could not get database connection");
	SolvedBy::set_solved(flag_name, team_name, db_conn).await;

	HttpResponse::Ok().json(json!({ "correct": true }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let sqlite_path = env::var("SQLITE_PATH").expect("missing environment variable SQLITE_PATH");

	let db_pool = r2d2::Pool::builder()
		.build(ConnectionManager::<SqliteConnection>::new(sqlite_path))
		.expect("could not build database connection pool");

	let mut migration_connection =
		db_pool.get().expect("could not get database connection for migrations");
	run_migrations(&mut migration_connection);

	let mut handlebars = Handlebars::new();
	handlebars
		.register_templates_directory(".hbs", "./templates")
		.expect("could not register templates directory");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(db_pool.clone()))
			.app_data(web::Data::new(handlebars.clone()))
			.wrap(middleware::UpdateTeams)
			.wrap(NormalizePath::trim())
			.wrap(Compress::default())
			.service(Files::new("/static", "./static"))
			.service(show_index)
			.service(get_scores)
			.service(verify_flag)
	})
	.bind(("0.0.0.0", 80))?
	.run()
	.await
}
