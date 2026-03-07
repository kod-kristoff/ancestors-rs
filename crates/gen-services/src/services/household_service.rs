use gen_types::Household;

pub trait HouseholdService {
    // fn add_household(&self, user: &str, household: &AddHousehold) -> Result<Household, AddHouseholdError>;
    fn add_household_raw(
        &self,
        user: &str,
        household: Household,
    ) -> Result<Household, AddHouseholdError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddHouseholdError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
