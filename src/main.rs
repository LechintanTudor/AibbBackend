use actix_web::web;
use actix_web::{App, HttpServer};
use business::PersonService;
use persistence::postgres::PgPersonRepository;
use sqlx::postgres::PgPoolOptions;

mod api;
mod business;
mod domain;
mod persistence;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let connection_string = std::env::var("CONNECTION_STRING")?;
    let max_connections: u32 = std::env::var("MAX_CONNECTIONS")?.parse()?;
    let server_address = std::env::var("SERVER_ADDRESS")?;

    let pool =
        PgPoolOptions::new().max_connections(max_connections).connect(&connection_string).await?;

    let server = HttpServer::new(move || {
        let persons = PgPersonRepository::new(pool.clone());
        let persons_service = PersonService::new(persons.clone());

        App::new().service(
            web::resource("/persons")
                .app_data(web::Data::new(persons_service))
                .route(web::post().to(api::person::create_person))
                .route(web::get().to(api::person::get_all_persons)),
        )
    })
    .bind(server_address.clone())?
    .run();

    println!("Started server at: {}/", server_address);
    Ok(server.await?)
}
