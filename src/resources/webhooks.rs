use crate::client::Client;

pub struct Webhooks<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Webhooks<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
