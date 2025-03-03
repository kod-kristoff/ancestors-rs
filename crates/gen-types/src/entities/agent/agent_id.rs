use id_ulid::{Id, Identifiable};

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct AgentTag;

impl Identifiable for AgentTag {
    const PREFIX: &'static str = "Agen";
}

pub type AgentId = Id<AgentTag>;
