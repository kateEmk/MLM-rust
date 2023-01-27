use anchor_lang::prelude::*;


#[derive(Accounts)]
#[instruction(program_bump: u8)]
pub struct Initialize<'info>{
    #[account(
        init,
        seeds = [b"program".as_ref()],
        bump,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1
    )]
    pub program: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BaseAccount {
    pub balance: u64,
    pub bump: u8
}

#[derive(Accounts)]
#[instruction(program_bump: u8)]
pub struct UserPdaAccount<'info> {
    #[account(
        init,
        seeds = [b"program".as_ref()],
        bump,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    // /// CHECK: This is just an example, not checking data
    // pub recipient: UncheckedAccount<'info>,
}

#[account]
pub struct UserInfo {
    pub balance: u64,
    pub partners: Vec<Pubkey>,
    pub referal: Pubkey,
    pub seeds: String,
    pub bump: u8,
}

// #[derive(Accounts)]
// pub struct Invest<'info> {
//     // #[account(
//     //     init, payer = user, space = 8 + 2 + 4 + 200 + 1, seeds = [b"user-stats", user.key().as_ref()], bump
//     // )]
//     // pub user: Account<'info, CreateUserInfo<'info>>,
//     /// CHECK: This is just an example, not checking data
//     pub recipient: UncheckedAccount<'info>,
//     /// CHECK: This is just an example, not checking data
//     #[account(mut)]
//     pub payer: UncheckedAccount<'info>,
// }

// #[derive(Accounts)]
// #[instruction(program_bump: u8)]
// pub struct Withdraw<'info> {
//     #[account(
//         init, payer = user, space = 8 + 2 + 4 + 200 + 1, seeds = [b"user_info", user.key().as_ref()], bump
//     )]
//     pub user_info: Account<'info, UserInfo<'info>>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Withdraw<'info>{
//     pub mlm_system: Account<'info, MLmSystem<'info>>,
//     pub account_to_withdraw: Account<'info, WithdrawAccount>,
//     /// CHECK: This is just an example, not checking data
//     pub recipient: UncheckedAccount<'info>,
//     /// CHECK: This is just an example, not checking data
//     #[account(mut)]
//     pub payer: UncheckedAccount<'info>,
// }

// #[account]
// #[derive(Default, Debug)]
// pub struct WithdrawAccount{
//     pub features: u64,
//     /// Authority address.
//     pub authority: Pubkey,
//     /// Authority address allowed to mint from the candy machine.
//     pub mint_authority: Pubkey,
//     /// The collection mint for the candy machine.
//     pub collection_mint: Pubkey,
//     /// Number of assets redeemed.
//     pub items_redeemed: u64,
// }

// #[derive(Accounts)]
// pub struct Signup<'info> {
//     pub mlm_system: Account<'info, MLmSystem<'info>>,
// }
//
// #[derive(Accounts)]
// pub struct DirectPartners<'info>{
//     pub mlm_system: Account<'info, MLmSystem<'info>>,
// }
