use super::{Id, Identifiable};

use ::serde::{de, Deserialize, Deserializer, Serialize, Serializer};

impl<O: Identifiable> Serialize for Id<O> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.public_id())
    }
}

impl<'de, O: Identifiable> Deserialize<'de> for Id<O> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_public_id(&s).map_err(de::Error::custom)
    }
}

pub fn deserialize_raw<'de, O: Identifiable, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Id<O>, D::Error> {
    let s = String::deserialize(deserializer)?;
    Id::from_db_id(&s).map_err(de::Error::custom)
}
