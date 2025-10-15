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
import { expect } from "chai";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.escrow as Program<Escrow>;
  const provider = anchor.getProvider();
  const tokenMaster = provider.wallet;
  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();

  let mint_a_details: Mint;
  let mint_b_details: Mint;

  let maker_ata: anchor.web3.PublicKey;
  let taker_ata: anchor.web3.PublicKey;
  let vault_a_ata : anchor.web3.PublicKey;

  const initialTokenAmount = new anchor.BN(100);

  // escrow details
  let escrowBytes = randomBytes(8);
  let escrowSeedBuffer = new anchor.BN(escrowBytes, "le");
  const escrowBufferSeeds = [Buffer.from("escrow"), maker.publicKey.toBuffer(), escrowSeedBuffer.toArrayLike(Buffer, "le", 8)];
  const escrowDetailsPubKey = anchor.web3.PublicKey.findProgramAddressSync(
    escrowBufferSeeds,
    program.programId
  )[0];

  before(async () => {
    await airdropUser(maker.publicKey, 100, provider.connection);
    await airdropUser(taker.publicKey, 100, provider.connection);
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

    // Create maker ata
    maker_ata = await createAssociatedTokenAccount(
      provider.connection,
      maker,
      mint_a_details.address,
      maker.publicKey,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Create taker ata
    taker_ata = await createAssociatedTokenAccount(
      provider.connection,
      taker,
      mint_b_details.address,
      taker.publicKey,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Transfer token_a to maker
    const mintATx = await mintTo(
      provider.connection,
      tokenMaster.payer,
      mint_a_details.address,
      maker_ata,
      tokenMaster.publicKey,
      (10 ** 6) * initialTokenAmount.toNumber(),
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Transfer token_b to taker
    const mintBTx = await mintTo(
      provider.connection,
      tokenMaster.payer,
      mint_b_details.address,
      taker_ata,
      tokenMaster.publicKey,
      (10 ** 6) * initialTokenAmount.toNumber(),
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    vault_a_ata = getAssociatedTokenAddressSync(mint_a_details.address, escrowDetailsPubKey, true, TOKEN_2022_PROGRAM_ID);

    console.log("Tokens minted successfully to maker : ", mintATx);
    console.log("Tokens minted successfully to taker : ", mintBTx);
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
       .makeEscrow(tokenAAmount, tokenBAmount, escrowSeedBuffer)
       .accounts({
         maker: maker.publicKey,
         mintA: mint_a_details.address,
         mintB: mint_b_details.address,
         tokenProgram: TOKEN_2022_PROGRAM_ID,
       })
      .signers([maker])
      .rpc();

    console.log("Escrow made successfully : ", makeEscrowResponse);

    await confirmTx(makeEscrowResponse, provider.connection);

    const escrowDetails = await program.account.escrowDetails.fetch(escrowDetailsPubKey)
    expect(escrowDetails.mintA.toBase58()).eq(mint_a_details.address.toBase58());
  
    const vaultAtaADetails = await getAccount(provider.connection, vault_a_ata, "confirmed", TOKEN_2022_PROGRAM_ID);
    expect(vaultAtaADetails.mint.toBase58()).eq(mint_a_details.address.toBase58());
    expect(new anchor.BN(vaultAtaADetails.amount).eq(tokenAAmount)).true;

    // check if maker ATA balance has been reduced or not
    const makerAtaADetails = await getAccount(provider.connection, maker_ata, "confirmed", TOKEN_2022_PROGRAM_ID);
    const leftoverTokenAamount = new anchor.BN((10**6) * initialTokenAmount.toNumber()).sub(tokenAAmount);
    expect(new anchor.BN(makerAtaADetails.amount).eq(leftoverTokenAamount)).true;
  });


  it("Taker accepts the deal and submits their share of the token to escrow", async () => {
    const takeEscrowTx = await program.methods
      .takeEscrow(escrowSeedBuffer)
      .accounts({
        taker: taker.publicKey,
        escrowOwner: maker.publicKey,
        tokenMintA: mint_a_details.address,
        tokenMintB: mint_b_details.address,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .signers([taker])
      .rpc()

    console.log("Successfully traded tokens : ", takeEscrowTx);

    await confirmTx(takeEscrowTx, provider.connection);

    // Check both ATAs and see whether they have received the payment or not
    const tokenAAmount = new anchor.BN(50 * (10 ** 6)); // taker should have this amount of token_a
    const tokenBAmount = new anchor.BN(40 * (10 ** 6)); // maker should have this amount of token_b

    // get ata_b for maker
    const makerAtaBPubkey = getAssociatedTokenAddressSync(mint_b_details.address, maker.publicKey, false, TOKEN_2022_PROGRAM_ID);
    const makerAtaBDetails = await getAccount(provider.connection, makerAtaBPubkey, "confirmed", TOKEN_2022_PROGRAM_ID);
    
    expect(makerAtaBDetails.mint.toBase58()).eq(mint_b_details.address.toBase58());
    expect(new anchor.BN(makerAtaBDetails.amount).eq(tokenBAmount)).true;

    // get ata_a for taker
    const takerAtaAPubkey = getAssociatedTokenAddressSync(mint_a_details.address, taker.publicKey, false, TOKEN_2022_PROGRAM_ID);
    const takerAtaADetails = await getAccount(provider.connection, takerAtaAPubkey, "confirmed", TOKEN_2022_PROGRAM_ID);
    
    expect(takerAtaADetails.mint.toBase58()).eq(mint_a_details.address.toBase58());
    expect(new anchor.BN(takerAtaADetails.amount).eq(tokenAAmount)).true;
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