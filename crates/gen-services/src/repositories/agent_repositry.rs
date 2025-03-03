use gen_types::{Agent, AgentId};

pub trait AgentRepository {
    fn get_agent(&self, id: &AgentId) -> Result<Option<Agent>, AgentRepositoryError>;
    fn get_all_agents(&self) -> Result<Vec<Agent>, AgentRepositoryError>;
    fn save_agent(&self, agent: &Agent) -> Result<(), AgentRepositoryError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AgentRepositoryError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
