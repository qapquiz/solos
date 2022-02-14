use anchor_lang::{prelude::*, solana_program::{instruction::Instruction, program::invoke_signed}};
use spl_token_lending::instruction::LendingInstruction;

declare_id!("BVnmVsfaHhHm8QMNYHezgXvNMuNwAz2JaR3Xbwx86SxH");

// @todo #3 need to change token-lending program id
//  do not know yet where to get this
pub fn token_lending_program_id() -> Pubkey {
    Pubkey::new(&[1, 2, 3, 4]) 
}

pub fn init_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligation<'info>>
) -> ProgramResult {
    let ix = Instruction {
        program_id: token_lending_program_id(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.obligation.key(), false),
            AccountMeta::new_readonly(ctx.accounts.lending_market.key(), false),
            AccountMeta::new_readonly(ctx.accounts.obligation_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.obligation_owner.key(), false),
            AccountMeta::new_readonly(ctx.accounts.clock.key(), false),
            AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
            AccountMeta::new_readonly(ctx.accounts.spl_token_id.key(), false),
        ],
        data: LendingInstruction::InitObligation.pack(),
    };

    invoke_signed(
        &ix, 
        &[
            ctx.accounts.obligation,
            ctx.accounts.lending_market,
            ctx.accounts.obligation_owner,
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.spl_token_id,
            ctx.program,
        ], 
        ctx.signer_seeds
    )
}

#[derive(Accounts)]
pub struct InitObligation<'info> {
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_owner: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub spl_token_id: AccountInfo<'info>,
}
