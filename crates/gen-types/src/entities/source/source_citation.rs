#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SourceCitation {
    value: String,
    lang: Option<String>,
}

impl SourceCitation {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SourceCitation {
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.set_value(value.into());
        self
    }
}

impl SourceCitation {
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<S: Into<String>> From<S> for SourceCitation {
    fn from(value: S) -> Self {
        Self {
            value: value.into(),
            lang: None,
        }
    }
}
