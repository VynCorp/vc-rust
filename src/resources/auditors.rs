use crate::client::Client;

pub struct Auditors<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Auditors<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
