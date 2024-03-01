use crate::{
    value_objects::{verify_attribution_opt, Attribution},
    Agent, Document, Person, Relationship, SourceDescription,
};

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename = "batch", rename_all = "camelCase")]
pub struct Batch {
    attribution: Option<Attribution>,
    persons: Vec<Person>,
    relationships: Vec<Relationship>,
    source_descriptions: Vec<SourceDescription>,
    agents: Vec<Agent>,
    documents: Vec<Document>,
}

impl Batch {
    pub fn new() -> Self {
        Self {
            attribution: None,
            persons: Vec::new(),
            relationships: Vec::new(),
            source_descriptions: Vec::new(),
            agents: Vec::new(),
            documents: Vec::new(),
        }
    }
}

impl Batch {
    pub fn person(mut self, p: Person) -> Self {
        self.add_person(p);
        self
    }

    pub fn add_person(&mut self, p: Person) {
        self.persons.push(p);
    }

    pub fn attribution(mut self, attribution: Attribution) -> Self {
        self.set_attribution(attribution);
        self
    }

    pub fn set_attribution(&mut self, attribution: Attribution) {
        self.attribution = Some(attribution);
    }

    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.add_relationship(relationship);
        self
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    pub fn source_description(mut self, source_description: SourceDescription) -> Self {
        self.add_source_description(source_description);
        self
    }

    pub fn add_source_description(&mut self, source_description: SourceDescription) {
        self.source_descriptions.push(source_description);
    }

    pub fn agent(mut self, p: Agent) -> Self {
        self.add_agent(p);
        self
    }

    pub fn add_agent(&mut self, p: Agent) {
        self.agents.push(p);
    }

    pub fn document(mut self, p: Document) -> Self {
        self.add_document(p);
        self
    }

    pub fn add_document(&mut self, p: Document) {
        self.documents.push(p);
    }
}

impl Batch {
    pub fn persons(&self) -> &[Person] {
        self.persons.as_slice()
    }
    pub fn persons_mut(&mut self) -> &mut [Person] {
        self.persons.as_mut_slice()
    }
}

pub fn verify_batch(a: &Batch, b: &Batch) -> Result<(), String> {
    verify_attribution_opt(a.attribution.as_ref(), b.attribution.as_ref())?;
    assert_eq!(a.agents, b.agents);
    assert_eq!(a.documents, b.documents);
    for (a_person, b_person) in a.persons.iter().zip(b.persons.iter()) {
        assert_eq!(a_person, b_person);
    }
    assert_eq!(a.relationships, b.relationships);
    assert_eq!(a.source_descriptions, b.source_descriptions);
    // verify_agents(&a.agents, &b.agents)?;
    // verify_relationships(&a.relationships, &b.relationships)?;
    // verify_source_descriptions(&a.source_descriptions, &b.source_descriptions)?;
    // verify_persons(&a.persons, &b.persons)?;
    // verify_documents(&a.documents, &b.documents)?;
    Ok(())
}
#[cfg(test)]
mod tests;
