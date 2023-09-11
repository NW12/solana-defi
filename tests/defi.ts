import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Defi } from "../target/types/defi";

import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import fs from "fs";
import { Keypair } from "@solana/web3.js";

import {
  closeAccountInstructionData,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
const receiptMint = new anchor.web3.PublicKey(
  "DAYN3qFmf2q3PpGHVWKiXi7tthgahXfjDkzJno9tF2ox"
);

// describe("defi", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.Defi as Program<Defi>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

//

describe("defi", () => {
  // Configure the client to use the local cluster.

  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.Defi as Program<Defi>;
  const user5 = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      34, 190, 48, 111, 97, 113, 83, 136, 188, 13, 70, 81, 53, 181, 218, 57,
      168, 214, 4, 120, 233, 81, 7, 231, 159, 7, 35, 60, 15, 172, 36, 33, 109,
      172, 88, 169, 223, 83, 74, 187, 20, 220, 71, 102, 178, 161, 60, 199, 251,
      203, 28, 9, 206, 104, 107, 252, 45, 211, 127, 132, 0, 123, 151, 232,
    ])
  );

  const user4 = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      135, 9, 173, 50, 198, 24, 243, 13, 231, 75, 250, 9, 93, 184, 127, 20, 233,
      166, 146, 29, 182, 242, 206, 244, 62, 158, 78, 151, 142, 225, 235, 238,
      255, 134, 69, 136, 45, 120, 1, 102, 105, 82, 225, 137, 137, 101, 176, 195,
      241, 225, 20, 132, 27, 133, 37, 173, 142, 163, 114, 208, 171, 175, 81, 38,
    ])
  );
  // it("It creates the program 💰 token bag", async () => {
  //   console.log("user 5 is  ", user5.publicKey.toBase58());

  //   console.log("the program id is ", program.programId);
  //   const mint = new anchor.web3.PublicKey(
  //     "653ofEaJ3f8gE5aDwbNzaXgMgvW9uQBoB1pb1Tgrppb8"
  //   );
  //   const [lpBagPDA, finoBagBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [mint.toBuffer()],
  //       program.programId
  //     );
  //   console.log("program pda is ", lpBagPDA);
  //   const user = user5;
  //   await program.methods
  //     .createLpTokenBag(mint)
  //     .accounts({
  //       lpMint: mint,
  //       programLpTokenBag: lpBagPDA,
  //       payer: user.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       rent: SYSVAR_RENT_PUBKEY,
  //     })
  //     .signers([user])
  //      .rpc({
  //        skipPreflight: true,
  //     });

  // });

  // it("It creates the program LP💰 token bag", async () => {
  //   console.log("user is  ",user5.publicKey);
  //   const [lpBagPDA, finoBagBump] = await PublicKey.findProgramAddress(
  //     [
  //       new anchor.web3.PublicKey(
  //         "CGmyyfX8mRtzFCxjFt2CvoFsUpAMDNxBF235p3JxScAP"
  //       ).toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   console.log("program pda is ", lpBagPDA);
  //   const user = user5;
  //   await program.methods
  //     .createLpTokenBag(new anchor.web3.PublicKey("CGmyyfX8mRtzFCxjFt2CvoFsUpAMDNxBF235p3JxScAP"))
  //     .accounts({
  //       lpMint: new anchor.web3.PublicKey(
  //         "CGmyyfX8mRtzFCxjFt2CvoFsUpAMDNxBF235p3JxScAP"
  //       ),
  //       programLpTokenBag: lpBagPDA,
  //       payer: user.publicKey,
  //       systemProgram: SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       rent: SYSVAR_RENT_PUBKEY,
  //     })
  //     .signers([user])
  //     .rpc();
  //   console.log("fino Program Account ", lpBagPDA);
  // });
  // it("It creates user Profile", async () => {
  //   const user = user5;

  //   console.log("use is  ", user.publicKey.toBase58());

  //   console.log("the program id is ", program.programId);
  //   const mint = new anchor.web3.PublicKey(
  //     "9ysjcYcujHZLRWqHAfbdh3sg46zBTVhKkSfLiF7h34Px"
  //   );
  //   const [userProfile, finoBagBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode(  "userProfile"), user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   console.log("user profile  pda is ", userProfile);
  //   await program.methods
  //     .intializeUserProfil()
  //     .accounts({
  //       user: user.publicKey,
  //       userProfile:userProfile,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .signers([user])
  //     .rpc();
  //   const data = await program.account.userProfile.all();
  //   console.log("The data fo all account is ", data);

  //   const fetchdata = await program.account.userProfile.fetch(userProfile);
  //   console.log("The data for pda account is ", fetchdata);
  // });

  // it("it Borrow amount", async () => {
  // const user = user4;
  // const borrowMint = new anchor.web3.PublicKey(
  //   "8cNscppBjFQGYAupzkmhSvBYEQnrvoQzXsLPMhodBWD8"
  // );
  // const [borrowPDA, borrowBump] =
  //   await anchor.web3.PublicKey.findProgramAddress(
  //     [borrowMint.toBuffer()],
  //     program.programId
  //   );
  // const [userProfile, userProfileBump] =
  //   await anchor.web3.PublicKey.findProgramAddress(
  //     [utf8.encode("userProfile"), user.publicKey.toBuffer()],
  //     program.programId
  //   );
  // const [userMintProfile, userMintProfileBump] =
  //   await anchor.web3.PublicKey.findProgramAddress(
  //     [utf8.encode("userMintProfile0"), borrowMint.toBuffer(),user.publicKey.toBuffer()],
  //     program.programId
  //   );
  // const [userBorrow, _] = await anchor.web3.PublicKey.findProgramAddress(
  //   [
  //     utf8.encode("borrow"),
  //     borrowMint.toBuffer(),
  //     user.publicKey.toBuffer(),
  //     utf8.encode("3"),
  //   ],
  //   program.programId
  // );
  // const usesrBorrowBag= await getOrCreateAssociatedTokenAccount(connection,user,borrowMint,user.publicKey);
  // const res= await program.account.userProfile.fetch(userProfile);
  // console.log("Res is ",res);
  // await program.methods
  //   .borrow(
  //     borrowMint,
  //     borrowBump,
  //     userProfileBump,
  //     userMintProfileBump,
  //     "3",
  //     1.4
  //   )
  //   .accounts({
  //     borrow: userBorrow,
  //     programBorrowMintBag: borrowPDA,
  //     borrowMint: borrowMint,
  //     userProfile: userProfile,
  //     userBorrowBag:usesrBorrowBag.address,
  //    userMintProfile: userMintProfile,
  //     user: user.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).signers([user]).rpc();

  //   const data = await program.account.userProfile.fetch(userProfile);
  //   console.log("The data is ",data);

  //   const datav2 = await program.account.userMintProfile.fetch(userMintProfile);
  //   console.log("The data is ",datav2);

  //   const datav3 = await program.account.borrow.fetch(userBorrow);
  //   console.log("The borrow data is ",datav3);

  // });
  // it("It creates user mint Profile", async () => {
  //   const user = user5;

  //   console.log("user is  ", user.publicKey.toBase58());

  //   console.log("the program id is ", program.programId);
  //   const mint = new anchor.web3.PublicKey(
  //     "BoiVVmmdsFbSssoDFwYNm5Yb7k3xMHb9pJEh3zjEbyAV"
  //   );
  //   const [userProfile, _] = await anchor.web3.PublicKey.findProgramAddress(
  //     [utf8.encode("userMintProfile0"), mint.toBuffer(), user.publicKey.toBuffer()],
  //     program.programId
  //   );
  //   console.log("user profile  pda is ", userProfile);
  //   const [programMintPda, programPdaBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [mint.toBuffer()],
  //       program.programId
  //     );
  //   await program.methods
  //     .intializeUserMintProfile(mint, programPdaBump)
  //     .accounts({
  //       user: user.publicKey,
  //       userMintProfile: userProfile,
  //       programLpTokenBag: programMintPda,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .signers([user])
  //     .rpc();
  //   const dat = await program.account.userMintProfile.all();
  //   console.log("The data fo all account is ", dat);

  //   const fetchdata = await program.account.userMintProfile.fetch(userProfile);
  //   console.log("The data for pda account is ", fetchdata);
  // });

  //  it("Create Mints ", async () => {
  //   const user=user5;
  //   console.log("user5",user.publicKey);
  //   const receiptData = JSON.parse(
  //     fs.readFileSync("mint.json", { encoding: "utf8", flag: "r" })
  //   );
  //   const receiptMintKeypair = Keypair.fromSecretKey(new Uint8Array(receiptData));
  //   const receiptMintAddress = receiptMintKeypair.publicKey;
  //   const [receiptPDA, receiptPDABump] = await PublicKey.findProgramAddress(
  //     [receiptMintAddress.toBuffer()],
  //     program.programId
  //   );

  //   const receiptMint = await createMintAcct(
  //     receiptMintKeypair,
  //     receiptPDA,user,connection)

  // console.log(`receipt  Mint Address: ${receiptMint}`);
  // });

  // it("provide Supply to program", async () => {
  //   const user = user5;
  //   console.log("user  is ", user5.publicKey);
  //   const mint = new anchor.web3.PublicKey(
  //     "BoiVVmmdsFbSssoDFwYNm5Yb7k3xMHb9pJEh3zjEbyAV"
  //   );
  //   const [programPda, programBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [mint.toBuffer()],
  //       program.programId
  //     );
  //   console.log("Program pda account is ", programPda);
  //   const [userProfile, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userProfile"), user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userMintProfile, userProfilePMintBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("userMintProfile0"),
  //         mint.toBuffer(),
  //         user.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );
  //   const [supply, supplyBump] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       utf8.encode("supply"),
  //       mint.toBuffer(),
  //       user.publicKey.toBuffer(),
  //       utf8.encode("6"),
  //     ],
  //     program.programId
  //   );
  //   const [receiptPDA, receiptPDABump] = await PublicKey.findProgramAddress(
  //     [receiptMint.toBuffer()],
  //     program.programId
  //   );
  //   console.log("Recpit pda is ", receiptPDA);
  //   console.log("supply pda is ", supply);
  //   const userMintAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     user,
  //     mint,
  //     user.publicKey,
  //     false
  //   );
  //   // const userReceiptBag = await getOrCreateAssociatedTokenAccount(
  //   //   connection,
  //   //   user,
  //   //   receiptMint,
  //   //   user.publicKey,
  //   //   false
  //   // );
  //   //console.log("User receiptBag account  is ", userReceiptBag);
  //   console.log("User mint account  is ", userMintAccount);
  //   await program.methods
  //     .supply(
  //       mint,
  //       programBump,
  //       "1",
  //       userProfileBump,
  //       userProfilePMintBump,
  //       receiptPDABump,
  //       1.0
  //     )
  //     .accounts({
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       programMintTokenBag: programPda,
  //       userProfile: userProfile,
  //       userMintProfile: userMintProfile,
  //       userMintTokenAcc: userMintAccount.address,
  //       supply: supply,
  //    //   receiptMint: receiptMint,
  //      // receiptMintAuthority: receiptPDA,
  //      // userReceiptBag: userReceiptBag.address,
  //       mint: mint,
  //       user: user.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([user])
  //     .rpc({
  //       skipPreflight: true,
  //     });

  //   const data = await program.account.userProfile.fetch(userProfile);

  //   const data2 = await program.account.userMintProfile.fetch(userMintProfile);
  //   const data3 = await program.account.supply.fetch(supply);
  //   console.log("user main profile is ", data);
  //   console.log("user mint profile is ", data2);
  //   console.log("supply data3 is ", data3);
  // });
  // it("it Borrow amount", async () => {
  //   const user = user4;
  //   const borrowMint = new anchor.web3.PublicKey(
  //     "8cNscppBjFQGYAupzkmhSvBYEQnrvoQzXsLPMhodBWD8"
  //   );
  //   const [borrowPDA, borrowBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [borrowMint.toBuffer()],
  //       program.programId
  //     );
  //   const [userProfile, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userProfile"), user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userMintProfile, userMintProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userMintProfile0"), borrowMint.toBuffer(),user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userBorrow, _] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       utf8.encode("borrow"),
  //       borrowMint.toBuffer(),
  //       user.publicKey.toBuffer(),
  //       utf8.encode("3"),
  //     ],
  //     program.programId
  //   );
  //   const usesrBorrowBag= await getOrCreateAssociatedTokenAccount(connection,user,borrowMint,user.publicKey);
  //   const res= await program.account.userProfile.fetch(userProfile);
  //   console.log("Res is ",res);
  //   await program.methods
  //     .borrow(
  //       borrowMint,
  //       borrowBump,
  //       userProfileBump,
  //       userMintProfileBump,
  //       "3",
  //       1.4
  //     )
  //     .accounts({
  //       borrow: userBorrow,
  //       programBorrowMintBag: borrowPDA,
  //       borrowMint: borrowMint,
  //       userProfile: userProfile,
  //       userBorrowBag:usesrBorrowBag.address,
  //      userMintProfile: userMintProfile,
  //       user: user.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }).signers([user]).rpc();

  //     const data = await program.account.userProfile.fetch(userProfile);
  //     console.log("The data is ",data);

  //     const datav2 = await program.account.userMintProfile.fetch(userMintProfile);
  //     console.log("The data is ",datav2);

  //     const datav3 = await program.account.borrow.fetch(userBorrow);
  //     console.log("The borrow data is ",datav3);

  // });

  // it("it repay borrow amount", async () => {
  //   const user = user4;
  //   const borrowMint = new anchor.web3.PublicKey(
  //     "8cNscppBjFQGYAupzkmhSvBYEQnrvoQzXsLPMhodBWD8"
  //   );
  //   const [borrowPDA, borrowBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [borrowMint.toBuffer()],
  //       program.programId
  //     );
  //   const [userProfile, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userProfile"), user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userMintProfile, userMintProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userMintProfile0"), borrowMint.toBuffer(),user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userBorrow, userborrowBump] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       utf8.encode("borrow"),
  //       borrowMint.toBuffer(),
  //       user.publicKey.toBuffer(),
  //       utf8.encode("1"),
  //     ],
  //     program.programId
  //   );
  //   const usesrBorrowBag= await getOrCreateAssociatedTokenAccount(connection,user,borrowMint,user.publicKey);
  //   const res= await program.account.userProfile.fetch(userProfile);
  //   console.log("Res is ",res);
  //   await program.methods
  //     .repay(
  //       borrowMint,userborrowBump,
  //       borrowBump,
  //       userProfileBump,
  //       userMintProfileBump,
  //       "1"
  //     )
  //     .accounts({
  //       borrow: userBorrow,
  //       programBorrowMintBag: borrowPDA,
  //       borrowMint: borrowMint,
  //       userProfile: userProfile,
  //       userBorrowBag:usesrBorrowBag.address,
  //      userMintProfile: userMintProfile,
  //       user: user.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     }).signers([user]).rpc();
  //     const data = await program.account.userProfile.fetch(userProfile);
  //     console.log("The data is ",data);

  // });

  //  it("it withdraw supply ", async () => {
  //   const user = user5;

  //   console.log("user is ",user.publicKey);
  //   const supplyMint = new anchor.web3.PublicKey(
  //     "BoiVVmmdsFbSssoDFwYNm5Yb7k3xMHb9pJEh3zjEbyAV"
  //   );
  //   const [supplyPDA, ProgramMintBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [supplyMint.toBuffer()],
  //       program.programId
  //     );
  //   const [userProfile, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userProfile"), user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userMintProfile, userMintProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [utf8.encode("userMintProfile0"), supplyMint.toBuffer(),user.publicKey.toBuffer()],
  //       program.programId
  //     );
  //   const [userSupply, userSupplyBump] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       utf8.encode("supply"),
  //       supplyMint.toBuffer(),
  //       user.publicKey.toBuffer(),
  //       utf8.encode("6"),
  //     ],
  //     program.programId
  //   );
  // const usesrMintBag= await getOrCreateAssociatedTokenAccount(connection,user,supplyMint,user.publicKey);
  // // const usesrReceiptMintBag= await getOrCreateAssociatedTokenAccount(connection,user,receiptMint,user.publicKey);

  // const res= await program.account.supply.fetch(userSupply);
  // console.log("Res is ",res);
  // await program.methods
  //   .withdraw(supplyMint,userSupplyBump,userProfileBump,userMintProfileBump,"6",1,ProgramMintBump)
  //   .accounts({
  //     supply: userSupply,
  //     programMintTokenBag: supplyPDA,
  //     mint:supplyMint,
  //     userProfile: userProfile,
  //     userMintProfile:userMintProfile,
  //     tokenProgram:TOKEN_PROGRAM_ID,
  //    // recpitMint:receiptMint,
  //     user: user.publicKey,
  //     solUsdAccount: new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"),
  //     userMintTokenAcc:usesrMintBag.address,
  //    // userReceiptTokenAcc:new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"),//usesrReceiptMintBag.address,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).signers([user]).rpc({
  //     skipPreflight: true,
  //   });
  //   const data = await program.account.userProfile.fetch(userProfile);
  //   console.log("The data is ",data);
  //  }) 
     
});
const createMintAcct = async (
  keypairToAssign: Keypair,
  authorityToAssign: PublicKey,
  payer: Keypair,
  connection: anchor.web3.Connection
): Promise<PublicKey> => {
  return await createMint(
    connection,
    payer,
    authorityToAssign, // mint authority
    null, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
    9, // decimals
    keypairToAssign // address of the mint
  );
};
