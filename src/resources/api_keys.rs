use crate::client::Client;

pub struct ApiKeys<'a> {
    client: &'a Client,
}

impl<'a> ApiKeys<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
