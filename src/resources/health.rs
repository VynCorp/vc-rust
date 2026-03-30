use crate::client::Client;

pub struct Health<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Health<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
