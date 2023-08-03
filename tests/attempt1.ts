import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Attempt1 } from "../target/types/attempt1";
import { expect } from "chai";

describe("attempt1", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Attempt1 as Program<Attempt1>

  const [walletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("initialize", async () => {
    const tx = await program.methods.initialize().rpc()

    const account = await program.account.wallet.fetch(walletPDA)
    expect(account.initializer === provider.wallet.publicKey)
  });

  it("closeAccount", async () => {
    const tx = await program.methods.closeAccount().rpc()
  })
});
