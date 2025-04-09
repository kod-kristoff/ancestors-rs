use gen_services::services::{AddPerson, EditPerson, PersonService};

use crate::context::TestContext;
#[test]
fn adding_person_succeeds() {
    let ctx = TestContext::default();

    let cmd = AddPerson::default();
    let person = ctx.service.add_person("user", &cmd).unwrap();

    let person = ctx.person_repo.get_person(&person.id()).unwrap().unwrap();
    assert_eq!(person.updated_by(), "user");
}
#[test]
fn editing_person_succeeds() {
    let ctx = TestContext::default();

    let add_cmd = AddPerson::default();
    let person = ctx.service.add_person("user", &add_cmd).unwrap();
    let _cmd = EditPerson { id: person.id() };
    // service.edit_person(&cmd).unwrap();
}
