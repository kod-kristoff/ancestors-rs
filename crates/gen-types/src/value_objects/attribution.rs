use chrono::{DateTime, Utc};
use serde_with::TimestampMilliSeconds;

use crate::entities::AgentReference;

#[serde_with::serde_as]
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Attribution {
    contributor: Option<AgentReference>,
    #[serde_as(as = "TimestampMilliSeconds")]
    modified: DateTime<Utc>,
}

impl Attribution {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Attribution {
    pub fn contributor<C: Into<AgentReference>>(mut self, contributor: C) -> Self {
        self.set_contributor(contributor.into());
        self
    }

    pub fn set_contributor(&mut self, contributor: AgentReference) {
        self.contributor = Some(contributor);
    }

    pub fn modified(mut self, modified: DateTime<Utc>) -> Self {
        self.set_modified(modified);
        self
    }

    pub fn set_modified(&mut self, modified: DateTime<Utc>) {
        self.modified = modified;
    }
}
pub fn verify_attribution_opt(
    a: Option<&Attribution>,
    b: Option<&Attribution>,
) -> Result<(), String> {
    if let Some(a_inner) = a {
        if let Some(b_inner) = b {
            if a_inner.contributor != b_inner.contributor {
                return Err(format!(
                    "a.contributor != b.contributor, '{:?}' != '{:?}'",
                    a_inner.contributor, b_inner.contributor,
                ));
            }
            if a_inner.modified != b_inner.modified {
                return Err(format!(
                    "a.modified != b.modified, '{:?}' != '{:?}'",
                    a_inner.modified, b_inner.modified,
                ));
            }
        } else {
            return Err(format!("a != b, '{:?}' != 'None'", a));
        }
    } else if b.is_some() {
        return Err(format!("a != b, 'None' != '{:?}'", b));
    }
    Ok(())
}
