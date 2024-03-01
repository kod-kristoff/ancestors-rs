use core::fmt;

/// Enumeration of standard fact types.
#[derive(Debug, Clone, Copy, PartialEq)] //, serde::Deserialize)] //, serde::Serialize)]
pub enum FactType {
    /// A fact of a person's birth.
    Birth,
    /// A fact of a record of a person's living for a specific period. This is designed to include "flourish", defined to mean
    /// the time period
    /// in an adult's life where he was most productive, perhaps
    /// as a writer or member of the state assembly. It does not reflect the person's birth and death dates.
    Living,
    /// A fact of a person's occupation or employment.
    Occupation,
}

impl FactType {
    pub fn as_qname_uri(&self) -> &str {
        match self {
            Self::Birth => "http://gedcomx.org/Birth",
            Self::Living => "http://gedcomx.org/Living",
            Self::Occupation => "http://gedcomx.org/Occupation",
            // Self::Intersex => "http://gedcomx.org/Intersex",
            // Self::Unknown => "http://gedcomx.org/Unknown",
        }
    }
    pub fn from_qname_uri(qname_uri: &str) -> Self {
        match qname_uri {
            "http://gedcomx.org/Birth" => Self::Birth,
            "http://gedcomx.org/Living" => Self::Living,
            "http://gedcomx.org/Occupation" => Self::Occupation,
            // Self::Intersex => "http://gedcomx.org/Intersex",
            // Self::Unknown => "http://gedcomx.org/Unknown",
            _ => todo!("handle qname_uri='{}'", qname_uri),
        }
    }
}

impl serde::Serialize for FactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_qname_uri())
    }
}

impl<'de> serde::Deserialize<'de> for FactType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FactTypeVisitor)
    }
}

struct FactTypeVisitor;

impl<'de> serde::de::Visitor<'de> for FactTypeVisitor {
    type Value = FactType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a uri")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FactType::from_qname_uri(value))
    }
}
