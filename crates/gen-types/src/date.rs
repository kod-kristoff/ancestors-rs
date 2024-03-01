/// A name conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Date {
    original: String,
}

impl Date {
    pub fn new() -> Self {
        Self {
            original: String::new(),
        }
    }
}

// Builder lite
impl Date {
    pub fn original<S: Into<String>>(mut self, original: S) -> Self {
        self.set_original(original.into());
        self
    }
}

impl Date {
    pub fn set_original(&mut self, original: String) {
        self.original = original;
    }
    pub fn get_original(&self) -> &str {
        self.original.as_str()
    }
}

impl<S: Into<String>> From<S> for Date {
    fn from(s: S) -> Self {
        Date::new().original(s)
    }
}
