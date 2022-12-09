use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


#[account]
pub struct MLmSystem{
    pub balance: f32,
    pub accounts_balance: HashMap<Pubkey, f32>,
    pub partners_users: HashMap<Pubkey, Vec<Pubkey>>,
    pub referal_of_the_user: HashMap<Pubkey, Pubkey>
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
pub struct Login<'info> {
    pub mlm_system: Account<'info, MLmSystem>,
}

#[derive(Accounts)]
pub struct DirectPartners<'info>{
    pub mlm_system: Account<'info, MLmSystem>,
}
