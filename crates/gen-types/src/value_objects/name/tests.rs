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
