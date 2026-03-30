use crate::client::Client;

pub struct Teams<'a> {
    client: &'a Client,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
