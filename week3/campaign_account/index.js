const {
    Connection,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    PublicKey,
    SystemProgram,
    Keypair,
} = require('@solana/web3.js');
const fs = require('mz/fs');

const rpcUrl = 'http://localhost:8899';
let connection = new Connection(rpcUrl, 'confirmed');


async function createKeypairFromFile() {
    const secretKeyString = await fs.readFile("~/.config/solana/id.json", { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

async function createAccount() {

    const rpcUrl = 'http://localhost:8899';
    connection = new Connection(rpcUrl, 'confirmed');
    const signer = await createKeypairFromFile();
    const newAccountPubkey = await PublicKey.createWithSeed(
        signer.publicKey,
        "campaign A",
        new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
        1024,
    );
    const instruction = SystemProgram.createAccountWithSeed({
        fromPubkey: signer.publicKey,
        basePubkey: signer.publicKey,
        seed: "campaign A",
        newAccountPubkey,
        lamports,
        space: 1024,
        programId: new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
    });
    const transaction = new Transaction().add(
        instruction
    );


    await sendAndConfirmTransaction(connection, transaction, [signer]);

}



createAccount();