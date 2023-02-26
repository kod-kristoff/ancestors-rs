use std::fmt::Display;

use gedcomx_model::common::{IriParseError, IriRef};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PersonId {
    inner: IriRef,
}

impl Default for PersonId {
    fn default() -> Self {
        let value = IriRef::parse(format!("p#{}", Ulid::new())).unwrap();
        Self::new(value)
    }
}
impl PersonId {
    pub fn new(value: IriRef) -> Self {
        Self { inner: value }
    }

    pub fn gen() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl Display for PersonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.inner.as_str())
    }
}
impl From<PersonId> for String {
    fn from(value: PersonId) -> Self {
        value.inner.to_string()
    }
}

impl TryFrom<String> for PersonId {
    type Error = IriParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = IriRef::parse(value)?;
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_id_is_default() {
        let pid = PersonId::default();
        assert!(pid.as_str().starts_with("p#"));
    }

    #[test]
    fn try_from_string_succeeds() {
        let pid1 = PersonId::default();
        let pid2 = PersonId::try_from(pid1.clone().to_string()).unwrap();
        assert_eq!(pid1, pid2);
    }

    #[test]
    fn try_from_string_fails() {
        let pid = PersonId::try_from("++ ++".to_string());
        assert!(pid.is_err());
    }
}
