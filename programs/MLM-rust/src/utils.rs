use crate::{LEVEL_INVESTMENTS, MLmSystem};
use anchor_lang::{
    ToAccountInfo,
    prelude::{Context, CpiContext, Pubkey}
};
use anchor_spl::token;
use anchor_spl::token::{transfer, Transfer};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;


pub fn get_level(mlm_system: &MLmSystem, account: Pubkey) -> u128 {
    let balance: f64 = *(mlm_system.accounts_balance.get(&account).unwrap()) as f64;
    let mut res: u128 = 0;
    for mut i in 0..9 {
        if balance < LEVEL_INVESTMENTS[i] {
            i = i + 1;
            res = i as u128;
        }
    }
    return res
}
