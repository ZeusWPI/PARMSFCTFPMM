use std::env;

use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use reqwest::Client;
use serde_json::Map;

use crate::models::{InsertableTeam, Team};
use crate::DbPool;

/// Middleware to update the list of teams before every request
pub(crate) struct UpdateTeams;

impl<S, B> Transform<S, ServiceRequest> for UpdateTeams
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Error = Error;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;
	type InitError = ();
	type Response = ServiceResponse<EitherBody<B>>;
	type Transform = UpdateTeamsService<S>;

	fn new_transform(&self, service: S) -> Self::Future { ok(UpdateTeamsService { service }) }
}

pub(crate) struct UpdateTeamsService<S> {
	service: S,
}

impl<S, B> Service<ServiceRequest> for UpdateTeamsService<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
	type Response = ServiceResponse<EitherBody<B>>;

	fn poll_ready(
		&self,
		ctx: &mut core::task::Context<'_>,
	) -> std::task::Poll<Result<(), Self::Error>> {
		self.service.poll_ready(ctx)
	}

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let db_pool =
			req.app_data::<web::Data<DbPool>>().expect("could not get database pool").to_owned();
		let db_conn = db_pool.get().expect("could not get database connection");

		let fut = self.service.call(req);

		Box::pin(async move {
			let client = Client::new();
			let jopser_url = env::var("PARMESAN_URL").expect("could not get PARMESAN_URL");
			let parmesan_res =
				client.get(jopser_url).send().await.expect("could not query parmesan");
			let json = parmesan_res
				.json::<Map<String, serde_json::Value>>()
				.await
				.expect("could not get json");

			let team_names: Vec<InsertableTeam> = json
				.values()
				.into_iter()
				.map(|n| n.as_str().unwrap().to_owned())
				.map(InsertableTeam::from)
				.collect();

			Team::update_team_list(team_names, db_conn).await;

			let res = fut.await.unwrap().map_into_left_body();

			Ok(res)
		})
	}
}
