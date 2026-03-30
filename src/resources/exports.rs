use crate::client::Client;
use crate::response::ResponseMeta;

pub struct Exports<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Exports<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}

/// A downloaded export file with metadata.
pub struct ExportFile {
    pub meta: ResponseMeta,
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}
