use crate::client::Client;

pub struct Watchlists<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Watchlists<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }
}
