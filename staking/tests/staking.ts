import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import { before } from "mocha";
import {
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

describe("staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.staking as Program<Staking>;

  // create an admin
  let admin = provider.wallet;
  let tokenPubKey : anchor.web3.PublicKey;
  const MINT_DECIMALS = 6;
  const REWARD_RATE = 100;
  const ADMIN_TOKEN_AMOUNT = 1_000_000;

  before(async () => {
    // create an spl token 2022
    tokenPubKey = await createMint(
      provider.connection,
      admin.payer,
      admin.publicKey,
      null,
      MINT_DECIMALS,
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // create admin_ata for that spl token
    const adminATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin.payer,
      tokenPubKey,
      admin.publicKey,
      false,
      "confirmed",
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // mint 1_000_000 token to spl token to admin
    await mintTo(
      provider.connection,
      admin.payer,
      tokenPubKey,
      adminATA.address,
      admin.publicKey,
      ADMIN_TOKEN_AMOUNT,
      undefined,
      { commitment: "confirmed" },
      TOKEN_2022_PROGRAM_ID
    );

    // Check how much SPL_token does admin has
    const adminAccountDetails = await getAccount(
      provider.connection,
      adminATA.address,
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );
    console.log(
      "Checking admin SPL-Token balance : ",
      adminAccountDetails.amount.toString()
    );
  });

  it("Initialize staking with admin", async () => {
    const tx = await program.methods.initialize()
      .accounts({
        admin : admin.publicKey,
      })
      .signers([admin.payer])
      .rpc();
    
    console.log("Token initialized : ", tx);
  });

  it("Initilize SPL Token Pool ", async () => {
    // const initPoolTx = await program.methods.initializePool(new anchor.BN(REWARD_RATE), new anchor.BN(ADMIN_TOKEN_AMOUNT))
    //   .accounts({
    //     admin : admin.publicKey,
    //     tokenMint: tokenPubKey,
    //     tokenProgram: TOKEN_2022_PROGRAM_ID
    //   })
    //   .signers([admin.payer])
    //   .rpc();
    
    // console.log("Initializing pool : ", initPoolTx);
  })

    // initialize the admin on program.methods.initialize_pool() with admin creating a staking contract of that spl token
    // register it on staking program
    // create client
    // create client_ata
    // mint 100_000 token to client_ata
    // call program.method.create_user for client
    // call program.method.stake for client and stake 100_000
    // wait for 5 seconds
    // call program.method.claim_reward and confirm if they got the correct amount
});
