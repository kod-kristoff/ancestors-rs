use crate::domain::conclusion::Person;
use ulid::Ulid;

pub trait PersonRepository {
    fn get(&self, id: Ulid) -> Result<Option<Person>, ()>;
    fn save(&self, person: Person) -> Result<(), ()>;
}
