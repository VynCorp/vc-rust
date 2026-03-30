use crate::client::Client;

pub struct Changes<'a> {
    client: &'a Client,
}

impl<'a> Changes<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
