use std::fmt::{self, Debug};

use chrono::{DateTime, Utc};
use id_ulid::{Id, Identifiable};

fn utc_now() -> i64 {
    Utc::now().timestamp()
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Entity<O: Identifiable, T>
where
    T: fmt::Debug + Clone,
{
    id: Id<O>,
    body: T,
    updated: DateTime<Utc>,
    updated_by: String,
}

impl<'de, O: Identifiable, T> Entity<O, T>
where
    T: fmt::Debug + Clone + serde::Serialize + serde::Deserialize<'de>,
{
    pub fn new<S: Into<String>>(body: T, updated_by: S) -> Self {
        Self::reconstruct(Id::default(), body, Utc::now(), updated_by.into())
    }

    pub fn reconstruct(id: Id<O>, body: T, updated: DateTime<Utc>, updated_by: String) -> Self {
        Self {
            id,
            body,
            updated,
            updated_by,
        }
    }

    pub fn id(&self) -> Id<O> {
        self.id
    }

    pub fn body(&self) -> &T {
        &self.body
    }

    pub fn update_body(&mut self, updated_by: &str, update: impl FnOnce(&mut T)) {
        update(&mut self.body);
        self.stamp_user_and_time(updated_by);
    }

    pub fn stamp_user_and_time(&mut self, updated_by: &str) {
        self.updated_by = updated_by.to_string();
        self.updated = Utc::now();
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    pub fn updated_by(&self) -> &str {
        self.updated_by.as_str()
    }
}
impl<O: Identifiable, T> Default for Entity<O, T>
where
    T: fmt::Debug + Clone + Default,
{
    fn default() -> Self {
        Self {
            id: Id::default(),
            body: T::default(),
            updated: Utc::now(),
            updated_by: "default".to_string(),
        }
    }
}

impl<O: Identifiable, T> PartialEq for Entity<O, T>
where
    T: fmt::Debug + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
