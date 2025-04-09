use gen_services::{
    repositories::{
        mem::{InMemoryAgentRepo, InMemoryPersonRepo},
        SharedAgentRepository, SharedPersonRepository,
    },
    service::Service,
};

#[derive(Clone)]
pub struct TestContext {
    pub service: Service,
    pub agent_repo: SharedAgentRepository,
    pub person_repo: SharedPersonRepository,
}

impl Default for TestContext {
    fn default() -> Self {
        let agent_repo = InMemoryAgentRepo::arc_new();
        let person_repo = InMemoryPersonRepo::arc_new();
        let service = Service::new(agent_repo.clone(), person_repo.clone());
        TestContext {
            service,
            agent_repo,
            person_repo,
        }
    }
}
