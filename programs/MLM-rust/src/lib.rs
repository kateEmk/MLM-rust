use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

pub mod instructions;
pub mod state;
use state::*;


pub const PROGRAM_ID: &str = "A8YCxz6TRy2Y1sEvYQUWhD7ZoR81syiuWMD3LEJbLEVB";     // token_address
pub const LEVEL_INVESTMENTS: [f64; 10] = [0.005, 0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
pub const LEVEL_COMISSION: [f32; 10] = [1.0, 0.7, 0.5, 0.2, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
pub const MINIMUM_INVEST: u64 = 1;


declare_id!("A8YCxz6TRy2Y1sEvYQUWhD7ZoR81syiuWMD3LEJbLEVB");


#[program]
pub mod mlm_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let program = &mut ctx.accounts.program;
        let authority = &mut ctx.accounts.authority;
        program.authority = authority.key();
        Ok(())
    }

    pub fn invest(ctx: Context<Invest>, amount_to_invest: f32, payer_account: Pubkey) -> ProgramResult {
        instructions::invest(ctx, amount_to_invest, payer_account)
    }

    pub fn withdraw(ctx: Context<Withdraw>, payment_account: Pubkey) -> ProgramResult {
        instructions::withdraw(ctx, payment_account)
    }

    pub fn signup(ctx: Context<Signup>, account: Pubkey, referal_link: Pubkey) -> ProgramResult {
        instructions::signup(ctx, account, referal_link)
    }

    pub fn direct_partners_info(ctx: Context<DirectPartners>, sender: Pubkey) -> ProgramResult {
        instructions::get_partners_info(ctx, sender)
    }
}
