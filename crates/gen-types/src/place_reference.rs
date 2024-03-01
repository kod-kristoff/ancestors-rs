/// A place conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlaceReference {
    original: String,
}

impl PlaceReference {
    pub fn new() -> Self {
        Self {
            original: String::new(),
        }
    }
}

// Builder lite
impl PlaceReference {
    pub fn original<S: Into<String>>(mut self, original: S) -> Self {
        self.set_original(original.into());
        self
    }
}

impl PlaceReference {
    pub fn set_original(&mut self, original: String) {
        self.original = original;
    }
}
