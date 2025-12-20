use gen_services::{
    repositories::{
        SharedAgentRepository, SharedDocumentRepository, SharedHouseholdRepository,
        SharedPersonRepository,
        mem::{InMemoryAgentRepo, InMemoryDocumentRepo, InMemoryHouseholdRepo, InMemoryPersonRepo},
    },
    service::Service,
};

#[derive(Clone)]
pub struct TestContext {
    pub service: Service,
    pub agent_repo: SharedAgentRepository,
    pub document_repo: SharedDocumentRepository,
    pub household_repo: SharedHouseholdRepository,
    pub person_repo: SharedPersonRepository,
}

impl Default for TestContext {
    fn default() -> Self {
        let agent_repo = InMemoryAgentRepo::arc_new();
        let document_repo = InMemoryDocumentRepo::arc_new();
        let household_repo = InMemoryHouseholdRepo::arc_new();
        let person_repo = InMemoryPersonRepo::arc_new();
        let service = Service::new(
            agent_repo.clone(),
            document_repo.clone(),
            household_repo.clone(),
            person_repo.clone(),
        );
        TestContext {
            service,
            agent_repo,
            document_repo,
            household_repo,
            person_repo,
        }
    }
}
