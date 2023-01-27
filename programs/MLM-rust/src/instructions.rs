use std::borrow::Borrow;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    entrypoint::ProgramResult,
};
use crate::{MINIMUM_INVEST, LEVEL_COMISSION, LEVEL_INVESTMENTS, state::*};


pub fn invest(ctx: Context<Invest>, invest_amount: f32, payer_account: Pubkey) -> ProgramResult {
    if invest_amount < MINIMUM_INVEST as f32 {
        panic!("You sent less lamports than needed");
    }

    **ctx.accounts.payer
        .to_account_info()
        .try_borrow_mut_lamports()? -= invest_amount;
    **ctx.accounts.recipient
        .to_account_info()
        .try_borrow_mut_lamports()? += invest_amount;

    //let _ = ctx.accounts.mlm_system.accounts_balance[&payer_account] + invest_amount as f32 * 5.0 / 100.0;
    let amount_to_account = invest_amount - (invest_amount * 5 / 100);
    ctx.accounts.mlm_system.accounts_balance.insert(payer_account, amount_to_account);

    Ok(())
}


pub fn withdraw(ctx: Context<Withdraw>, payment_account: Pubkey) -> ProgramResult {
    let mut user_balance: f32 = *ctx.accounts.mlm_system.accounts_balance.get(&payment_account).unwrap() as f32;

    if user_balance <= 0.0 {
        panic!("You don't have money to withdraw");
    }

    const PERCENTAGE: usize = 1000;
    let mut current_address: Pubkey = payment_account;
    let mut counter_depth_of_referal: f64 = 0.0;
    let mut comission_to_partner_account: f32 = 0.0;
    let mut index;

    for _i in 0..10 {
        while payment_account.to_string() != " " {
            counter_depth_of_referal += 1.0;
            current_address = *ctx.accounts.mlm_system.referal_of_the_user.get(&payment_account).unwrap();
            index = get_level(ctx.accounts.mlm_system.borrow(), payment_account) as usize / PERCENTAGE;
            comission_to_partner_account = user_balance * LEVEL_COMISSION[index];
            **ctx.accounts.payer
                .to_account_info()
                .try_borrow_mut_lamports()? -= comission_to_partner_account as u64;
            **ctx.accounts.recipient
                .to_account_info()
                .try_borrow_mut_lamports()? += comission_to_partner_account as u64;
            user_balance = user_balance - comission_to_partner_account;
        }
    }

    *ctx.accounts.mlm_system.accounts_balance.get_mut(&payment_account).unwrap() = 0;
    Ok(())
}


pub fn get_level(mlm_system: &MLmSystem, account: Pubkey) -> u128 {
    let user_balance: u64 = *(mlm_system.accounts_balance.get(&account).unwrap());
    let mut res: u128 = 0;
    for mut i in 0..9 {
        if (user_balance as f64) < LEVEL_INVESTMENTS[i] {
            res = (i + 1) as u128;
        }
    }
    return res;
}


pub fn signup(ctx: Context<Signup>, account: Pubkey, referal_link: Pubkey) -> ProgramResult {
    let mut addresses_of_referals: Vec<Pubkey> = (ctx.accounts.mlm_system.partners_users.get(&referal_link))
        .expect("")
        .to_vec();
    addresses_of_referals.push(account);
    ctx.accounts.mlm_system.partners_users.insert(referal_link, addresses_of_referals as Vec<Pubkey>);
    println!("done");
    Ok(())
}


pub fn get_partners_info(ctx: Context<DirectPartners>, account_to_get_partners: Pubkey) -> ProgramResult {
    let partners_addresses: Vec<Pubkey> = (*ctx.accounts.mlm_system.partners_users[&account_to_get_partners]).to_vec();
    let amount_of_partners = partners_addresses.len();
    let mut partners_levels = vec![];

    for i in 0..amount_of_partners {
        partners_levels[i as usize] = get_level(ctx.accounts.mlm_system.borrow(), partners_addresses[i as usize]);
    }

    print!("Amount of partners: {}", amount_of_partners);
    print!("Levels of partners: {:?}", partners_levels);

    Ok(())
}
