use std::fmt::Display;

use crate::shared_kernel::{IriParseError, IriRef};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PersonId {
    pub value: IriRef,
}

impl Default for PersonId {
    fn default() -> Self {
        let value = IriRef::parse(format!("p#{}", Ulid::new())).unwrap();
        Self::new(value)
    }
}
impl PersonId {
    pub fn new(value: IriRef) -> Self {
        Self { value }
    }

    pub fn gen() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl Display for PersonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.as_str())
    }
}
impl From<PersonId> for String {
    fn from(value: PersonId) -> Self {
        value.value.to_string()
    }
}

impl From<&IriRef> for PersonId {
    fn from(value: &IriRef) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

impl TryFrom<String> for PersonId {
    type Error = IriParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = IriRef::parse(value)?;
        Ok(Self { value: inner })
    }
}

#[cfg(test)]
mod tests;
