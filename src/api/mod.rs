pub mod blocks;
pub mod bridgechains;
pub mod businesses;
pub mod delegates;
pub mod locks;
pub mod models;
pub mod node;
pub mod peers;
pub mod transactions;
pub mod votes;
pub mod wallets;

use self::blocks::Blocks;
use self::bridgechains::Bridgechains;
use self::businesses::Businesses;
use self::delegates::Delegates;
use self::locks::Locks;
use self::models::shared::Response;
use self::node::Node;
use self::peers::Peers;
use self::transactions::Transactions;
use self::votes::Votes;
use self::wallets::Wallets;
use super::error::Error;

use crate::http::client::Client;

pub type Result<T> = std::result::Result<Response<T>, Error>;

pub struct Api {
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub node: Node,
    pub peers: Peers,
    pub transactions: Transactions,
    pub votes: Votes,
    pub wallets: Wallets,
    pub locks: Locks,
    pub businesses: Businesses,
    pub bridgechains: Bridgechains,
    pub client: Client,
}

impl Api {
    pub fn new(host: &str) -> Api {
        Api::new_with_client(&Client::new(host))
    }

    pub fn new_with_client(client: &Client) -> Api {
        let client = client.clone();

        Api {
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            node: Node::new(client.clone()),
            peers: Peers::new(client.clone()),
            transactions: Transactions::new(client.clone()),
            votes: Votes::new(client.clone()),
            wallets: Wallets::new(client.clone()),
            locks: Locks::new(client.clone()),
            businesses: Businesses::new(client.clone()),
            bridgechains: Bridgechains::new(client.clone()),
            client,
        }
    }
}
