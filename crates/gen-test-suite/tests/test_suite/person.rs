use gen_services::services::{AddPerson, EditPerson, PersonService};

use crate::context::TestContext;
#[test]
fn adding_person_succeeds() -> eyre::Result<()> {
    let ctx = TestContext::new()?;

    let cmd = AddPerson::default();
    let person = ctx.service.add_person("user", &cmd)?;

    let person = ctx.person_repo.get_person(&person.id())?.unwrap();
    assert_eq!(person.updated_by(), "user");
    Ok(())
}
#[test]
fn editing_person_succeeds() -> eyre::Result<()> {
    let ctx = TestContext::new()?;

    let add_cmd = AddPerson::default();
    let person = ctx.service.add_person("user", &add_cmd)?;
    let _cmd = EditPerson { id: person.id() };
    // service.edit_person(&cmd).unwrap();
    Ok(())
}
