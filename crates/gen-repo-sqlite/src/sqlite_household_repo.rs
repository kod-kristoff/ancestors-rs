use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use gen_services::repositories::{
    HouseholdRepository, HouseholdRepositoryError, SharedHouseholdRepository,
};
use gen_types::{Household, HouseholdId};

use crate::{
    models::{HouseholdInDb, NewHousehold, NewHouseholdMember},
    pool::DbPool,
};

pub struct SqliteHouseholdRepository {
    db_pool: DbPool,
}

impl SqliteHouseholdRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn arc_new(db_pool: DbPool) -> SharedHouseholdRepository {
        Arc::new(Self::new(db_pool))
    }
}

impl HouseholdRepository for SqliteHouseholdRepository {
    fn get_household(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, gen_services::repositories::HouseholdRepositoryError> {
        use crate::schema::households::dsl::households;
        let mut conn = self.db_pool.read().unwrap();
        let household = households
            .find(id.db_id())
            .select(HouseholdInDb::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Selecting from households failed."),
                )
            })?;
        dbg!(&household);
        if let Some(household) = household {
            Ok(serde_json::from_str(&household.body).unwrap())
        } else {
            Ok(None)
        }
    }

    fn get_all_households(
        &self,
    ) -> Result<Vec<Household>, gen_services::repositories::HouseholdRepositoryError> {
        use crate::schema::households::dsl::households;
        let mut conn = self.db_pool.read().unwrap();
        let all_households = households
            .select(HouseholdInDb::as_select())
            .load(&mut conn)
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Selecting into household_members failed."),
                )
            })?;
        let mut result = Vec::new();
        for household in all_households {
            result.push(serde_json::from_str(&household.body).unwrap());
        }
        Ok(result)
    }
    fn save_household(
        &self,
        household: &Household,
    ) -> Result<(), gen_services::repositories::HouseholdRepositoryError> {
        use crate::schema::household_members;
        use crate::schema::households;

        let id = household.id().db_id();
        let name = household.body().get_name();
        let body = serde_json::to_string(&household.body().facts()).unwrap();
        dbg!(&body);
        let new_household = NewHousehold {
            id: &id,
            name,
            body: &body,
            updated: household.updated().naive_utc(),
            updated_by: household.updated_by(),
        };
        let mut new_household_members = Vec::new();
        for member in household.body().members() {
            new_household_members.push(NewHouseholdMember {
                household_id: &id,
                person_id: member.id().db_id(),
                role: member.get_role(),
                from_date: member.get_from().map(ToString::to_string),
                to_date: member.get_to().map(ToString::to_string),
            })
        }

        let mut conn = self.db_pool.write().unwrap();
        let household = diesel::insert_into(households::table)
            .values(&new_household)
            .execute(&mut conn)
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Inserting into households failed."),
                )
            })?;
        let _ = diesel::insert_into(household_members::table)
            .values(&new_household_members)
            .execute(&mut conn)
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Inserting into household_members failed."),
                )
            })?;
        dbg!(household);
        Ok(())
    }
}
