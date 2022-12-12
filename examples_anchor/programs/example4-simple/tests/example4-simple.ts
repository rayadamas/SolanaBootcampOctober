import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Example4Simple } from "../target/types/example4_simple";

describe("example4-simple", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Example4Simple as Program<Example4Simple>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
