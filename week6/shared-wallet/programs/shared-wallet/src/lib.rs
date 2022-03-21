use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;

declare_id!("G433gWMP3PKJ526HA5ADKD6tvMPkJNB4yobEwMfZqLry");

const NUMBER_OF_USERS: usize = 10;

#[program]
pub mod sharedwallet {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owners: [Pubkey; NUMBER_OF_USERS]) -> ProgramResult {
        ctx.accounts.info_account.owners = owners;
        ctx.accounts.info_account.wallet_address = ctx.accounts.spend_account.key();
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> ProgramResult {
        require!(
            ctx.accounts.info_account.owners.contains(&ctx.accounts.payer.key()),
            ConstraintOwner
        );
        let t_ix = transfer(
            &ctx.accounts.spend_account.key(),
            &ctx.accounts.payee.key(),
            amount,
        );
        invoke(
            &t_ix,
            &[
                ctx.accounts.spend_account.to_account_info(),
                ctx.accounts.payee.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ]
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer=signer)]
    pub info_account: Account<'info, State>,
    #[account()]
    pub spend_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    pub info_account: Account<'info, State>,
    #[account(mut)]
    pub spend_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub payee: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub owners: [Pubkey; NUMBER_OF_USERS],
    pub wallet_address: Pubkey
}