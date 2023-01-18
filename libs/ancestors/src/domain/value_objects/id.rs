
use ulid::Ulid;

pub struct Id {
    pub value: Ulid,
}

impl Id {
    pub fn new(value: Ulid) -> Self {
        Self {
            value,
        }
    }

    pub fn gen() -> Self {
        Self::new(Ulid::new())
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.value.to_string()
    }
}
