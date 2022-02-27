import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Mycalculatordapp } from '../target/types/mycalculatordapp';
const assert = require('assert');
const { SystemProgram } = anchor.web3;

describe('mycalculatordapp', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();

  const program = anchor.workspace.Mycalculatordapp as Program<Mycalculatordapp>;
  const calculator = anchor.web3.Keypair.generate();

  it('Creates a calculator', async () => {
    await program.rpc.create("Welcome to Solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator]
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it("Adds two numbers", async function() {
    
    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)));
    assert.ok(account.greeting === "Welcome to Solana");
  });

  it('Multiplies two numbers', async function() {

    await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(6)));
    assert.ok(account.greeting === "Welcome to Solana");

  })

  it('Subtracts two numbers', async function() {

    await program.rpc.subtract(new anchor.BN(20), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(17)));

  });

  it('Divides two numbers', async function() {

    await program.rpc.divide(new anchor.BN(20), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
      },
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(6)));
    assert.ok(account.remainder.eq(new anchor.BN(2)));

  });
});
