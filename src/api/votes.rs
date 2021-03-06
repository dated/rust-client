use crate::http::client::Client;
use std::borrow::Borrow;

use crate::api::models::transaction::Transaction;
use crate::api::Result;

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Transaction>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("votes", parameters).await
    }

    pub async fn show(&mut self, id: &str) -> Result<Transaction> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint).await
    }
}
