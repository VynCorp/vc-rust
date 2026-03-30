use crate::client::Client;

pub struct Billing<'a> {
    client: &'a Client,
}

impl<'a> Billing<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
