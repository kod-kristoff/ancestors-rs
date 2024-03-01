/// Enumeration of known gender types.
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Gender {
    /// Male.
    Male,
    /// Female.
    #[serde(rename = "http://gedcomx.org/Female")]
    Female,
    /// Intersex.
    Intersex,
    /// Unknown. Note that this should be used strictly as "unknown" and not to
    /// indicate a type that is not set or not understood.
    Unknown,
}
const QNAME_MAP: [(Gender, &str); 4] = [
    (Gender::Female, "http://gedcomx.org/Female"),
    (Gender::Male, "http://gedcomx.org/Male"),
    (Gender::Intersex, "http://gedcomx.org/Intersex"),
    (Gender::Unknown, "http://gedcomx.org/Unknown"),
];
impl Gender {
    pub fn as_qname_uri(&self) -> &str {
        match self {
            Self::Female => "http://gedcomx.org/Female",
            Self::Male => "http://gedcomx.org/Male",
            Self::Intersex => "http://gedcomx.org/Intersex",
            Self::Unknown => "http://gedcomx.org/Unknown",
        }
    }
    pub fn from_qname_uri(qname: &str) -> Self {
        for (gender, qname_uri) in QNAME_MAP {
            if qname == qname_uri {
                return gender;
            }
        }
        Self::Unknown
    }
}

impl serde::Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("type", self.as_qname_uri())?;
        map.end()
    }
}
