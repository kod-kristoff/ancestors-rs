use crate::shared::IriRef;
use crate::Result;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Reference {
    resource: IriRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
}

impl Reference {
    pub fn new(resource: IriRef) -> Self {
        Self {
            resource,
            resource_id: None,
        }
    }
    pub fn try_new(resource: String) -> Result<Self> {
        let resource = IriRef::parse(resource)?;
        Ok(Self::new(resource))
    }
    // pub fn with_resource(resource: String) -> Self {
    //     Self {
    //         resource,
    //         resource_id: None,
    //     }
    // }
}

impl Reference {
    pub fn resource(&self) -> &str {
        self.resource.as_str()
    }
}
