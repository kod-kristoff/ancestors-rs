use gen_types::Agent;

pub trait AgentService {
    // fn add_agent(&self, user: &str, agent: &AddAgent) -> Result<Agent, AddAgentError>;
    fn add_agent_raw(&self, user: &str, agent: Agent) -> Result<Agent, AddAgentError>;
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddAgentError {
    #[error("Unknown error")]
    #[diagnostic(transparent)]
    Unknown(miette::Report),
}
