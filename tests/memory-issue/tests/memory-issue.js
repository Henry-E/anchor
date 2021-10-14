const anchor = require('@project-serum/anchor');
const Token = require("@solana/spl-token");

describe('memory-issue', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.MemoryIssue;

    const [pubkeyAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("pubkey_account")],
      program.programId
    );
    const [anotherPubkeyAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("another_pubkey_account")],
      program.programId
    );

    console.log("init struct");
    await program.rpc.initializeStruct(
      {
        accounts: {
          authority: program.provider.wallet.publicKey,
          pubkeyAccount,
          anotherPubkeyAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
      }
    );

    console.log("init tokens");

    const [openOrders] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("open_orders")],
      program.programId
    );

    const [thisMint] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("this_mint")],
      program.programId
    );

    const [token1] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_1")],
      program.programId
    );
    const [token2] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_2")],
      program.programId
    );
    const [token3] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_3")],
      program.programId
    );
    const [token4] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_4")],
      program.programId
    );
    const [token5] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_5")],
      program.programId
    );
    const [token6] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("token_6")],
      program.programId
    );

    const tx = await program.rpc.initializeTokens(
      {
        accounts: {
          authority: program.provider.wallet.publicKey,
          pubkeyAccount,
          anotherPubkeyAccount,
          openOrders,
          thisMint,
          token1,
          token2,
          token3,
          token4,
          token5,
          token6,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: Token.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        }
      });
    console.log("Your transaction signature", tx);
  });
});
