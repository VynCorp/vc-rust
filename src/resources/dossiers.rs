use crate::client::Client;

pub struct Dossiers<'a> {
    client: &'a Client,
}

impl<'a> Dossiers<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
