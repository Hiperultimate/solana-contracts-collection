import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  createMint,
  TOKEN_2022_PROGRAM_ID,
  getMint,
  mintTo,
  Mint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  getAccount,
} from "@solana/spl-token";
import { randomBytes } from "crypto";

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
  let vault_a_ata : anchor.web3.PublicKey;

  // escrow details
  // let escrowBytes = randomBytes(8);
  // let escrowSeedBuffer = new anchor.BN(escrowBytes, "le");
  let escrowSeed = new anchor.BN(randomBytes(8));
  console.log("Checking escrow seed : ", Array.from(escrowSeed.toArrayLike(Buffer, "le", 8)));
  const escrowDetails = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), user_a.publicKey.toBuffer(), escrowSeed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];

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
      (10 ** 6) * 100,
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
      (10 ** 6) * 100,
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    let vault_ata_a = getAssociatedTokenAddressSync(mint_a_details.address, escrowDetails, true);

    console.log("Tokens minted successfully to user_a : ", mintATx);
    console.log("Tokens minted successfully to user_b : ", mintBTx);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("User creator creates a new escrow", async () => {
    const tokenAAmount = new anchor.BN(50 * 10 ** 6);
    const tokenBAmount = new anchor.BN(40 * 10 ** 6);
    const makeEscrowResponse = await program.methods
      .makeEscrow(tokenAAmount, tokenBAmount, escrowSeed)
      .accounts({
        maker: user_a.publicKey,
        mintA: mint_a_details.address,
        mintB: mint_b_details.address,
        tokenProgram: TOKEN_2022_PROGRAM_ID
        // tokenMintA: mint_a_details.address,
        // tokenMintB: mint_b_details.address,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
        // associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        // systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([user_a])
      .rpc();

    console.log("Escrow made successfully : ", makeEscrowResponse);

    await confirmTx(makeEscrowResponse, provider.connection);

    // check details in escrow_details
    // const seedBuffer = escrow_seed.toArrayLike(Buffer, "le", 8);
    // const escrowSeeds = [Buffer.from("escrow"), user_a.publicKey.toBuffer(), seedBuffer]
    // const escrowSeeds = [Buffer.from("escrow"), user_a.publicKey.toBuffer()]
    // const [escrowAccount, _] = anchor.web3.PublicKey.findProgramAddressSync(escrowSeeds, program.programId);
    // const escrowDetails = await program.account.escrowDetails.fetch(escrowAccount)
    // // console.log("Escrow account details :", escrowDetails);

    // expect(escrowDetails.mintA.toBase58()).eq(mint_a_details.address.toBase58());
  
      
    // // get escrowDetail.address mint_a ATA
    // const vaultAtaAaccount = getAssociatedTokenAddressSync(mint_a_details.address, escrowAccount, true);
    // const vaultAtaADetails = await getAccount(provider.connection, vaultAtaAaccount, "confirmed", TOKEN_2022_PROGRAM_ID);
    // console.log("Vault details :", JSON.stringify(vaultAtaADetails));
  });


  it("Taker submits their share of the token to escrow", async () => {
    // const takeEscrowTx = await program.methods
    //   .takeEscrow(escrowSeedBuffer)
    //   .accounts({
    //     signer: user_b.publicKey,
    //     escrowOwner: user_a.publicKey,
    //     tokenMintA: mint_a_details.address,
    //     tokenMintB: mint_b_details.address,
    //     tokenProgram: TOKEN_2022_PROGRAM_ID
    //   })
    //   .signers([user_b])
    //   .rpc()

    // console.log("Successfully traded tokens : ", takeEscrowTx);
  } )
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


const confirmTx = async (signature: string, connection : anchor.web3.Connection): Promise<string> => {
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    signature,
    ...block,
  });
  return signature;
};