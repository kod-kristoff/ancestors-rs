use id_ulid::{Id, Identifiable};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersonTag;

impl Identifiable for PersonTag {
    const PREFIX: &'static str = "Pers";
}

pub type PersonId = Id<PersonTag>;

#[cfg(test)]
mod tests;
