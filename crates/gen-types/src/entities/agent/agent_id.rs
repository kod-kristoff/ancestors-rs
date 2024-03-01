use id_ulid::{Id, Identifiable};

pub struct AgentTag;

impl Identifiable for AgentTag {
    const PREFIX: &'static str = "Agen";
}

pub type AgentId = Id<AgentTag>;
