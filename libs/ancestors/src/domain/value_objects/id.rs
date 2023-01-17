use std::marker::PhantomData;

use ulid::Ulid;

pub struct Id<T> {
    pub value: Ulid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: Ulid) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn gen() -> Self {
        Self::new(Ulid::new())
    }
}

impl<T> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.value.to_string()
    }
}
