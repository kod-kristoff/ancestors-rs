use chrono::{DateTime, LocalResult, Utc};
use serde::{Deserialize, Deserializer, Serializer};

use crate::{
    entities::AgentReference,
    shared::Iri,
    value_objects::{ResourceType, TextValue},
};

use super::{SourceCitation, SourceId, SourceReference};

// use super::{SourceCitation, SourceReference};
// use crate::{
//     common::{DateTime, IriRef, ResourceReference, TextValue, Uri},
//     types::ResourceType,
// };

pub fn ser_opt_date<S: Serializer>(v: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        // Some(d) => s.serialize_i64(DateTime::from_utc(d.and_hms(0, 0, 0), Utc).timestamp()),
        Some(d) => s.serialize_i64(d.timestamp_millis()),
        None => s.serialize_unit(),
    }
}

pub fn deserialize_optional_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionalDateTimeVisitor)
}

struct OptionalDateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for OptionalDateTimeVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("optional timestamp")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        use chrono::TimeZone;
        match Utc.timestamp_millis_opt(i64::deserialize(deserializer)?) {
            LocalResult::Single(dt) => Ok(Some(dt)),
            LocalResult::None => todo!("handle error"),
            LocalResult::Ambiguous(_dt1, _dt2) => todo!("handle ambiguous"),
        }
        // Ok(Some(Utc.timestamp_millis(i64::deserialize(deserializer)?)))
    }
}

// #[serdxce_as]
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    resource_type: Option<Iri>,
    citations: Vec<SourceCitation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    names: Vec<TextValue>,
    titles: Vec<TextValue>,
    #[serde(
        serialize_with = "ser_opt_date",
        deserialize_with = "deserialize_optional_datetime"
    )]
    // #[serde_as(as = "TimestampMilliSeconds<i64>")]
    created: Option<DateTime<Utc>>,
    repository: Option<AgentReference>,
    // #[serde(default)]
    id: SourceId,
}

impl SourceDescription {
    pub fn new(id: SourceId) -> Self {
        Self {
            id,
            names: Vec::new(),
            titles: Vec::new(),
            citations: Vec::new(),
            repository: None,
            created: None,
            resource_type: None,
        }
    }
}

impl SourceDescription {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }

    pub fn repository<S: Into<AgentReference>>(mut self, repository: S) -> Self {
        self.set_repository(repository.into());
        self
    }

    pub fn created<S: Into<DateTime<Utc>>>(mut self, created: S) -> Self {
        self.set_created(created.into());
        self
    }

    pub fn resource_type(mut self, resource_type: ResourceType) -> Self {
        self.set_resource_type(resource_type.into());
        self
    }

    pub fn citation<T: Into<SourceCitation>>(mut self, citation: T) -> Self {
        self.add_citation(citation.into());
        self
    }

    pub fn name<T: Into<TextValue>>(mut self, name: T) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn title<T: Into<TextValue>>(mut self, title: T) -> Self {
        self.add_title(title.into());
        self
    }
}
impl SourceDescription {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn set_repository(&mut self, repository: AgentReference) {
        self.repository = Some(repository);
    }
    pub fn set_created(&mut self, created: DateTime<Utc>) {
        self.created = Some(created);
    }
    pub fn set_resource_type(&mut self, resource_type: Iri) {
        self.resource_type = Some(resource_type);
    }
    // pub fn get_id(&self) -> &str {
    //     self.id.as_str()
    // }
    pub fn add_title(&mut self, title: TextValue) {
        self.titles.push(title);
    }
    pub fn add_citation(&mut self, citation: SourceCitation) {
        self.citations.push(citation);
    }
    pub fn add_name(&mut self, name: TextValue) {
        self.names.push(name);
    }
}

// impl From<&SourceDescription> for ResourceReference {
//     fn from(source: &SourceDescription) -> Self {
//         ResourceReference::new(source.id.to_iri_ref())
//     }
// }

impl From<&SourceDescription> for SourceReference {
    fn from(source: &SourceDescription) -> Self {
        Self::new(source.id) //, source.id.clone().into_inner())
    }
}
