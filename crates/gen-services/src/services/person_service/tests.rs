use super::*;

use crate::repositories::mem::InMemoryPersonRepo;
#[test]
fn adding_person_succeeds() {
    let repo = InMemoryPersonRepo::arc_new();
    let service = PersonService::new(repo.clone());

    let cmd = AddPerson::default();
    service.add(&cmd).unwrap();

    let person = repo.get(&cmd.id).unwrap();
}
#[test]
fn editing_person_succeeds() {
    let repo = InMemoryPersonRepo::arc_new();
    let service = PersonService::new(repo.clone());

    let add_cmd = AddPerson::default();
    service.add(&add_cmd).unwrap();
    let cmd = EditPerson {
        id: add_cmd.id.clone(),
    };
    service.edit(&cmd).unwrap();
}
