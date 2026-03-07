pub use self::place_id::PlaceId;
pub type PlaceReference = IdReference<PlaceId>;
pub type Place = Entity<PlaceTag, PlaceBody>;

use self::place_id::PlaceTag;
use super::shared::Entity;
use crate::shared::IdReference;

mod place_id;

/// A place conclusion
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaceBody {
    original: String,
}

impl PlaceBody {
    pub fn new() -> Self {
        Self::default()
    }
}

// Builder lite
impl PlaceBody {
    pub fn original<S: Into<String>>(mut self, original: S) -> Self {
        self.set_original(original.into());
        self
    }
}

impl PlaceBody {
    pub fn set_original(&mut self, original: String) {
        self.original = original;
    }
}

impl From<&Place> for PlaceReference {
    fn from(value: &Place) -> Self {
        Self::new(value.id())
    }
}
