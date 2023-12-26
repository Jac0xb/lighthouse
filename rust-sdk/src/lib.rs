use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub mod tx_builder;

pub fn append_transaction_guard(rpc_client: RpcClient) {}

pub fn protect_fee_payer(pubkey: Pubkey, expected_balance: u64) {}
