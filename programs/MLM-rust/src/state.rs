use std::collections::HashMap;
use anchor_lang::prelude::*;


#[account]
pub struct MLmSystem{
    pub accounts_balance: HashMap<Pubkey, u64>,         // save balances of users' accounts
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
    /// CHECK: This is just an example, not checking data
    pub recipient: UncheckedAccount<'info>,
    /// CHECK: This is just an example, not checking data
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    pub mlm_system: Account<'info, MLmSystem>,
    pub account_to_withdraw: Account<'info, WithdrawAccount>,
    /// CHECK: This is just an example, not checking data
    pub recipient: UncheckedAccount<'info>,
    /// CHECK: This is just an example, not checking data
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,
}

#[account]
#[derive(Default, Debug)]
pub struct WithdrawAccount{
    pub features: u64,
    /// Authority address.
    pub authority: Pubkey,
    /// Authority address allowed to mint from the candy machine.
    pub mint_authority: Pubkey,
    /// The collection mint for the candy machine.
    pub collection_mint: Pubkey,
    /// Number of assets redeemed.
    pub items_redeemed: u64,
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
