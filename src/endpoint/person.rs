use crate::business::PersonService;
use crate::domain::Person;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatePerson {
    pub name: String,
}

pub async fn create_person(
    person_service: web::Data<PersonService>,
    person: web::Json<CreatePerson>,
) -> HttpResponse {
    person_service
        .create(&Person { id: 0, name: person.name.clone() })
        .await
        .map(|person| HttpResponse::Ok().json(person))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error.to_string()))
}

pub async fn update_person(
    person_service: web::Data<PersonService>,
    person: web::Json<Person>,
) -> HttpResponse {
    person_service
        .update(&person)
        .await
        .map(|modified| HttpResponse::Ok().json(modified))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error.to_string()))
}

pub async fn get_all_persons(person_service: web::Data<PersonService>) -> HttpResponse {
    person_service
        .get_all()
        .await
        .map(|persons| HttpResponse::Ok().json(persons))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error.to_string()))
}

pub async fn clear_persons(person_service: web::Data<PersonService>) -> HttpResponse {
    person_service
        .clear()
        .await
        .map(|count| HttpResponse::Ok().json(count))
        .unwrap_or_else(|error| HttpResponse::InternalServerError().body(error.to_string()))
}
