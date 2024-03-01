use crate::common::emma_bocock_example;

mod persons {
    use gen_types::value_objects::FactType;

    use super::*;
    #[test]
    fn test_emma() -> eyre::Result<()> {
        let gedcom = emma_bocock_example()?;
        let emma = &gedcom.persons()[0];
        assert_eq!(
            emma.names()[0].name_forms()[0].get_full_text(),
            "Emma Bocock"
        );
        assert!(emma.is_extracted());
        assert_eq!(emma.facts()[0].r#type(), FactType::Birth);
        let emma_conclusion = &gedcom.persons()[3];
        assert_eq!(emma_conclusion.evidences()[0], emma.id());
        assert!(!emma_conclusion.is_extracted());
        Ok(())
    }

    #[test]
    fn search_person_by_key() -> eyre::Result<()> {
        let batch = emma_bocock_example()?;
        let emma = batch
            .persons()
            .iter()
            .find(|p| {
                p.identifiers()
                    .iter()
                    .any(|i| i.namespace() == Some("http://gedcomx.org") && i.id() == Some("#P-1"))
            })
            .unwrap();
        assert_eq!(
            emma.names()[0].name_forms()[0].get_full_text(),
            "Emma Bocock"
        );
        Ok(())
    }
}
