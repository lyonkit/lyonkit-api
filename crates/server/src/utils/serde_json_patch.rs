use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Patch<T> {
    Missing,
    Null,
    Value(T),
}

impl<T> Patch<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Patch::Null)
    }

    pub fn is_missing(&self) -> bool {
        matches!(self, Patch::Missing)
    }
}

impl<T> Default for Patch<T> {
    fn default() -> Self {
        Patch::Missing
    }
}

impl<T> From<Option<T>> for Patch<T> {
    fn from(opt: Option<T>) -> Patch<T> {
        match opt {
            Some(v) => Patch::Value(v),
            None => Patch::Null,
        }
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}
