use std::{
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use super::IriRef;

pub struct IdReference<I>(I);

impl<I> IdReference<I> {
    pub fn new(i: I) -> Self {
        Self(i)
    }
}

impl<I: Default> Default for IdReference<I> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<I: fmt::Display> IdReference<I> {
    pub fn to_iri_ref(&self) -> IriRef {
        IriRef::parse(self.to_string()).unwrap()
    }
}

impl<I: Clone> Clone for IdReference<I> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I: Copy> Copy for IdReference<I> {}

impl<I: PartialEq> PartialEq for IdReference<I> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I: PartialEq> PartialEq<I> for IdReference<I> {
    fn eq(&self, other: &I) -> bool {
        self.0 == *other
    }
}
impl<I: PartialEq> PartialEq<&I> for IdReference<I> {
    fn eq(&self, other: &&I) -> bool {
        self.0 == **other
    }
}
impl<I: Eq> Eq for IdReference<I> {}

impl<I: fmt::Display> fmt::Display for IdReference<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl<I: fmt::Debug> fmt::Debug for IdReference<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdReference").field("id", &self.0).finish()
    }
}

/// Parse an Id from its public representation, checking the class
impl<I: FromStr> FromStr for IdReference<I> {
    type Err = I::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(I::from_str(s)?))
    }
}

impl<I: Hash> Hash for IdReference<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub mod serde {

    use std::fmt;

    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::IdReference;

    impl<I: fmt::Display> Serialize for IdReference<I> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de, I: Deserialize<'de>> Deserialize<'de> for IdReference<I> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = I::deserialize(deserializer)?;
            Ok(Self(s))
        }
    }
}
