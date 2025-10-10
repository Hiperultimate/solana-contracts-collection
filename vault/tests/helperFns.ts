import * as anchor from "@coral-xyz/anchor";

export async function getVaultDetails({
  userPubKey,
  programId,
  provider
}: {
  userPubKey: anchor.web3.PublicKey;
  programId: anchor.web3.PublicKey;
  provider: anchor.Provider
}) {
  const vaultSeed = [Buffer.from("vault"), userPubKey.toBuffer()];
  const [vaultAddress, _vaultBump] =
    anchor.web3.PublicKey.findProgramAddressSync(vaultSeed, programId);
  const vaultDetails = await provider.connection.getAccountInfo(vaultAddress);

  return vaultDetails;
}
