use crate::domain::{conclusion::Person, value_objects::Id};

pub trait PersonRepository {
    fn get(&self, id: &Id<Person>) -> Result<Option<Person>, ()>;
    fn save(&self, person: Person) -> Result<(), ()>;
}
