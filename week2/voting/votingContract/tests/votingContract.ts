import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VotingContract } from "../target/types/voting_contract";
const assert = require('assert');

describe("votingContract", () => {
  const fs = require("fs");
  const file = fs.readFileSync("target/idl/voting_contract.json", "utf8");
  const idl = JSON.parse(file);
  //const provider = anchor.Provider.local("https://ssc-dao.genesysgo.net");
  const provider = anchor.Provider.local();
  const programId = new anchor.web3.PublicKey(
    "EqB1Lcj3rRKtdbrRXJn3ChtqJhziKbtdA3V8HMakBRdy"
  );
  const prog = new anchor.Program(idl, programId, provider);

  const airDropSol = async (walletKeyPair) => {
    try {
      const connection = new anchor.web3.Connection(
        "http://localhost:8899",
        "confirmed"
      );
      console.log(`-- Airdropping 2 SOL --`);
      const fromAirDropSignature = await connection.requestAirdrop(
        new anchor.web3.PublicKey(walletKeyPair.publicKey),
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(fromAirDropSignature);
    } catch (err) {
      console.log(err);
    }
  };

  const addVoterByChairPerson = async(voterAccount, chairPersonAccount) => {
    await prog.rpc.addVoter({
      accounts: {
        voter: voterAccount.publicKey,
        chairPerson: chairPersonAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [chairPersonAccount, voterAccount],
    })

  }

  const addCandidateByChairPerson = async(proposalAccount,chairPersonAccount, name) =>{
    await prog.rpc.addCandidate(name, {
      accounts: {
        proposals: proposalAccount.publicKey,
        chairPerson: chairPersonAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [chairPersonAccount, proposalAccount],
    });
  }

  const voteForCandidateFunction = async(voterAccount, proposalAccount, indexToVote ) =>{
    await prog.rpc.voteForCandidate(new anchor.BN(indexToVote), 
    {
      accounts: {
        voter: voterAccount.publicKey,
        voterSigner: voterAccount.publicKey,
        proposals: proposalAccount.publicKey,
      },
      signers: [voterAccount]
    })
  }

  const delegateVote = async (from_voter, to_voter) => {
    await prog.rpc.delegateVote({
      accounts: {
        fromVoter: from_voter.publicKey,
        toVoter: to_voter.publicKey,
        voterSigner: from_voter.publicKey
      },
      signers: [from_voter]
    })
  }

  let proposalAccount = anchor.web3.Keypair.generate();
    let voterAccount1 = anchor.web3.Keypair.generate();
    let voterAccount2= anchor.web3.Keypair.generate();
    let voterAccount3 = anchor.web3.Keypair.generate();
    let voterAccount4 = anchor.web3.Keypair.generate();
    let chairPersonAccount = anchor.web3.Keypair.generate();

  it("Initializing voters and candidates", async () => {
    
    await airDropSol(chairPersonAccount);

    //initialize voters
    await addVoterByChairPerson(voterAccount1, chairPersonAccount);
    await addVoterByChairPerson(voterAccount2, chairPersonAccount);
    await addVoterByChairPerson(voterAccount3, chairPersonAccount);
    await addVoterByChairPerson(voterAccount4, chairPersonAccount);

    //initialize candidates
    for (let i=1; i<4; i++)
      await addCandidateByChairPerson(proposalAccount, chairPersonAccount, "0000000" + i);
    
  });

  it("Execute voting", async () => {
    //voting by voters
    await delegateVote(voterAccount1, voterAccount2);
    await delegateVote(voterAccount2, voterAccount3);
    //await voteForCandidateFunction(voterAccount2, proposalAccount, 1);
    await voteForCandidateFunction(voterAccount3, proposalAccount, 2);
    await voteForCandidateFunction(voterAccount4, proposalAccount, 0);

    let acct = await prog.account.proposal.fetch(proposalAccount.publicKey);
    console.log(acct);
    assert.ok(acct.voteCount[0] === 1);
    assert.ok(acct.voteCount[2] === 3);
  });
});
