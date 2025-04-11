use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::agents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AgentInDb {
    pub id: String,
    pub body: String,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::agents)]
pub struct NewAgent<'a> {
    pub id: &'a str,
    pub body: &'a str,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: &'a str,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DocumentInDb {
    pub id: String,
    pub body: String,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::documents)]
pub struct NewDocument<'a> {
    pub id: &'a str,
    pub body: &'a str,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: &'a str,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PersonInDb {
    pub id: String,
    pub extracted: bool,
    pub body: String,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::persons)]
pub struct NewPerson<'a> {
    pub id: &'a str,
    pub extracted: bool,
    pub body: &'a str,
    pub updated: chrono::NaiveDateTime,
    pub updated_by: &'a str,
}
