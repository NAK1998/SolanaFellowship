const web3 = require("@solana/web3.js");
const connection = new web3.Connection("http://localhost:8899", "confirmed");
const userWallet = web3.Keypair.generate();

const transferSOL = async (from, to, transferAmt) => {
    try {
        const transaction = new web3.Transaction().add(
            web3.SystemProgram.transfer({
                fromPubkey: new web3.PublicKey(from.publicKey.toString()),
                toPubkey: new web3.PublicKey(to.publicKey.toString()),
                lamports: transferAmt * web3.LAMPORTS_PER_SOL
            })
        )
        const signature = await web3.sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )
        return signature;
    } catch (err) {
        console.log(err);
    }
}

const getWalletBalance = async (pubk) => {
    try {
        const balance = await connection.getBalance(new web3.PublicKey(pubk));
        return balance / web3.LAMPORTS_PER_SOL;
    } catch (err) {
        console.log(err);
    }
}