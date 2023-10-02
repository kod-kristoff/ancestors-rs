use crate::{domain::conclusion::Person, value_objects::Id};

pub trait PersonRepository {
    fn get(&self, id: &Id) -> Result<Option<Person>, ()>;
    fn save(&self, person: Person) -> Result<(), ()>;
}
