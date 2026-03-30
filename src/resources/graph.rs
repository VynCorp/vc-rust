use crate::client::Client;

pub struct Graph<'a> {
    client: &'a Client,
}

impl<'a> Graph<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
