import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createMintToInstruction,
  createMint,
  TOKEN_2022_PROGRAM_ID,
  getMint,
  mintTo,
  Mint,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.escrow as Program<Escrow>;
  const provider = anchor.getProvider();
  const tokenMaster = provider.wallet;
  const user_a = anchor.web3.Keypair.generate();
  const user_b = anchor.web3.Keypair.generate();

  let mint_a_details: Mint;
  let mint_b_details: Mint;

  let user_a_ata: anchor.web3.PublicKey;
  let user_b_ata: anchor.web3.PublicKey;

  before(async () => {
    await airdropUser(user_a.publicKey, 100, provider.connection);
    await airdropUser(user_b.publicKey, 100, provider.connection);
    mint_a_details = await createSPLToken(
      provider.connection,
      tokenMaster.payer,
      6
    );
    mint_b_details = await createSPLToken(
      provider.connection,
      tokenMaster.payer,
      6
    );

    // Create user_a ata
    user_a_ata = await createAssociatedTokenAccount(
      provider.connection,
      user_a,
      mint_a_details.address,
      user_a.publicKey,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Create user_b ata
    user_b_ata = await createAssociatedTokenAccount(
      provider.connection,
      user_b,
      mint_b_details.address,
      user_b.publicKey,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Transfer token_a to user_a
    const mintATx = await mintTo(
      provider.connection,
      tokenMaster.payer,
      mint_a_details.address,
      user_a_ata,
      tokenMaster.publicKey,
      10 * 6 * 100,
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Transfer token_b to user_b
    const mintBTx = await mintTo(
      provider.connection,
      tokenMaster.payer,
      mint_b_details.address,
      user_b_ata,
      tokenMaster.publicKey,
      10 * 6 * 100,
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    console.log("Tokens minted successfully to user_a : ", mintATx);
    console.log("Tokens minted successfully to user_b : ", mintBTx);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("User creator creates a new escrow", async () => {
    // const makeEscrowResponse = await program.methods
    //   .makeEscrow()
    //   .accounts()
    //   .signers([user_a])
    //   .rpc();
  });
});

async function airdropUser(
  userPubKey: anchor.web3.PublicKey,
  valueInSol: number,
  connection: anchor.web3.Connection
) {
  const valueInLamports = valueInSol * anchor.web3.LAMPORTS_PER_SOL;
  try {
    await connection.requestAirdrop(
      userPubKey,
      100 * anchor.web3.LAMPORTS_PER_SOL
    );
  } catch (error) {
    console.log("User airdrop failed : ", valueInLamports);
  }
}

async function createSPLToken(
  connection: anchor.web3.Connection,
  payer: anchor.web3.Keypair,
  decimals: number,
  amountToMint?: number
) {
  // const mintKeypair = anchor.web3.Keypair.generate();
  // const lamports = await getMinimumBalanceForRentExemptMint(connection);

  // const createMintTransaction = new anchor.web3.Transaction().add(
  //     anchor.web3.SystemProgram.createAccount({
  //         fromPubkey: payer.publicKey,
  //         newAccountPubkey: mintKeypair.publicKey,
  //         space: MINT_SIZE,
  //         lamports,
  //         programId: TOKEN_PROGRAM_ID,
  //     }),
  //     createInitializeMintInstruction(
  //         mintKeypair.publicKey,
  //         decimals,
  //         payer.publicKey, // Mint Authority
  //         null,             // Freeze Authority (null for no freeze authority)
  //         TOKEN_PROGRAM_ID
  //     )
  // );

  // await anchor.web3.sendAndConfirmTransaction(connection, createMintTransaction, [payer, mintKeypair]);
  // console.log("Mint Address:", mintKeypair.publicKey.toBase58());
  // return mintKeypair.publicKey;

  const mint = await createMint(
    connection,
    payer,
    payer.publicKey,
    null, // Use freezeAuthority.publicKey or null
    decimals,
    undefined,
    { commitment: "confirmed" },
    TOKEN_2022_PROGRAM_ID // Specify Token-2022 program ID
  );

  console.log("Token-2022 Mint Address:", mint.toBase58());

  // Optional: Verify the mint details
  const mintInfo = await getMint(
    connection,
    mint,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );
  return mintInfo;
}
