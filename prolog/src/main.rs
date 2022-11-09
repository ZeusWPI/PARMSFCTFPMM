#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_json;

use std::env;

use actix_files::Files;
use actix_web::middleware::{Compress, NormalizePath};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use diesel::backend::Backend;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use handlebars::Handlebars;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations<DB: Backend>(conn: &mut impl MigrationHarness<DB>) {
	conn.run_pending_migrations(MIGRATIONS).expect("could not run migrations");
}

#[get("/")]
async fn show_index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
	let body = hb.render("index", &json!({ "content": "test" })).expect("could not render index");

	HttpResponse::Ok().body(body)
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
			.wrap(NormalizePath::trim())
			.wrap(Compress::default())
			.service(Files::new("/static", "./static"))
			.service(show_index)
	})
	.bind(("0.0.0.0", 80))?
	.run()
	.await
}
