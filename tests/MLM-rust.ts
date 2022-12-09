import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MlmRust } from "../target/types/mlm_rust";

describe("MLM-rust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MlmRust as Program<MlmRust>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
