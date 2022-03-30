use crate::domain::Person;
use crate::persistence::interface::PersonRepository;
use std::sync::Arc;

pub type PersonServiceError = crate::persistence::error::RepositoryError;

#[derive(Clone)]
pub struct PersonService {
    persons: Arc<dyn PersonRepository>,
}

impl PersonService {
    pub fn new(persons: Arc<dyn PersonRepository>) -> Self {
        Self { persons }
    }

    pub async fn create(&self, person: &Person) -> Result<Person, PersonServiceError> {
        self.persons.create(person).await
    }

    pub async fn remove(&self, id: i32) -> Result<Option<Person>, PersonServiceError> {
        self.persons.remove(id).await
    }

    pub async fn get(&self, id: i32) -> Result<Option<Person>, PersonServiceError> {
        self.persons.get(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Person>, PersonServiceError> {
        self.persons.get_all().await
    }

    pub async fn clear(&self) -> Result<u64, PersonServiceError> {
        self.persons.clear().await
    }
}
