use crate::GedcomxDate;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberInfo<I> {
    id: I,
    role: Option<String>,
    from: Option<GedcomxDate>,
    to: Option<GedcomxDate>,
}

impl<I> MemberInfo<I> {
    pub fn new(
        id: I,
        role: Option<String>,
        from: Option<GedcomxDate>,
        to: Option<GedcomxDate>,
    ) -> Self {
        Self { id, role, from, to }
    }

    pub fn with_id(id: I) -> Self {
        Self::new(id, None, None, None)
    }

    pub fn id(&self) -> &I {
        &self.id
    }

    pub fn role<S: Into<String>>(mut self, role: S) -> Self {
        self.set_role(Some(role.into()));
        self
    }
    pub fn set_role(&mut self, role: Option<String>) {
        self.role = role;
    }
    pub fn get_role(&self) -> Option<&str> {
        self.role.as_deref()
    }
    pub fn from<S: Into<GedcomxDate>>(mut self, from: S) -> Self {
        self.set_from(Some(from.into()));
        self
    }
    pub fn set_from(&mut self, from: Option<GedcomxDate>) {
        self.from = from;
    }
    pub fn get_from(&self) -> Option<&GedcomxDate> {
        self.from.as_ref()
    }
    pub fn to<S: Into<GedcomxDate>>(mut self, to: S) -> Self {
        self.set_to(Some(to.into()));
        self
    }
    pub fn set_to(&mut self, to: Option<GedcomxDate>) {
        self.to = to;
    }
    pub fn get_to(&self) -> Option<&GedcomxDate> {
        self.to.as_ref()
    }
}

mod serde {
    use std::{fmt, marker::PhantomData};

    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::{
        de::{self, Visitor},
        ser::SerializeStruct,
    };

    use super::MemberInfo;

    impl<I: fmt::Display> Serialize for MemberInfo<I> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("MemberInfo", 4)?;
            state.serialize_field("id", &self.id.to_string())?;
            state.serialize_field("role", &self.role)?;
            state.serialize_field("from", &self.from.as_ref().map(ToString::to_string))?;
            state.serialize_field("to", &self.to.as_ref().map(ToString::to_string))?;
            state.end()
        }
    }
    impl<'de, I: Deserialize<'de>> Deserialize<'de> for MemberInfo<I> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            enum Field {
                Id,
                Role,
                From,
                To,
            }

            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct FieldVisitor;

                    impl<'de> Visitor<'de> for FieldVisitor {
                        type Value = Field;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("`id`, `role`, `from` or `to`")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "id" => Ok(Field::Id),
                                "role" => Ok(Field::Role),
                                "from" => Ok(Field::From),
                                "to" => Ok(Field::To),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }
            struct MemberInfoVisitor<J> {
                _data: PhantomData<J>,
            }

            impl<'de, J> Visitor<'de> for MemberInfoVisitor<J>
            where
                J: Deserialize<'de>,
            {
                type Value = MemberInfo<J>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct MemberInfo")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
                {
                    let id: J = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let role = seq.next_element()?;
                    let from: Option<String> = seq.next_element()?;
                    let from = if let Some(s) = from.as_ref() {
                        Some(gedcomx_date::parse(s).map_err(|err| {
                            de::Error::custom(format!(
                                "Failed parsing GedcomX date for `from`: {err}"
                            ))
                        })?)
                    } else {
                        None
                    };
                    let to: Option<String> = seq.next_element()?;
                    let to = if let Some(s) = to.as_ref() {
                        Some(gedcomx_date::parse(s).map_err(|err| {
                            de::Error::custom(format!(
                                "Failed parsing GedcomX date for `from`: {err}"
                            ))
                        })?)
                    } else {
                        None
                    };
                    Ok(MemberInfo { id, role, from, to })
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: de::MapAccess<'de>,
                {
                    let mut id = None;
                    let mut role = None;
                    let mut from: Option<Option<String>> = None;
                    let mut to: Option<Option<String>> = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Id => {
                                if id.is_some() {
                                    return Err(de::Error::duplicate_field("id"));
                                }
                                id = Some(map.next_value()?);
                            }
                            Field::Role => {
                                if role.is_some() {
                                    return Err(de::Error::duplicate_field("role"));
                                }
                                role = Some(map.next_value()?);
                            }
                            Field::From => {
                                if from.is_some() {
                                    return Err(de::Error::duplicate_field("from"));
                                }
                                from = Some(map.next_value()?);
                            }
                            Field::To => {
                                if to.is_some() {
                                    return Err(de::Error::duplicate_field("to"));
                                }
                                to = Some(map.next_value()?);
                            }
                        }
                    }
                    let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                    // let role = role.ok_or_else(|| de::Error::missing_field("role"))?;
                    let from = if let Some(s) = from {
                        dbg!(&s);
                        if let Some(s) = s {
                            Some(gedcomx_date::parse(&s).map_err(|err| {
                                de::Error::custom(format!(
                                    "Failed parsing GedcomX date for `from`: {err}"
                                ))
                            })?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let to = if let Some(s) = to {
                        if let Some(s) = s {
                            Some(gedcomx_date::parse(&s).map_err(|err| {
                                de::Error::custom(format!(
                                    "Failed parsing GedcomX date for `to`: {err}"
                                ))
                            })?)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    Ok(MemberInfo { id, role, from, to })
                }
            }

            const FIELDS: &[&str] = &["id", "role", "from", "to"];
            deserializer.deserialize_struct(
                "MemberInfo",
                FIELDS,
                MemberInfoVisitor { _data: PhantomData },
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use gedcomx_date::Simple;

        use crate::{value_objects::MemberInfo, PersonId};

        #[test]
        fn test_serialize_deserialize_roundtrip() {
            let mi1 = MemberInfo {
                id: PersonId::default(),
                role: Some("Man".to_string()),
                from: Some(gedcomx_date::parse("+1988").unwrap()),
                to: None,
            };
            let mi1_string = serde_json::to_string(&mi1).unwrap();
            dbg!(&mi1_string);
            let mi1_copy: MemberInfo<PersonId> = serde_json::from_str(&mi1_string).unwrap();

            assert_eq!(mi1, mi1_copy);
        }
    }
}
