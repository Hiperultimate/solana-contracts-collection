import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { assert, expect } from "chai";
import { getVaultDetails } from "./helperFns";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;
  const provider = anchor.getProvider();
  const LAMPORTS_PER_SOL = anchor.web3.LAMPORTS_PER_SOL;

  const user = provider.wallet.payer;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    const userBalance = await provider.connection.getBalance(user.publicKey);
    console.log('Checking user balance : ', userBalance / LAMPORTS_PER_SOL);
  });

  it("user creates vault", async () => {
    const result = await program.methods.createVault()
    .accounts({
      signer : user.publicKey
    })
    .signers([user])
    .rpc();

    console.log("Vault created successfully : ", result);

    // Derive the wallet for vault and check if it exists
    const vaultDetails = await getVaultDetails({
      userPubKey : user.publicKey, 
      programId : program.programId, 
      provider : provider
    })

    expect(vaultDetails.owner.toBase58()).eq(program.programId.toBase58());
  })

  it("user submits lamport to vault", async () => {
    const amountOfLamports = new anchor.BN(LAMPORTS_PER_SOL * 100);
    const result = await program.methods.submitLamports(amountOfLamports)
    .accounts({
      signer: user.publicKey
    })
    .signers([user])
    .rpc();
    console.log(`Successfully submitted ${amountOfLamports.toNumber() / LAMPORTS_PER_SOL} SOL to vault : `, result);
  
    // const userBalanceAfterTransfer = await provider.connection.getBalance(user.publicKey);
    // console.log(`Users balance after transferring lamports to vault : ${userBalanceAfterTransfer / LAMPORTS_PER_SOL}`, );

    const vaultDetails = await getVaultDetails({
      userPubKey : user.publicKey, 
      programId : program.programId, 
      provider : provider
    })

    expect(vaultDetails.owner.toBase58()).eq(program.programId.toBase58())
    expect(vaultDetails.lamports).gte(amountOfLamports.toNumber());
  })

  it("user withdraws more lamports than vault has ", async () => {
    const amountToWithdrawInSol = 200;
    const amountToWithdraw = new anchor.BN(amountToWithdrawInSol * LAMPORTS_PER_SOL);
    try {
      const withdrawResponse = await program.methods.withdrawLamports(amountToWithdraw)
        .accounts({
          signer: user.publicKey
        })
        .signers([user])
        .rpc();
      
      console.log("Amount withdrawn succesfully : ", withdrawResponse);
      assert.fail("This test should have failed.");
    } catch (error) {
      const anchorError = error as anchor.AnchorError;
      expect(error).to.be.instanceOf(anchor.AnchorError);
      expect(anchorError.error.errorCode.code).eq("InvalidLamportsRequested");
    }
  })

  it("user withdraws partial amount from vault ", async () => {
    const amountToWithdrawInSol = 50;
    const amountToWithdraw = new anchor.BN(amountToWithdrawInSol * LAMPORTS_PER_SOL);

    const withdrawResponse = await program.methods.withdrawLamports(amountToWithdraw)
      .accounts({
        signer: user.publicKey
      })
      .signers([user])
      .rpc();
    
    console.log("Amount withdrawn succesfully : ", withdrawResponse);
    
    const vaultDetails = await getVaultDetails({
      userPubKey : user.publicKey, 
      programId : program.programId, 
      provider : provider
    })

    const closeToAmount = 50 * LAMPORTS_PER_SOL;
    const deltaAmount = 2 * LAMPORTS_PER_SOL;
    expect(vaultDetails.lamports).closeTo(closeToAmount, deltaAmount);


    const userBalance = await provider.connection.getBalance(user.publicKey);
    console.log("Final amount left with the user : ", userBalance / LAMPORTS_PER_SOL);
  })

  it("close vault and retrieve all the leftover funds ", async () => {
    const initialVaultDetails = await getVaultDetails({
      programId : program.programId,
      provider : provider,
      userPubKey:  user.publicKey
    });

    const closeToAmount = 50 * LAMPORTS_PER_SOL;
    const deltaAmount = 2 * LAMPORTS_PER_SOL;
    // Vault at least have 50 SOL already
    expect(initialVaultDetails.lamports).closeTo(closeToAmount, deltaAmount).gt(49 * LAMPORTS_PER_SOL);
    const initialUserBalance = await provider.connection.getBalance(user.publicKey);

    const closeVaultTx = await program.methods.closeVault()
      .accounts({
        signer: user.publicKey
      })
      .signers([user])
      .rpc();
    
    console.log("Succesfully closed vault tx: ", closeVaultTx);

    // Check if the user received the remaining vault balance 
    const finalUserBalance = await provider.connection.getBalance(user.publicKey);
    expect(finalUserBalance - initialUserBalance).closeTo(closeToAmount,deltaAmount).gt(49 * LAMPORTS_PER_SOL);
  })
});
