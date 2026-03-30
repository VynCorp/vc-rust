use crate::client::Client;

pub struct Credits<'a> {
    client: &'a Client,
}

impl<'a> Credits<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
