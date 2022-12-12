use std::collections::HashMap;
use anchor_lang::prelude::*;

//
// pub struct AccountInfo<'a> {
//
// }

#[account]
pub struct MLmSystem{
    pub accounts_balance: HashMap<Pubkey, f32>,         // save balances of users' accounts
    pub partners_users: HashMap<Pubkey, Vec<Pubkey>>,   // address of directPartner -> users who entered with his referalLink //referals
    pub referal_of_the_user: HashMap<Pubkey, Pubkey>    // user - referal (who invited user)
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(init, payer = authority, space = 9000)]
    pub program: Account<'info, BaseAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,     // required element to create solana data
}

#[account]
pub struct BaseAccount {
    pub authority: Pubkey
}

#[derive(Accounts)]
pub struct Invest<'info> {
    pub mlm_system: Account<'info, MLmSystem>,
    //pub accounts: &[AccountInfo]
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    pub mlm_system: Account<'info, MLmSystem>,
}

#[derive(Accounts)]
pub struct Signup<'info> {
    pub mlm_system: Account<'info, MLmSystem>,
}

#[derive(Accounts)]
pub struct DirectPartners<'info>{
    pub mlm_system: Account<'info, MLmSystem>,
}

pub struct GetDirectPartnersInfo {
    pub amount_of_partners: usize,
    pub partners_levels: Vec<u128>
}