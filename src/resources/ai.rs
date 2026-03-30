use crate::client::Client;

pub struct Ai<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
