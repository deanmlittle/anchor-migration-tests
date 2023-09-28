import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js"
import { MigrationTests } from "../target/types/migration_tests";

describe("migration-tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MigrationTests as Program<MigrationTests>;
  const signer = new Keypair();
  const data = PublicKey.findProgramAddressSync([Buffer.from("account"), signer.publicKey.toBuffer()], program.programId)[0];


  it("Airdrop", async () => {
    await anchor.getProvider().connection.requestAirdrop(signer.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL).then(confirmTx)
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts(
      {
        signer: signer.publicKey,
        data,
        systemProgram: SystemProgram.programId
      }
    ).signers([signer]).rpc().then(confirmTx);
    console.log("Your transaction signature", tx);
    // Deserialize to prove migration worked
    await program.account.typeB.fetch(data);
  });

  it("Realloc big to small!", async () => {
    // Add your test here.
    const tx = await program.methods.migrateBigToSmall().accounts({
      signer: signer.publicKey,
      data,
      systemProgram: SystemProgram.programId
    }).signers([signer]).rpc().then(confirmTx);    
    console.log("Your transaction signature", tx);
    // Deserialize to prove migration worked
    await program.account.typeA.fetch(data);
  });

  it("Realloc small to big!", async () => {
    // Add your test here.
    const tx = await program.methods.migrateSmallToBig().accounts({
      signer: signer.publicKey,
      data,
      systemProgram: SystemProgram.programId
    }).signers([signer]).rpc().then(confirmTx);
    console.log("Your transaction signature", tx);

    // Deserialize to prove migration worked
    await program.account.typeB.fetch(data);
  });

  it("Realloc big to bigger!", async () => {
    // Add your test here.
    const tx = await program.methods.migrateBigToBigger().accounts({
      signer: signer.publicKey,
      data,
      systemProgram: SystemProgram.programId
    }).signers([signer]).rpc().then(confirmTx);
    console.log("Your transaction signature", tx);

    // Deserialize to prove migration worked
    await program.account.typeC.fetch(data);
  });
});

const confirmTx = async (signature: string): Promise<string> => {
  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  )
  return signature
}
