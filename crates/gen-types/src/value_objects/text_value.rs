#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TextValue {
    value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
}

impl TextValue {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
    }
}

impl<S: Into<String>> From<S> for TextValue {
    fn from(value: S) -> Self {
        Self {
            value: value.into(),
            lang: None,
        }
    }
}
