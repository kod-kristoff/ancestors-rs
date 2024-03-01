mod source_citation;
mod source_description;
mod source_id;

pub use source_citation::SourceCitation;
pub use source_description::SourceDescription;
pub use source_id::SourceId;

use crate::shared::IdReference;

pub type SourceReference = IdReference<SourceId>;
