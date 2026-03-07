pub use self::{
    attribution::{verify_attribution_opt, Attribution},
    date::Date,
    evidence_reference::EvidenceReference,
    fact::{Fact, FactType},
    gender::Gender,
    identifier::{Identifier, IdentifierType},
    member_info::MemberInfo,
    name::{Name, NameForm, NamePartType},
    reference::Reference,
    resource_reference::ResourceReference,
    resource_type::ResourceType,
    text_value::TextValue,
};
// pub use uri::Uri;
// pub use qname_uri::QnameUri;

// pub type Date = chrono::NaiveDate;
// pub type DateTime = chrono::DateTime<chrono::Utc>;

mod attribution;
mod date;
mod evidence_reference;
mod fact;
mod gender;
mod identifier;
mod member_info;
mod name;
mod resource_type;
// mod qname_uri;
mod reference;
mod resource_reference;
mod text_value;
// mod uri;
