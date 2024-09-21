use super::*;

use crate::repositories::mem::InMemoryPersonRepo;
#[test]
fn adding_person_succeeds() {
    let repo = InMemoryPersonRepo::arc_new();
    let service = PersonService::new(repo.clone());

    let cmd = AddPerson::default();
    let id = service.add("user", &cmd).unwrap();

    let _person = repo.get(&id).unwrap();
}
#[test]
fn editing_person_succeeds() {
    let repo = InMemoryPersonRepo::arc_new();
    let service = PersonService::new(repo.clone());

    let add_cmd = AddPerson::default();
    let id = service.add("user", &add_cmd).unwrap();
    let cmd = EditPerson { id };
    service.edit(&cmd).unwrap();
}
