import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import { MlmRust } from "../target/types/mlm_rust";
import {assert} from "chai";

async function send_lamports(account, program, connection) {
    let signature_account = await program.provider.connection.requestAirdrop(
        account.publicKey,
        1000000000,
    );
    const latestBlockHash = await connection.getLatestBlockhash();
    await program.provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: signature_account,
    });
}

describe("MLM-rust", () => {
    const program = anchor.workspace.MlmRust as Program<MlmRust>;
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = program.provider.connection;

    let owner, user1, user2, user3, user4, user5, user2_2, user2_3, level_comissions

    before(async function () {
        owner = anchor.web3.Keypair.generate()
        user1 = anchor.web3.Keypair.generate()
        user2 = anchor.web3.Keypair.generate()
        user3 = anchor.web3.Keypair.generate()
        user4 = anchor.web3.Keypair.generate()
        user5 = anchor.web3.Keypair.generate()
        user2_2 = anchor.web3.Keypair.generate()
        user2_3 = anchor.web3.Keypair.generate()

        level_comissions = [10, 7, 5, 2, 1, 1, 1, 1, 1, 1]
    })

    it("Is initialized!", async () => {
        const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature", tx);
    });

    it("Users had been signed up", async () => {
        const tx1 = await program.methods.signup(user2.publicKey, user1.publicKey).rpc()
        const tx2 = await program.methods.signup(user2_2.publicKey, user1.publicKey).rpc()
        const tx3 = await program.methods.signup(user2_3.publicKey, user1.publicKey).rpc()
        if (tx1 && tx2 && tx3) {
            console.log("Users had been signed up")
        }
    });

    it("User has 0 ether by default", async function() {
        const balance_of_account = await connection.getBalance(user1.publicKey)
        console.log("Starting balance:", balance_of_account.toString())
        assert.equal(balance_of_account, 0)
    })

    it("Should check that transaction (investing) went through", async function() {
        const amount = 1;
        let user_address = user1.publicKey;
        // @ts-ignore
        await program.methods.invest(amount, user_address)
        let accBalance = await connection.getBalance(user_address)
        assert.equal(accBalance, 0.95)
        console.log("transaction => successfull")
        console.log("Balance after sending transaction:", accBalance.toString())
    })

    it("Should withdraw all funds", async function() {
        send_lamports(user3, program, connection)
        console.log("Balance of user before withdraw: ", await connection.getBalance(user3.publicKey))
        await program.methods.withdraw(user3)
        assert.equal(await connection.getBalance(user3.publicKey), 0)
    })

});
