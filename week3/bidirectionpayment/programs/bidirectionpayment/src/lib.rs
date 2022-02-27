use anchor_lang::prelude::*;
use anchor_lang::solana_program as sp;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bidirectionpayment {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, alice_amount: u64, bob_amount: u64) -> ProgramResult {
        ctx.accounts.metadata.alice = *ctx.accounts.alice.key;
        ctx.accounts.metadata.bob = *ctx.accounts.bob.key;
        ctx.accounts.metadata.a_balance = alice_amount;
        ctx.accounts.metadata.b_balance = bob_amount;
        let clock = Clock::get()?;
        ctx.accounts.metadata.start_time = clock.unix_timestamp;
        let ix = sp::system_instruction::transfer(
            &ctx.accounts.alice.to_account_info().key, 
            &ctx.accounts.metadata.to_account_info().key(), 
            alice_amount
        );
        sp::program::invoke(&ix, &[ctx.accounts.alice.to_account_info(), ctx.accounts.metadata.to_account_info()])?;

        let ix = sp::system_instruction::transfer(
            &ctx.accounts.bob.to_account_info().key, 
            &ctx.accounts.metadata.to_account_info().key(), 
            bob_amount
        );
        sp::program::invoke(&ix, &[ctx.accounts.bob.to_account_info(), ctx.accounts.metadata.to_account_info()])?;
        
        Ok(())
    }

    pub fn update(ctx: Context<Update>, alice_amount: u64, bob_amount: u64) -> ProgramResult {
        let state_account = &mut ctx.accounts.metadata;
        if state_account.a_balance + state_account.b_balance < alice_amount + bob_amount {
            panic!("Not enough Balance");
        }
        state_account.a_balance = alice_amount;
        state_account.b_balance = bob_amount;
        Ok(())
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> ProgramResult {
        let state_account = &ctx.accounts.metadata;
        let alice = state_account.alice;
        let bob = state_account.bob;
        let clock = Clock::get()?;
        if (clock.unix_timestamp - ctx.accounts.metadata.start_time) < 60 * 2 {
            panic!("Time is invalid");
        }
        if *ctx.accounts.signer.key == alice {

            **ctx.accounts.metadata.to_account_info().try_borrow_mut_lamports()? -= state_account.a_balance;
            **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += state_account.a_balance;
            **ctx.accounts.metadata.to_account_info().try_borrow_mut_lamports()? -= state_account.b_balance;
            **ctx.accounts.other.to_account_info().try_borrow_mut_lamports()? += state_account.b_balance;
        
        } else if *ctx.accounts.signer.key == bob {
            
            **ctx.accounts.metadata.to_account_info().try_borrow_mut_lamports()? -= state_account.b_balance;
            **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += state_account.b_balance;
            **ctx.accounts.metadata.to_account_info().try_borrow_mut_lamports()? -= state_account.a_balance;
            **ctx.accounts.other.try_borrow_mut_lamports()? += state_account.a_balance;
        
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub alice: Signer<'info>,
    #[account(mut, constraint = bob.key != alice.key)]
    pub bob: Signer<'info>,
    #[account(init, payer=alice)]
    pub metadata: Account<'info, PaymentMetadata>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub alice: Signer<'info>,
    #[account(mut, constraint = bob.key != alice.key)]
    pub bob: Signer<'info>,
    #[account(mut, constraint = metadata.to_account_info().lamports() >= metadata.a_balance + metadata.b_balance)]
    pub metadata: Account<'info, PaymentMetadata>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub other: AccountInfo<'info>,
    #[account(mut, close=signer,
        constraint = metadata.alice == *signer.key || metadata.bob == *signer.key,
        constraint = metadata.alice == *other.key || metadata.bob == *other.key,
        constraint = metadata.to_account_info().lamports() >= metadata.a_balance + metadata.b_balance)]
    pub metadata: Account<'info, PaymentMetadata>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct PaymentMetadata {
    pub alice: Pubkey,
    pub bob: Pubkey,
    pub a_balance: u64,
    pub b_balance: u64,
    pub start_time: i64,
}
