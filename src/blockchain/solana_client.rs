use solana_client::rpc_client::RpcClient;

pub struct SolanaClient {
    client: RpcClient,
}

impl SolanaClient {
    pub fn new(url: &str) -> Self {
        SolanaClient {
            client: RpcClient::new(url.to_string()),
        }
    }

    pub fn get_latest_block(&self) {}
}
