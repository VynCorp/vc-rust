pub mod client;
pub mod error;
pub mod resources;
pub mod response;
pub mod types;

#[cfg(feature = "blocking")]
pub mod blocking;

// Re-export core types at crate root for ergonomic imports.
pub use client::{Client, ClientBuilder};
pub use error::{ErrorBody, VyncoError};
pub use resources::{Ai, Auditors, Companies, Dashboard, ExportFile, Exports, Health, Screening, Watchlists, Webhooks};
pub use response::{Response, ResponseMeta};
pub use types::*;
