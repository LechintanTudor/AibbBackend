use crate::business::PersonService;
use crate::persistence::postgres::PgPersonRepository;
use actix_web::web;
use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod business;
mod domain;
mod endpoint;
mod persistence;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("CONNECTION_STRING")?;
    let max_connections = std::env::var("MAX_CONNECTIONS")?.parse::<u32>()?;
    let server_address = std::env::var("SERVER_ADDRESS")?;

    let pool =
        PgPoolOptions::new().max_connections(max_connections).connect(&connection_string).await?;

    let server = HttpServer::new(move || {
        let persons = PgPersonRepository::new(pool.clone());
        let persons_service = PersonService::new(persons.clone());

        App::new().service(
            web::resource("/persons")
                .app_data(web::Data::new(persons_service))
                .route(web::post().to(endpoint::person::create_person))
                .route(web::put().to(endpoint::person::update_person))
                .route(web::delete().to(endpoint::person::clear_persons))
                .route(web::get().to(endpoint::person::get_all_persons)),
        )
    })
    .bind(server_address.clone())?
    .run();

    println!("Started server at: http:/{}/", server_address);
    Ok(server.await?)
}
