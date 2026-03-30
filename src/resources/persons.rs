use crate::client::Client;

pub struct Persons<'a> {
    client: &'a Client,
}

impl<'a> Persons<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
