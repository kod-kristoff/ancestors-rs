use gen_types::{Household, HouseholdId};

pub trait HouseholdRepository {
    fn get_household(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError>;
    fn get_all_households(&self) -> Result<Vec<Household>, HouseholdRepositoryError>;
    fn save_household(&self, household: &Household) -> Result<(), HouseholdRepositoryError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum HouseholdRepositoryError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
