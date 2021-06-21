#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::Session;
use actix_web::http::{header, StatusCode};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod email_service;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

#[get("/")]
async fn home(session: Session) -> Result<HttpResponse> {
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		println!("SESSION value: {}", count);
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

#[get("/app/*")]
async fn allviews(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	println!("{:?}", req);
	let mut counter = 1;
	if let Some(count) = session.get::<i32>("counter")? {
		println!("SESSION value: {}", count);
		counter = count + 1;
	}

	session.set("counter", counter)?;

	Ok(HttpResponse::build(StatusCode::OK)
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../public/index.html")))
}

fn initialize_db(name: &str) {
	println!("Running database migrations...");
	let connection = PgConnection::establish(&name).expect(&format!("Error connecting to {}", name));

	let result = diesel_migrations::run_pending_migrations(&connection);

	match result {
		Ok(_res) => {
			println!("Migrations done!");
		}
		Err(error) => {
			println!("Database migration error: \n {:#?}", error);
		}
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	std::env::set_var("RUST_LOG", "simple-auth-server=debug,actix_web=info,actix_server=info");
	env_logger::init();
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	initialize_db(&database_url);
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool: models::users::Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
	let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
	let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| "localhost:8086".to_string());

	HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			.wrap(middleware::Logger::default())
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
					.name("auth")
					.path("/")
					.domain(domain.as_str())
					.max_age(86400)
					.secure(false),
			))
			.data(web::JsonConfig::default().limit(4096))
			.service(
				web::scope("/api")
					.service(
						web::resource("/invitations")
							.route(web::post().to(handlers::invitation_handler::post_invitation)),
					)
					.service(
						web::resource("/users")
							.route(web::get().to(handlers::users_handler::get_all)),
					)
					.service(
						web::resource("/users/{user_id}")
							.route(web::get().to(handlers::users_handler::get_by_uuid))
							.route(web::put().to(handlers::users_handler::update_user))
							.route(web::delete().to(handlers::users_handler::delete_user)),
					)
					.service(
						web::resource("/userfavorites/{id}")
							.route(web::post().to(handlers::users_handler::add_favorite_project))
							.route(web::delete().to(handlers::users_handler::delete_favorite_project)),
					)
					.service(
						web::resource("/userskills/{user_id}")
							.route(web::post().to(handlers::users_handler::add_skill))
							.route(web::put().to(handlers::users_handler::update_year)),
					)
					.service(
						web::resource("/skills")
							.route(web::post().to(handlers::skills_handler::create_skill))
							.route(web::get().to(handlers::skills_handler::get_all_skills)),
					)
					.service(
						web::resource("/skills/{id}")
							.route(web::delete().to(handlers::skills_handler::delete_skill)),
					)
					.service(
						web::resource("/skills/levels")
							.route(web::get().to(handlers::skills_handler::get_skill_levels))
							.route(web::post().to(handlers::skills_handler::create_skill_scope_level)),
					)
					.service(
						web::resource("/skills/categories")
							.route(web::get().to(handlers::skills_handler::get_skill_categories))
							.route(web::post().to(handlers::skills_handler::create_skill_category)),
					)
					.service(
						web::resource("/skills/scopes")
							.route(web::get().to(handlers::skills_handler::get_skillscopes))
							.route(web::post().to(handlers::skills_handler::create_skill_scope)),
					)
					.service(
						web::resource("/projects")
							.route(web::get().to(handlers::projects_handler::get_all_projects))
							.route(web::post().to(handlers::projects_handler::create_project)),
					)
					.service(
						web::resource("/projects/{id}")
							.route(web::put().to(handlers::projects_handler::update_project))
							.route(web::delete().to(handlers::projects_handler::delete_project))
							.route(web::get().to(handlers::projects_handler::get_by_pid)),
					)
					.service(
						web::resource("/projectneeds")
							.route(web::post().to(handlers::projects_handler::create_projectneed)),
					)
					.service(
						web::resource("/projectneeds/{id}")
							.route(web::get().to(handlers::projects_handler::get_project_needs))
							.route(web::put().to(handlers::projects_handler::update_projectneed))
							.route(web::delete().to(handlers::projects_handler::delete_projectneed)),
					)
					.service(
						web::resource("/projectskills")
							.route(web::post().to(handlers::projects_handler::create_projectneedskill)),
					)
					.service(
						web::resource("/projectskills/{id}")
							.route(web::get().to(handlers::projects_handler::get_projectneedskills))
							.route(web::delete().to(handlers::projects_handler::delete_projectneedskill)),
					)
					.service(
						web::resource("/matches")
							.route(web::get().to(handlers::matches_handler::get_all_matches)),
					)
					.service(
						web::resource("/matchedusers")
							.route(web::post().to(handlers::matches_handler::get_matches_by_params)),
					)
					.service(
						web::resource("/register/{invitation_id}")
							.route(web::post().to(handlers::register_handler::register_user)),
					)
					.service(
						web::resource("/auth")
							.route(web::post().to(handlers::auth_handler::login))
							.route(web::delete().to(handlers::auth_handler::logout))
							.route(web::get().to(handlers::auth_handler::get_me)),
					),
			)
			.service(fs::Files::new("/public", "public").show_files_listing())
			.service(home)
			.service(allviews)
			.service(web::resource("/").route(web::get().to(|req: HttpRequest| {
				println!("HTTP REQ:\n{:?}\n", req);
				HttpResponse::Found().header(header::LOCATION, "index.html").finish()
			})))
	})
	.bind(server_url)?
	.run()
	.await
}
