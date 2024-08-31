use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};



fn create_account() -> Keypair {
    Keypair::new()
}


fn create_transaction(client: &RpcClient, payer: &Keypair, recipient: &Keypair, lamports: u64) -> Transaction {
    let ix = system_instruction::transfer(&payer.pubkey(), &recipient.pubkey(), lamports);
    Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], client.get_latest_blockhash().unwrap())
}
