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
mod tests {
    use super::*;

    mod get_part {
        use super::*;
        #[test]
        fn no_forms() {
            let name = Name::new();
            assert!(name.get_part(NamePartType::Given).is_none());
            assert!(name.get_part(NamePartType::Surname).is_none());
        }

        // #[test]
        // fn null_form() {
        //     let name = Name::new().name_form(None);
        //     assert!(name.get_part(NamePartType::Given).is_none());
        //     assert!(name.get_part(NamePartType::Surname).is_none());
        // }

        #[test]
        fn form_no_parts() {
            let name_form = NameForm::new()
                .full_text("John Fitzgerald Kennedy".into())
                .lang("en".into());
            let name = Name::new().name_form(name_form);
            assert!(name.get_part(NamePartType::Given).is_none());
            assert!(name.get_part(NamePartType::Surname).is_none());
        }
        // NameForm nameForm = new NameForm("John Fitzgerald Kennedy")
        //   .lang("en")
        //   .part(NamePartType.Given, "John")
        //   .part(NamePartType.Given, "Fitzgerald")
        //   .part(NamePartType.Surname, "Kennedy");
        // Name name = new Name().nameForm(nameForm);
        // assertEquals("John", name.getPart(NamePartType.Given));
        // assertEquals("Kennedy", name.getPart(NamePartType.Surname));

        // assertNull(nameNoParts.getPart(NamePartType.Given));
        // assertNull(nameNoParts.getPart(NamePartType.Surname));

        // NameForm nameFormNullParts = new NameForm("John Fitzgerald Kennedy")
        //   .lang("en")
        //   .part(NamePartType.Given, null)
        //   .part(NamePartType.Surname, null);
        // Name nameNullParts = new Name().nameForm(nameFormNullParts);
        // assertNull(nameNullParts.getPart(NamePartType.Given));
        // assertNull(nameNullParts.getPart(NamePartType.Surname));
    }
}
