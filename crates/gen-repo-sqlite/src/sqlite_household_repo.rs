use std::{str::FromStr, sync::Arc};

use chrono::NaiveDateTime;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use gen_services::repositories::{
    HouseholdRepository, HouseholdRepositoryError, SharedHouseholdRepository,
};
use gen_types::{
    entities::HouseholdBody,
    gedcomx_date_from_str,
    value_objects::{Fact, MemberInfo},
    GedcomxDate, Household, HouseholdId, PersonId,
};

use crate::{
    models::{HouseholdInDb, HouseholdMemberInDb, NewHousehold, NewHouseholdMember},
    pool::{DbConnection, DbPool},
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

    fn load_household(
        &self,
        HouseholdInDb {
            id,
            name,
            body,
            updated,
            updated_by,
        }: HouseholdInDb,
        conn: &mut DbConnection,
    ) -> Result<Household, HouseholdRepositoryError> {
        use crate::schema::household_members;
        let members_in_db: Vec<HouseholdMemberInDb> = household_members::table
            .filter(household_members::household_id.eq(&id))
            .load(conn)
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Selecting from household_members failed."),
                )
            })?;
        let mut members = Vec::with_capacity(members_in_db.len());
        for member_in_db in members_in_db {
            let HouseholdMemberInDb {
                household_id,
                person_id,
                role,
                from_date,
                to_date,
            } = member_in_db;
            let person_id = PersonId::from_db_id(&person_id).unwrap();
            let from = from_date
                .as_ref()
                .map(|date| gedcomx_date_from_str(date).unwrap());
            let to = to_date
                .as_ref()
                .map(|date| gedcomx_date_from_str(date).unwrap());
            let member = MemberInfo::new(person_id, role, from, to);
            members.push(member);
        }
        let facts: Vec<Fact> = serde_json::from_str(&body).unwrap();

        let body = HouseholdBody::new(name, members, facts);
        let id = HouseholdId::from_db_id(&id).unwrap();
        let updated = updated.and_utc();
        let household = Household::reconstruct(id, body, updated, updated_by);
        Ok(household)
    }
}

impl HouseholdRepository for SqliteHouseholdRepository {
    fn get_household(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, gen_services::repositories::HouseholdRepositoryError> {
        use crate::schema::households;

        let mut conn = self.db_pool.read().unwrap();
        let household = households::table
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
        if let Some(household_in_db) = household {
            let household = self.load_household(household_in_db, &mut conn)?;
            Ok(Some(household))
        } else {
            Ok(None)
        }
    }

    fn get_all_households(
        &self,
    ) -> Result<Vec<Household>, gen_services::repositories::HouseholdRepositoryError> {
        use crate::schema::households::dsl::households;
        let mut conn = self.db_pool.read().unwrap();
        let households_in_db = households
            .select(HouseholdInDb::as_select())
            .load(&mut conn)
            .map_err(|err| {
                HouseholdRepositoryError::Unknown(
                    miette::Report::msg(err).wrap_err("Selecting into household_members failed."),
                )
            })?;
        let mut result = Vec::with_capacity(households_in_db.len());
        for household_in_db in households_in_db {
            let household = self.load_household(household_in_db, &mut conn)?;
            result.push(household);
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
        let facts = serde_json::to_string(&household.body().facts()).unwrap();
        dbg!(&facts);
        let new_household = NewHousehold {
            id: &id,
            name,
            body: &facts,
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
