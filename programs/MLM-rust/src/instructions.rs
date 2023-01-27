use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::{MINIMUM_INVEST, LEVEL_COMISSION, LEVEL_INVESTMENTS, state::*};


pub fn invest(ctx: Context<UserPdaAccount>, invest_amount: u64, user_bump: u8) -> ProgramResult {
    if invest_amount < MINIMUM_INVEST {
        panic!("You sent less lamports than needed");
    }

    **ctx.accounts.user
        .to_account_info()
        .try_borrow_mut_lamports()? -= invest_amount;
    **ctx.accounts.system_program
        .to_account_info()
        .try_borrow_mut_lamports()? += invest_amount;

    let user = &mut ctx.accounts.user_info;
    let amount_to_account = invest_amount - (invest_amount * 5 / 100);
    ctx.accounts.mlm_system.accounts_balance.insert(payer_account, amount_to_account);
    Ok(())
}


pub fn withdraw(ctx: Context<UserPdaAccount>) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;
    let mut user_balance: f32 = user.balance as f32;

    let payment_account= &mut ctx.accounts.user.key();

    if user_balance <= 0.0 {
        panic!("You don't have money to withdraw");
    }

    const PERCENTAGE: usize = 1000;
    let mut current_address: &mut Pubkey = payment_account;
    let mut counter_depth_of_referal: f64 = 0.0;
    let mut comission_to_partner_account: f32 = 0.0;
    let mut index;

    for _i in 0..10 {
        while payment_account.to_string() != " " {
            counter_depth_of_referal += 1.0;
            current_address = &mut user.referal;
            index = get_level(payment_account) as usize / PERCENTAGE;
            comission_to_partner_account = user_balance * LEVEL_COMISSION[index];
            **ctx.accounts.user
                .to_account_info()
                .try_borrow_mut_lamports()? -= comission_to_partner_account as u64;
            **ctx.accounts.system_program
                .to_account_info()
                .try_borrow_mut_lamports()? += comission_to_partner_account as u64;
            user_balance = user_balance - comission_to_partner_account;
        }
    }

    user.balance = 0;
    Ok(())
}


pub fn get_level(account: &mut Pubkey) -> u128 {
    let (pda, bump_seed) = Pubkey::find_program_address(&[b"program"], account);


    //let user_balance: u64 = *(mlm_system.accounts_balance.get(&account).unwrap());
    let user_balance: Pubkey = pda.key();
    // user_balance.
    let mut res: u128 = 0;
    for mut i in 0..9 {
        if (user_balance as f64) < LEVEL_INVESTMENTS[i] {
            res = (i + 1) as u128;
        }
    }
    return res;
}


pub fn signup(ctx: Context<UserPdaAccount>, referal_link: Pubkey) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;
    user.referal = referal_link;
    Ok(())
}


pub fn get_partners_info(ctx: Context<UserPdaAccount>) -> ProgramResult {
    let user = &mut ctx.accounts.user_info;

    let amount_of_partners = user.partners.len();
    let partners_addresses = &mut user.partners;
    let mut partners_levels = Vec::<usize>::with_capacity(amount_of_partners);

    // for i in 0..amount_of_partners {
    //     partners_levels[i as usize] = get_level(ctx.accounts.mlm_system.borrow(), partners_addresses[i as usize]);
    // }

    print!("Amount of partners: {}", amount_of_partners);
    print!("Levels of partners: {:?}", partners_levels);
    Ok(())
}
