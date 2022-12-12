use anchor_lang::prelude::*;
use solana_client::rpc_client::RpcClient;
use anchor_lang::{
    solana_program::msg,
    solana_program::native_token::LAMPORTS_PER_SOL,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    system_instruction::transfer,
    transaction::Transaction,
    signature::Keypair
};


pub fn transfer_tokens(from: Pubkey, to: Pubkey, amount: f32) {
    let signer = Keypair::new();

    msg!(
        "Transferring {} tokens from {} to {}",
        amount,
        from.to_string(),
        to.to_string()
    );

    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    match connection.request_airdrop(&from, LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };

    let ix = transfer(&from, &to, amount as u64);

    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get latest blockhash.");
    let txn = Transaction::new_signed_with_payer(&[ix], Some(&from), &[&signer], recent_blockhash);

    match connection.send_and_confirm_transaction(&txn){
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(e) => println!("Error transferring Sol:, {:?}", e),
    }
}