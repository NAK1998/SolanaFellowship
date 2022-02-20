use anchor_lang::prelude::*;

declare_id!("EqB1Lcj3rRKtdbrRXJn3ChtqJhziKbtdA3V8HMakBRdy");

const DUMMY_NAME: &str = "00000000";
const MAX_PROPOSALS: i32 = 20;

pub fn get_init_names() -> Vec<String> {
    let mut name_vec: Vec<String> = Vec::new();
    for _ in 0..MAX_PROPOSALS {
        name_vec.push(DUMMY_NAME.to_string());
    }
    return name_vec;
}

pub fn get_init_votes() -> Vec<i32> {
    let mut vote_vec: Vec<i32> = Vec::new();
    for _ in 0..MAX_PROPOSALS {
        vote_vec.push(0);
    }
    return vote_vec;
}

#[program]
pub mod voting_contract {
    use super::*;

    pub fn add_voter(ctx: Context<AddVoter>) -> ProgramResult {
        let voter = &mut ctx.accounts.voter;
        voter.voted = false;
        voter.delegate = *voter.to_account_info().key;
        voter.weight = 1;
        voter.vote = -1;
        Ok(())
    }

    pub fn add_candidate(ctx: Context<AddProposal>, name: String) -> ProgramResult {
        let proposals = &mut ctx.accounts.proposals;
        msg!("Is initialised: {:?}", proposals.initialized);
        if proposals.initialized == true
        {
            msg!("Proposal list already initialized");
        }
        else
        {
            proposals.name = get_init_names();
            proposals.vote_count = get_init_votes();
            proposals.initialized = true;
        }
        let index = proposals.name.iter().position(|p| p == DUMMY_NAME).unwrap();
        proposals.name[index] = name;
        proposals.vote_count[index] = 0;
        Ok(())
    }

    pub fn vote_for_candidate(ctx: Context<VoteCandidate>, index: i32) -> ProgramResult {
        let proposals = &mut ctx.accounts.proposals;
        let voter = &mut ctx.accounts.voter;
        require!(
            *voter.to_account_info().key == ctx.accounts.voter_signer.key(),
            ProgramError::MissingRequiredSignature
        );

        if voter.delegate != *voter.to_account_info().key
        {
            panic!("Vote has been delegated to {:?}", voter.delegate)
        }

        if voter.voted == true
        {
            panic!("Already voted {:?}", voter.to_account_info().key);
        }
        else
        {
            if index > MAX_PROPOSALS || proposals.name[index as usize] == DUMMY_NAME
            {
                panic!("Invalid Option for vote {:?}", index);
            }
            proposals.vote_count[index as usize] += voter.weight;
            voter.weight = 0;
            voter.voted = true;
            voter.vote = index;
        }
        Ok(())
    }

    pub fn delegate_vote(ctx: Context<DelegateVote>) -> ProgramResult{
        let from_voter = &mut ctx.accounts.from_voter;
        let to_voter = &mut ctx.accounts.to_voter;
        let voter_signer = &mut ctx.accounts.voter_signer;
        require!(
            //check if from_voter is the signer
            *from_voter.to_account_info().key == voter_signer.key(),
            ProgramError::MissingRequiredSignature
        );
        require!(
            //check if vote of from_voter is already delegated
            from_voter.delegate == *from_voter.to_account_info().key,
            ProgramError::InvalidAccountData
        );
        to_voter.weight += from_voter.weight;
        from_voter.weight = 0;
        from_voter.delegate = *to_voter.to_account_info().key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddProposal<'info> {
    #[account(init_if_needed, payer = chair_person, space = 480)]
    pub proposals: Account<'info, Proposal>,
    #[account(mut)]
    pub chair_person: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddVoter<'info> {
    #[account(init, payer = chair_person, space = 128)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub chair_person: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteCandidate<'info> {
    #[account(mut)]
    pub voter_signer: Signer<'info>,
    #[account(mut)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub proposals: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct DelegateVote<'info>{
    #[account(mut)]
    pub voter_signer: Signer<'info>,
    #[account(mut)]
    pub from_voter: Account<'info, Voter>,
    #[account(mut)]
    pub to_voter: Account<'info, Voter>,
}

#[account]
pub struct Voter {
    pub weight: i32,
    pub voted: bool,
    pub delegate: Pubkey,
    pub vote: i32
}

#[account]
pub struct Proposal {
    pub name: Vec<String>,
    pub vote_count: Vec<i32>,
    pub initialized: bool
}