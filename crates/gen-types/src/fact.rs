mod fact_type;

use crate::Date;
use crate::PlaceReference;
pub use fact_type::FactType;

// #[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fact {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    value: String,
    r#type: FactType,
    date: Option<Date>,
    place: Option<PlaceReference>,
}

impl Fact {
    pub fn new(fact_type: FactType) -> Self {
        Self {
            r#type: fact_type,
            date: None,
            place: None,
            value: String::new(),
        }
    }
}

impl Fact {
    pub fn date(mut self, date: Date) -> Self {
        self.set_date(date);
        self
    }

    pub fn place(mut self, place: PlaceReference) -> Self {
        self.set_place(place);
        self
    }

    pub fn value<S: Into<String>>(mut self, s: S) -> Self {
        self.set_value(s.into());
        self
    }
}

impl Fact {
    pub fn set_date(&mut self, date: Date) {
        self.date = Some(date);
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn set_place(&mut self, place: PlaceReference) {
        self.place = Some(place);
    }

    pub fn r#type(&self) -> FactType {
        self.r#type
    }
}
