use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};

use ulid::Ulid;

use crate::{IdError, Identifiable};

pub struct Id<O: Identifiable> {
    id: Ulid,
    _marker: PhantomData<O>,
}

impl<O: Identifiable> Id<O> {
    /// Return the internal ULID
    pub fn ulid(&self) -> Ulid {
        self.id
    }
    /// Return the database identifier as a string.
    ///
    /// This method should rarely be useful as Id can directly
    /// be written in database using sqlx without going through
    /// a string representation.
    pub fn db_id(&self) -> String {
        self.id.to_string()
    }
    /// Return the public representation as a string, which should
    /// be used in JSON, URL, or anywhere except the database.
    pub fn public_id(&self) -> String {
        self.to_string()
    }
    /// Parse an Id from its public representation, checking the class
    pub fn from_public_id(public_id: &str) -> Result<Self, IdError> {
        let db_id = public_id
            .trim_start_matches(O::PREFIX)
            .trim_start_matches('_');
        // let db_id = <O as Identifiable>::PREFIX.strip_prefix(public_id)?;
        let id: Ulid = db_id.parse().map_err(|_| IdError::InvalidFormat)?;
        Ok(Self::unchecked(id))
    }
    /// Parse the Id from its database string representation, *not* checking
    /// the class (as it's not embedded in this representation)
    pub fn from_db_id(db_id: &str) -> Result<Self, IdError> {
        let id: Ulid = db_id.parse().map_err(|_| IdError::InvalidFormat)?;
        Ok(Self::unchecked(id))
    }
    /// Build an Id without checking the class
    pub(crate) fn unchecked(id: Ulid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}
impl<O: Identifiable> Default for Id<O> {
    fn default() -> Self {
        Self {
            id: Ulid::new(),
            _marker: PhantomData,
        }
    }
}

impl<O: Identifiable> Clone for Id<O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<O: Identifiable> Copy for Id<O> {}

impl<O: Identifiable> PartialEq for Id<O> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<O: Identifiable> Eq for Id<O> {}

impl<O: Identifiable> fmt::Display for Id<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", O::PREFIX, self.id)
    }
}

impl<O: Identifiable> fmt::Debug for Id<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Id")
            .field("id", &self.id)
            .field("prefix", &O::PREFIX)
            .finish()
    }
}

/// Parse an Id from its public representation, checking the class
impl<O: Identifiable> FromStr for Id<O> {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_public_id(s)
    }
}

impl<O: Identifiable> Hash for Id<O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
