use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PersonInDb {
    pub id: String,
    pub extracted: bool,
    pub body: String,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub updated_by: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::persons)]
pub struct NewPerson<'a> {
    pub id: &'a str,
    pub extracted: bool,
    pub body: &'a str,
    pub updated: &'a chrono::DateTime<chrono::Utc>,
    pub updated_by: &'a str,
}
