mod error;
mod scrape;

pub use error::{ProcessError, ScrapeError};
pub use scrape::scrape;
