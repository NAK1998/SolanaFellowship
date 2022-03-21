use anchor_lang::prelude::*;

declare_id!("9GmG7KYiu1w34Nf3RGiHBgxRpVQBGxj6tGEvXTV28W78");

#[program]
pub mod identity_verify {
    use super::*;

    pub fn claim_username(ctx: Context<CreateUser>, username: String, bump: u8) -> ProgramResult {
        // check that our username meets length requirements
        if username.len() >= UserAccount::MAX_USERNAME_LEN {
            return Err(ProgramError::InvalidArgument.into());
        }
        // set the fields in the user account
        ctx.accounts.user.username = username.clone();
        ctx.accounts.user.bump = bump;
        ctx.accounts.user.authority = ctx.accounts.authority.key();
        Ok(())
    }
}



#[derive(Accounts)]
// Anchor provides a handy way to reference instruction arguments
// to assist in loading accounts
#[instruction(username: String, bump: u8)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds = [username.as_bytes()],
        bump,
        payer = authority,
        space = UserAccount::SPACE,
        owner = *program_id
    )]
    user: Account<'info, UserAccount>,

    system_program: AccountInfo<'info>,

    #[account(mut, signer)]
    authority: AccountInfo<'info>,
}


#[account]
pub struct UserAccount {
    pub username: String,
    /// the authority of this username (the user's personal public key)
    pub authority: Pubkey,
    /// the PDA bump number
    pub bump: u8,
}

impl UserAccount {
    /// Account storage space: `tag, bump, pubkey, username`
    /// Anchor prefixes the bytes with a `tag=SHA256(StructName)[..8]`
    const SPACE: usize = 8 + 1 + 32 + Self::MAX_USERNAME_LEN;

    /// we desire to limit usernames to 140 bytes
    const MAX_USERNAME_LEN: usize = 140;
}