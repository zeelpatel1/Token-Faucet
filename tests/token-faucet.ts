import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

import {
  LAMPORTS_PER_SOL,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";

import { BN } from "bn.js";
import { TokenFaucet } from "../target/types/token_faucet";

describe("token-faucet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenFaucet as Program<TokenFaucet>;

  const user = Keypair.generate();

  let mint: PublicKey;
  let userTokenAccount: PublicKey;

  let faucetPDA: PublicKey;
  let faucetBump: number;

  let vaultPDA: PublicKey;

  before(async () => {
    console.log("Airdropping SOL to test wallet...");
    const sig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    console.log("Creating test token mint...");
    mint = await createMint(
      provider.connection,
      user, // payer
      user.publicKey, // mint authority
      null, // freeze authority
      6 // decimals
    );

    console.log("Creating ATA...");
    const ata = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint,
      user.publicKey
    );
    userTokenAccount = ata.address;

    console.log("Minting tokens into user ATA...");
    await mintTo(
      provider.connection,
      user,
      mint,
      userTokenAccount,
      user.publicKey,
      1000_000_000 // 1000 tokens with 6 decimals
    );

    console.log("Deriving PDA addresses...");
    [faucetPDA, faucetBump] = await PublicKey.findProgramAddress(
      [Buffer.from("faucet_metadata"), mint.toBuffer()],
      program.programId
    );

    [vaultPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("faucet_vault"), mint.toBuffer()],
      program.programId
    );

    console.log("Setup complete.");
  });

  it("Initializes the faucet", async () => {
    console.log("Calling initialize_faucet...");

    await program.methods
      .initializeFaucet(
        new BN(4),  // drip_amount
        new BN(10)  // cooldown_seconds
      )
      .accounts({
        user: user.publicKey,
        userTokenMint: mint,
        userTokenAta: userTokenAccount,
        faucet: faucetPDA,
        vaultFaucet: vaultPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    const faucet = await program.account.faucet.fetch(faucetPDA);
    console.log("Faucet account state:", faucet);

    // Assertions
    if (!faucet) throw new Error("Faucet not initialized!");

    console.log("Faucet initialized successfully!");
  });
});
