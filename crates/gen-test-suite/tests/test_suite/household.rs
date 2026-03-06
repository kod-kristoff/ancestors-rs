use gen_services::services::{HouseholdService, PersonService};
use gen_types::{Household, Person, value_objects::MemberInfo};

use crate::context::TestContext;
#[test]
fn adding_household_succeeds() -> eyre::Result<()> {
    let ctx = TestContext::new()?;

    let mut household = Household::default();
    let household_id = household.id();
    let person = Person::default();
    let person_id = person.id();
    household.update_body("user", |body| {
        body.add_member(MemberInfo::with_id(person.id()).role("man"))
    });
    ctx.service.add_person_raw("user", person)?;
    ctx.service.add_household_raw("user", household)?;

    let household = ctx.household_repo.get_household(&household_id)?.unwrap();
    assert_eq!(household.updated_by(), "user");
    let mut found_person = false;
    for member in household.body().members() {
        if *member.id() == person_id {
            found_person = true;
            break;
        }
    }
    assert!(found_person, "Did not find '{person_id}' among members");

    let actual = ctx.household_repo.get_all_households()?;
    assert_eq!(actual, &[household]);
    Ok(())
}
