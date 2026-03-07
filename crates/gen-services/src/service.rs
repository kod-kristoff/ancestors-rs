use gen_types::{entities::PersonBody, Person};

use crate::{
    repositories::{
        SharedAgentRepository, SharedDocumentRepository, SharedHouseholdRepository,
        SharedPersonRepository,
    },
    services::{
        household_service::AddHouseholdError, person_service::AddPersonError, AddAgentError,
        AddDocumentError, AddPerson, AgentService, DocumentService, GenService, HouseholdService,
        PersonService,
    },
};

#[derive(Clone)]
pub struct Service {
    agent_repo: SharedAgentRepository,
    document_repo: SharedDocumentRepository,
    household_repo: SharedHouseholdRepository,
    person_repo: SharedPersonRepository,
}

impl Service {
    pub fn new(
        agent_repo: SharedAgentRepository,
        document_repo: SharedDocumentRepository,
        household_repo: SharedHouseholdRepository,
        person_repo: SharedPersonRepository,
    ) -> Self {
        Self {
            agent_repo,
            document_repo,
            household_repo,
            person_repo,
        }
    }
}

impl PersonService for Service {
    fn add_person(&self, user: &str, cmd: &AddPerson) -> Result<Person, AddPersonError> {
        let mut person = PersonBody::default();
        if let Some(name) = &cmd.name {
            person = person.name(name.as_str());
        }
        // person.set_extracted(cmd.extracted);
        let person = Person::new(person, user);
        self.person_repo.save_person(&person).unwrap();
        Ok(person)
    }

    fn add_person_raw(&self, user: &str, mut person: Person) -> Result<Person, AddPersonError> {
        person.stamp_user_and_time(user);
        self.person_repo
            .save_person(&person)
            .map_err(|err| AddPersonError::Unknown(err.into()))?;
        Ok(person)
    }

    // pub fn edit(&self, cmd: &EditPerson) -> UseCaseResult<()> {
    //     // let person = Person::new(cmd.id);
    //     // if let Some(name) = &cmd.name {
    //     //     person = person.name(name.as_str());
    //     // }
    //     // person.set_extracted(cmd.extracted);
    //     // self.repo.save(person).unwrap();
    //     Ok(())
    // }
}

impl AgentService for Service {
    fn add_agent_raw(
        &self,
        user: &str,
        mut agent: gen_types::Agent,
    ) -> Result<gen_types::Agent, crate::services::AddAgentError> {
        agent.stamp_user_and_time(user);
        self.agent_repo
            .save_agent(&agent)
            .map_err(|err| AddAgentError::Unknown(err.into()))?;
        Ok(agent)
    }
}

impl DocumentService for Service {
    fn add_document_raw(
        &self,
        user: &str,
        mut document: gen_types::Document,
    ) -> Result<gen_types::Document, crate::services::AddDocumentError> {
        document.stamp_user_and_time(user);
        self.document_repo
            .save_document(&document)
            .map_err(|err| AddDocumentError::Unknown(err.into()))?;
        Ok(document)
    }
}

impl HouseholdService for Service {
    fn add_household_raw(
        &self,
        user: &str,
        mut household: gen_types::Household,
    ) -> Result<gen_types::Household, crate::services::AddHouseholdError> {
        household.stamp_user_and_time(user);
        self.household_repo
            .save_household(&household)
            .map_err(|err| AddHouseholdError::Unknown(err.into()))?;
        Ok(household)
    }
}

impl GenService for Service {}
