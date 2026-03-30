use crate::client::Client;

pub struct Dashboard<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Dashboard<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
