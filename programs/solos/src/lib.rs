use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solos {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let database = &mut ctx.accounts.database;
        database.current_strategy = *ctx.accounts.strategy.key;
        database.authority = *ctx.accounts.database_authority_pda.key;
        Ok(())
    }

    pub fn set_strategy(ctx: Context<SetStrategy>) -> ProgramResult {
        // @todo #2 replace current strategy in SolosDatabase
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority_bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = SolosDatabase::space(),
    )]
    pub database: Account<'info, SolosDatabase>,
    #[account(
        seeds = [b"database_authority"],
        bump = authority_bump,
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
    pub authority: Pubkey,
}

impl SolosDatabase {
    pub fn space() -> usize {
        8 + 
        32 + 
        32
    }
}
