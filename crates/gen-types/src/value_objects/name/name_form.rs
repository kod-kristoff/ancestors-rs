/// A name form conclusion
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameForm {
    full_text: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    lang: String,
}

impl NameForm {
    pub fn new() -> Self {
        Self::default()
    }
}

// Builder lite
impl NameForm {
    pub fn full_text(mut self, full_text: String) -> Self {
        self.full_text = full_text;
        self
    }

    pub fn lang(mut self, lang: String) -> Self {
        self.lang = lang;
        self
    }
}

impl NameForm {
    pub fn get_full_text(&self) -> &str {
        &self.full_text
    }
}

#[cfg(test)]
mod tests;
