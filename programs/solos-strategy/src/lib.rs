use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("E1eeAWjEGqfH1CQ1SAtsnnuwwoJyrswjjboEwGLg8eG4");

#[program]
pub mod solos_strategy {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn deposit_reserve_and_obligation(ctx: Context<DepositReserveAndObligation>, amount: u64) -> ProgramResult {
        Ok(())
    }

    pub fn withdraw_reserve_and_obligation(ctx: Context<WithdrawReserveAndObligation>, amount: u64) -> ProgramResult {
        Ok(())
    }

    pub fn sell_reserves(ctx: Context<SellReserves>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DepositReserveAndObligation<'info> {
    pub deposit_token_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = deposit_token_mint,
        associated_token::authority = depositor,
    )]
    pub user_associated_token: Account<'info, TokenAccount>,
    pub depositor: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawReserveAndObligation {}

#[derive(Accounts)]
pub struct SellReserves {}
