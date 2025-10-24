import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cinepass } from "../target/types/cinepass";
import { Connection, Keypair,LAMPORTS_PER_SOL,PublicKey, SYSVAR_EPOCH_SCHEDULE_PUBKEY } from "@solana/web3.js";
import { assert } from "chai";

describe("cinepass", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

 
  const program = anchor.workspace.cinepass as Program<Cinepass>;
  const user = anchor.web3.Keypair.generate();
  let counterPDA: PublicKey;

  async function getCounterPDA(signer: anchor.web3.PublicKey){
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), signer.toBuffer()],
      program.programId
    );
  }

  function sleep(ms: number){
    return new Promise(resolve => setTimeout(resolve,ms));
  }

  async function airdropSol(publicKey: PublicKey, amount: number =2 ){
    console.log(`Airdropping ${amount} SOL to ${publicKey.toString()}`);

    const airdropSignature = await provider.connection.requestAirdrop(
      publicKey,
      amount*LAMPORTS_PER_SOL
    )

    await sleep(10000);

    await provider.connection.confirmTransaction({
      signature: airdropSignature,
      blockhash: (await provider.connection.getLatestBlockhash()).blockhash,
      lastValidBlockHeight: (await provider.connection.getLatestBlockhash()).lastValidBlockHeight
    });

    console.log('Airdrop Confirmed');
  }


  before(async() => {
    await airdropSol(user.publicKey);
    [counterPDA] = await getCounterPDA(user.publicKey);
  });


  it("Is initialized!", async () => {
    await airdropSol(user.publicKey);

    const [counterPDA] = await getCounterPDA(user.publicKey);

    const tx = await program.methods.initialize().accounts({
      signer: user.publicKey
    }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counter.fetch(counterPDA);

    assert.equal(account.nftCount.toNumber(),0);
  });

  it("Increment", async() => {
    const [counterPda]  = await getCounterPDA(user.publicKey);
    const tx = await program.methods.increment().accounts({
      signer:  user.publicKey,
    }).signers([user]).rpc();
    console.log("Your transaction signature",tx);

    const account  = await program.account.counter.fetch(counterPda);
    assert.equal(account.nftCount.toNumber(),1);
  })

    it("Decrement", async() => {
    const [counterPda]  = await getCounterPDA(user.publicKey);
    const tx = await program.methods.decrement().accounts({
      signer:  user.publicKey,
    }).signers([user]).rpc();
    console.log("Your transaction signature",tx);

    const account  = await program.account.counter.fetch(counterPda);
    assert.equal(account.nftCount.toNumber(),0);
  })
});
