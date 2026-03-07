use std::fmt;

pub use agent_id::AgentId;
use agent_id::AgentTag;
pub type AgentReference = IdReference<AgentId>;
pub type Agent = Entity<AgentTag, AgentBody>;

use crate::{
    shared::{IdReference, IriRef},
    value_objects::{ResourceReference, TextValue},
    Result,
};

use super::shared::Entity;

mod agent_id;

#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AgentBody {
    names: Vec<TextValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    emails: Vec<ResourceReference>,
}

impl AgentBody {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            emails: Vec::new(),
        }
    }
}

impl AgentBody {
    // pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    //     self.set_id(id.into());
    //     self
    // }

    pub fn name<T: Into<TextValue>>(mut self, name: T) -> Self {
        self.add_name(name.into());
        self
    }

    pub fn email(mut self, email: IriRef) -> Self {
        self.add_email(ResourceReference::new(email));
        self
    }
    pub fn try_email(mut self, email: impl fmt::Display) -> Result<Self> {
        self.add_email(ResourceReference::new(IriRef::parse(format!(
            "mailto:{}",
            email
        ))?));
        Ok(self)
    }
}
impl AgentBody {
    // pub fn set_id(&mut self, id: String) {
    //     self.id = id;
    // }
    pub fn add_name(&mut self, name: TextValue) {
        self.names.push(name);
    }
    pub fn add_email(&mut self, email: ResourceReference) {
        self.emails.push(email);
    }
    // pub fn get_id(&self) -> &str {
    //     self.id.as_str()
    // }
    pub fn names(&self) -> &[TextValue] {
        self.names.as_slice()
    }
    pub fn emails(&self) -> &[ResourceReference] {
        self.emails.as_slice()
    }
}

// impl From<&Agent> for ResourceReference {
//     fn from(agent: &Agent) -> Self {
//         ResourceReference::new(agent.id.clone())
//     }
// }
impl From<&Agent> for AgentReference {
    fn from(value: &Agent) -> Self {
        Self::new(value.id())
    }
}

pub fn verify_agents(aas: &[Agent], bs: &[Agent]) -> std::result::Result<(), String> {
    if aas.len() != bs.len() {
        return Err(format!("length mismatch: {} != {}", aas.len(), bs.len()));
    }
    todo!()
}
