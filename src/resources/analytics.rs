use crate::client::Client;

pub struct Analytics<'a> {
    client: &'a Client,
}

impl<'a> Analytics<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
