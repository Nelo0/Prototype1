import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Attempt1 } from "../target/types/attempt1"
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { expect } from "chai"
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"

describe("attempt1", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)

  const program = anchor.workspace.Attempt1 as Program<Attempt1>

  const [walletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [utf8.encode("wallet"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  it("initialize", async () => {
    const tx = await program.methods.initialize().rpc()

    const account = await program.account.wallet.fetch(walletPDA)
    expect(account.initializer === provider.wallet.publicKey)
  });

  it("sendLamports", async () => {
    const destinationAccount = Keypair.generate()

    // Airdrop SOL to walletPDA
    await provider.connection.requestAirdrop(walletPDA, 2 * LAMPORTS_PER_SOL)

    // Call PDA to send SOL to destinationAccount
    const tx = await program.methods
      .sendLamports(new anchor.BN(LAMPORTS_PER_SOL))
      .accounts({
        sendingWallet: walletPDA, 
        receiver: destinationAccount.publicKey
      })
      .rpc()

    // Check SOL is received
    expect(
      await provider.connection.getBalance(destinationAccount.publicKey)
    ).to.equal(LAMPORTS_PER_SOL);
  })

  it("closeAccount", async () => {
    const tx = await program.methods.closeAccount().rpc()
  })
});
