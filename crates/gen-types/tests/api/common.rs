// use gen_types::agent::Agent;
// use gen_types::common::{DateTime, IriRef};
// use gen_types::conclusion::{Date, Document, Fact, Person, PlaceReference, Relationship};
// use gen_types::gedcomx::Attribution;
// use gen_types::source::{SourceCitation, SourceDescription};
// use gen_types::types::{FactType, Gender, RelationshipType, ResourceType};
// use gen_types::GedcomX;

use chrono::{DateTime, Utc};
use gen_types::{
    value_objects::{
        Attribution, Date, Fact, FactType, Gender, Identifier, IdentifierType, ResourceType,
    },
    Agent, Batch, Document, Person, PlaceReference, Relationship, RelationshipType, SourceCitation,
    SourceDescription,
};

// pub fn iri(s: &str) -> IriRef {
//     IriRef::parse(s.into()).expect("parse iri")
// }
pub fn emma_bocock_example() -> eyre::Result<Batch> {
    let contributor = Agent::default()
        .name("Jane Doe")
        .try_email("example@example.org")
        .expect("email");
    let repository = Agent::default().name("General Registry Office, Southport");
    let attribution = Attribution::new().contributor(&contributor).modified(
        "2014-03-07T00:00:00-07:00"
            .parse::<DateTime<Utc>>()
            .expect("failed parsing date"),
    );
    let source_description = SourceDescription::default()
    //     iri("#S-1"))
    .title("Birth Certificate of Emma Bocock, 23 July 1843, General Registry Office")
      .citation(SourceCitation::new().value("England, birth certificate for Emma Bocock, born 23 July 1843; citing 1843 Birth in District and Sub-district of Ecclesall-Bierlow in the County of York, 303; General Registry Office, Southport."))
      .resource_type(ResourceType::PhysicalArtifact)
      .created("1843-07-27T00:00:00-07:00".parse::<DateTime<Utc>>().expect("failed"))
      .repository(&repository);
    let birth = Fact::new(FactType::Birth)
        .date(Date::new().original("23 June 1843"))
        .place(PlaceReference::new().original(
            "Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United Kingdom",
        ));
    let mut emma_ident = Identifier::new(
        IdentifierType::Primary,
        "http://gedcomx.org/example#P-1".parse()?,
    );
    emma_ident.set_namespace("http://gedcomx.org".parse()?);
    emma_ident.set_id("#P-1".parse()?);
    let emma = Person::default()
        // .extracted(true)
        .source(&source_description)
        .name("Emma Bocock")
        .gender(Gender::Female)
        .identifier(emma_ident)
        .fact(birth);
    let father = Person::default() //new(iri("#P-2"))
        // .extracted(true)
        .source(&source_description)
        .name("William Bocock")
        .fact(Fact::new(FactType::Occupation).value("Toll Collector"));
    let mother = Person::default() //:new(iri("#P-3"))
        // .extracted(true)
        .source(&source_description)
        .name("Sarah Bocock formerly Brough");
    let father_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&father)
        .person2(&emma);

    let mother_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&mother)
        .person2(&emma);

    let analysis = Document::default().text("...Jane Doe's analysis document...");
    let emma_conclusion = Person::default().evidence(&emma).analysis(&analysis);
    // // GedcomX::new()
    Ok(Batch::new()
        .agent(contributor)
        .agent(repository)
        .attribution(attribution)
        .source_description(source_description)
        .person(emma)
        .person(father)
        .person(mother)
        .relationship(father_relationship)
        .relationship(mother_relationship)
        .document(analysis)
        .person(emma_conclusion))
}
