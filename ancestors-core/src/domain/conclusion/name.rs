use crate::domain::types::name_part_type::NamePartType;

use super::name_form::NameForm;

/// A name conclusion
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    name_forms: Vec<NameForm>,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            name_forms: Vec::new(),
        }
    }
}

impl Name {
    pub fn new() -> Name {
        Self::default()
    }
}

// Builder lite
impl Name {
    pub fn name_form(mut self, name_form: NameForm) -> Self {
        self.add_name_form(name_form);
        self
    }
}

impl Name {
    pub fn add_name_form(&mut self, name_form: NameForm) {
        self.name_forms.push(name_form);
    }
    pub fn get_part(&self, part: NamePartType) -> Option<&str> {
        None
    }
    pub fn name_forms(&self) -> &[NameForm] {
        &self.name_forms
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self::new().name_form(NameForm::new().full_text(s.into()))
    }
}

#[cfg(test)]
mod tests;
