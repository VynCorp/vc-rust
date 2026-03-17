pub mod client;
pub mod error;
pub mod response;
pub mod resources;
pub mod types;

#[cfg(feature = "blocking")]
pub mod blocking;

// Re-export core types at crate root for ergonomic imports.
pub use client::{Client, ClientBuilder};
pub use error::{ErrorBody, VyncoError};
pub use response::{Response, ResponseMeta};
pub use resources::{
    ApiKeys, Billing, Companies, Credits, Dossiers, Persons, Settings, Teams, Users, Webhooks,
};
pub use types::*;
