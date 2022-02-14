use anchor_lang::prelude::*;

declare_id!("CRM9fKJvPbLWckdHdrRMndtRDvhXVW5dxD6tCeAYcZZ5");

#[program]
pub mod solos {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority_seed: String) -> ProgramResult {
        let database = &mut ctx.accounts.database;
        database.current_strategy = *ctx.accounts.strategy.key;
        database.authority_seed = authority_seed;
        database.authority_bump = *ctx.bumps.get("database").unwrap();
        database.authority = *ctx.accounts.database_authority_pda.key;
        Ok(())
    }

    pub fn set_strategy(ctx: Context<SetStrategy>) -> ProgramResult {
        // @todo #2 replace current strategy in SolosDatabase
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority_seed: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = SolosDatabase::space(&authority_seed),
    )]
    pub database: Account<'info, SolosDatabase>,
    #[account(
        seeds = [authority_seed.as_ref()],
        bump,
    )]
    pub database_authority_pda: AccountInfo<'info>,
    pub strategy: AccountInfo<'info>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetStrategy {
    // @todo #1 add accounts for setting new strategy
    //  it will replace current_strategy in SolosDatabase
}

#[account]
pub struct SolosDatabase {
    pub current_strategy: Pubkey,
    pub authority_seed: String,
    pub authority_bump: u8,
    pub authority: Pubkey,
}

impl SolosDatabase {
    pub fn space(authority_seed: &str) -> usize {
        8 +  // discriminator
        4 + authority_seed.len() + // database_authority_seed
        1 + // database_authority_bump
        32 // authority
    }
}
