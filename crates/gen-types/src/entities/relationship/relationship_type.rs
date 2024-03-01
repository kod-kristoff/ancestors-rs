/// Enumeration of standard relationship types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelationshipType {
    AncestorDescendant,
    Couple,
    EnslavedBy,
    Godparent,
    ParentChild,
    Other,
}
const QNAME_MAP: [(RelationshipType, &str); 5] = [
    (
        RelationshipType::AncestorDescendant,
        "http://gedcomx.org/AncestorDescendant",
    ),
    (RelationshipType::Couple, "http://gedcomx.org/Couple"),
    (
        RelationshipType::EnslavedBy,
        "http://gedcomx.org/EnslavedBy",
    ),
    (RelationshipType::Godparent, "http://gedcomx.org/Godparent"),
    (
        RelationshipType::ParentChild,
        "http://gedcomx.org/ParentChild",
    ),
];
impl RelationshipType {
    pub fn as_qname_uri(&self) -> &str {
        match self {
            Self::AncestorDescendant => "http://gedcomx.org/AncestorDescendant",
            Self::Couple => "http://gedcomx.org/Couple",
            Self::EnslavedBy => "http://gedcomx.org/EnslavedBy",
            Self::Godparent => "http://gedcomx.org/Godparent",
            Self::ParentChild => "http://gedcomx.org/ParentChild",
            Self::Other => "OTHER",
        }
    }
    pub fn from_qname_uri(qname_uri: &str) -> Self {
        for (relationship, qname) in QNAME_MAP {
            if qname == qname_uri {
                return relationship;
            }
        }
        Self::Other
    }
}

impl serde::Serialize for RelationshipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // use serde::ser::SerializeMap;
        // let mut map = serializer.serialize_map(Some(1))?;
        // map.serialize_entry("type", self.as_qname_uri())?;
        // map.end()
        serializer.serialize_str(self.as_qname_uri())
    }
}

impl<'de> serde::Deserialize<'de> for RelationshipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RelationshipTypeVisitor)
    }
}

struct RelationshipTypeVisitor;

impl<'de> serde::de::Visitor<'de> for RelationshipTypeVisitor {
    type Value = RelationshipType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an uri")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RelationshipType::from_qname_uri(v))
    }
}
