use std::borrow::Borrow;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    entrypoint::ProgramResult,
    account_info::next_account_info,
    program::invoke,
    system_instruction::transfer,
};
use anchor_spl::token::{Approve};
use anchor_spl::token::spl_token::instruction::TokenInstruction::Transfer;
use crate::{
    MINIMUM_INVEST, LEVEL_COMISSION,
    utils::get_level,
    state::*,
};


pub fn invest(ctx: Context<Invest>, amount: u64, payer_account: Pubkey) -> ProgramResult {
    if amount < MINIMUM_INVEST {
        panic!("You sent less lamports than needed");
    }

    let _ = ctx.accounts.mlm_system.balance + amount as f32 * 5.0 / 100.0;
    let amount_to_account = amount as f64 * 0.95;
    ctx.accounts.mlm_system.accounts_balance.insert(payer_account, amount_to_account as f32);
    Ok(())
}


pub fn withdraw(mut ctx: Context<Withdraw>, account_to_withdraw: Pubkey) -> ProgramResult {
    let mut user_balance: f32 = *ctx.accounts.mlm_system.accounts_balance.get(&account_to_withdraw).unwrap();

    if user_balance <= 0.0 {
        panic!("You don't have money to withdraw");
    }

    const GET_PERCENTAGE: usize = 1000;
    let mut current_address: Pubkey = account_to_withdraw;
    let mut counter_depth: f64 = 0.0;
    let mut comission: f32 = 0.0;
    let mut index;

    for _i in 0..10 {
        while account_to_withdraw.to_string() != " " {
            counter_depth += 1.0;
            current_address = *ctx.accounts.mlm_system.referal_of_the_user.get(&account_to_withdraw).unwrap();
            index = get_level(ctx.accounts.mlm_system.borrow(), account_to_withdraw) as usize / GET_PERCENTAGE;
            comission = user_balance * LEVEL_COMISSION[index];
            user_balance = user_balance - comission;
        }
    }

    *ctx.accounts.mlm_system.accounts_balance.get_mut(&account_to_withdraw).unwrap() = 0.0;
    Ok(())
}


pub fn login(mut ctx: Context<Login>, account: Pubkey, referal_link: Pubkey) -> ProgramResult {
    let mut addresses_of_referals: Vec<Pubkey> = (ctx.accounts.mlm_system.partners_users.get(&referal_link))
        .expect("")
        .to_vec();
    addresses_of_referals.push(account);
    ctx.accounts.mlm_system.partners_users.insert(referal_link, addresses_of_referals as Vec<Pubkey>);
    Ok(())
}


pub fn direct_partners_info(ctx: Context<DirectPartners>, sender: Pubkey) -> ProgramResult {
    let partners_addresses: Vec<Pubkey> = (*ctx.accounts.mlm_system.partners_users[&sender]).to_vec();
    let amount_of_partners = partners_addresses.len();
    let mut partners_levels = vec![];

    for i in 0..amount_of_partners {
        partners_levels[i as usize] = get_level(ctx.accounts.mlm_system.borrow(), partners_addresses[i as usize]);
    }

    print!("Amount of partners: {}", amount_of_partners);
    print!("Levels of partners: {:?}", partners_levels);
    Ok(())
}