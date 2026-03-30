use crate::client::Client;

pub struct Screening<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Screening<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
