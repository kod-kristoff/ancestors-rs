use gedcomx_model::common::IriRef;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Id {
    pub value: IriRef,
}

impl Id {
    pub fn new(value: IriRef) -> Self {
        Self { value }
    }

    pub fn gen() -> Self {
        let value = IriRef::parse(format!("#{}", Ulid::new())).unwrap();
        Self::new(value)
        // Self::new(Ulid::new().to_string().try_into().unwrap())
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.value.to_string()
    }
}

impl From<&IriRef> for Id {
    fn from(value: &IriRef) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}
