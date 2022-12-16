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

    pub fn initialize(ctx: Context<Initialize>, program_bump: u8) -> Result<()> {
        let program = &mut ctx.accounts.program;
        program.bump = program_bump;
        Ok(())
    }

    pub fn invest(ctx: Context<UserPdaAccount>, amount_to_invest: u64, user_bump: u8) -> ProgramResult {
        instructions::invest(ctx, amount_to_invest, user_bump)
    }

    // pub fn withdraw(ctx: Context<UserPdaAccount>) -> ProgramResult {
    //     instructions::withdraw(ctx)
    // }

    pub fn signup(ctx: Context<UserPdaAccount>, referal_link: Pubkey) -> ProgramResult {
        instructions::signup(ctx, referal_link)
    }

    pub fn direct_partners_info(ctx: Context<UserPdaAccount>) -> ProgramResult {
        instructions::get_partners_info(ctx)
    }
}
