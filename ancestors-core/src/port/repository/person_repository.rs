use crate::component::person::domain::Person;
use crate::shared_kernel::component::person::domain::PersonId;

pub trait PersonRepository {
    fn get(&self, id: &PersonId) -> Result<Option<Person>, ()>;
    fn save(&self, person: Person) -> Result<(), ()>;
}
