use crate::shared::IriRef;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
    r#type: IdentifierType,
    raw: IriRef,
    namespace: Option<IriRef>,
    id: Option<IriRef>,
}

impl Identifier {
    pub fn new(r#type: IdentifierType, raw: IriRef) -> Self {
        Self {
            r#type,
            raw,
            namespace: None,
            id: None,
        }
    }

    pub fn set_namespace(&mut self, namespace: IriRef) {
        self.namespace = Some(namespace);
    }

    pub fn set_id(&mut self, id: IriRef) {
        self.id = Some(id);
    }
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(IriRef::as_str)
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(IriRef::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IdentifierType {
    Primary,
}
