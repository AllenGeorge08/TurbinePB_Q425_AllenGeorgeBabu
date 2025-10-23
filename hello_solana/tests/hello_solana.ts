import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";
import * as assert from "assert";
import { PublicKey } from "@solana/web3.js";

describe("hello_solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloSolana as Program<HelloSolana>;
  const user = anchor.web3.Keypair.generate();

  before(async () => {
    await airdrop(user.publicKey);
  });

  it("Is initialized!", async () => {
    const name = "John Doe";
    // get PDA for initialize account
    const [pda] = getInitializeAddress(user.publicKey, program.programId, name);

    // initialize account
    await program.methods
      .initialize(name)
      .accounts({
        greetingAccount: pda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user]) // this is the wallet signing the transaction
      .rpc();

    // fetch the new greeting account
    const greetingAccount = await program.account.greetingAccount.fetch(pda);
    // console.log("Greeting Account:", greetingAccount);

    // check that fetched name matches the name we initialized with
    assert.ok(name === greetingAccount.name);
  });

  it("Updates greeting", async () => {
    // initialize account
    const name = "Jane Doe";
    const [pda] = getInitializeAddress(user.publicKey, program.programId, name);
    await program.methods
      .initialize(name)
      .accounts({
        greetingAccount: pda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user]) // this is the wallet signing the transaction
      .rpc();
    // fetch the new greeting account
    const greetingAccount = await program.account.greetingAccount.fetch(pda);
    // confirm that fetched name matches the name we initialized with
    assert.ok(name === greetingAccount.name);

    // update greeting
    const newName = "Jennifer Doe";
    // Use the original name to get the correct PDA (not the new name)
    const [updatePda] = getInitializeAddress(user.publicKey, program.programId, name);
    await program.methods.greet(newName).accounts({
      greetingAccount: updatePda,
      user: user.publicKey,
    })
    .signers([user]) // this is the wallet signing the transaction
    .rpc();

    // fetch the updated greeting account
    const updatedGreetingAccount = await program.account.greetingAccount.fetch(updatePda);
    // confirm that fetched name matches the name we updated with
    assert.ok(newName === updatedGreetingAccount.name);
  });

  /* helpers */

  // returns PDA of account
  const getInitializeAddress = (user: PublicKey, programID: PublicKey, name: string) => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("greeting"), Buffer.from(name), user.toBuffer()],
      programID
    );
  };

  // function to airdrop 1 SOL to a user
  const airdrop = async (publicKey: anchor.web3.PublicKey) => {
    const sig = await program.provider.connection.requestAirdrop(
      publicKey,
      1_000_000_000 // 1 SOL
    );
    await program.provider.connection.confirmTransaction(sig, "confirmed");
  };
});
