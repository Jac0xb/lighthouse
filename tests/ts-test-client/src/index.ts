import { Address, address } from '@solana/addresses';
import { AccountRole, WritableSignerAccount } from '@solana/instructions';
import {
  SystemProgram,
  Transaction,
  VersionedTransaction,
  TransactionInstruction,
} from '@solana/web3.js';
import fs from 'fs';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { keccak_256 } from 'js-sha3';
import { toWeb3JsInstruction } from '@metaplex-foundation/umi-web3js-adapters';
import { u128, u8 } from '@metaplex-foundation/umi/serializers';
import {
  EquatableOperator,
  IntegerOperator,
  KnownProgram,
  LogLevel,
  accountInfoAssertion,
  dataValueAssertion,
  getAssertAccountDataInstruction,
  getAssertAccountInfoInstruction,
  findMemoryPda,
  getAssertAccountInfoMultiInstruction,
  getMemoryWriteInstruction,
  writeType,
  getAssertAccountDeltaInstruction,
  getAssertMerkleTreeAccountInstruction,
  getAssertMerkleTreeAccountInstructionRaw,
  LIGHTHOUSE_PROGRAM_ADDRESS,
  getAssertTokenAccountMultiInstruction,
  tokenAccountAssertion,
  mintAccountAssertion,
  stakeAccountAssertion,
  StakeStateType,
  getAssertStakeAccountInstruction,
  metaAssertion,
  getAssertUpgradeableLoaderAccountInstruction,
  upgradeableLoaderStateAssertion,
  UpgradeableLoaderStateType,
  accountDeltaAssertion,
  dataValueDeltaAssertion,
  ByteSliceOperator,
  getMemoryCloseInstruction,
  WriteType,
  AccountInfoField,
  ClockField,
} from 'lighthouse-sdk';
import {
  appendTransactionInstructions,
  createTransaction,
  pipe,
  setTransactionFeePayer,
  setTransactionLifetimeUsingBlockhash,
  signTransaction,
  IAccountMeta,
} from 'web3-js-preview';

(async () => {
  // let ix = AssertAccountDataBuilder::new()
  //     .target_account(test_account_key)
  //     .assertion(DataValueAssertion::U64 {
  //         value: 420, // The expected value.
  //         operator: IntegerOperator::GreaterThanOrEqual,
  //     })
  //     .offset(8) // The account data byte offset where 'balance' is stored.
  //     .instruction();

  const targetAccount = '' as any;

  // const ix = getAssertAccountDataInstruction({
  //   targetAccount,
  //   assertion: dataValueAssertion('U64', {
  //     value: 420,
  //     operator: IntegerOperator.GreaterThanOrEqual,
  //   }),
  //   offset: 8,
  // });

  // AssertAccountDataBuilder::new()
  //   .target_account(test_account_key)
  //   .assertion(DataValueAssertion::Pubkey {
  //       value: owner_key,
  //       operator: EquatableOperator::Equal,
  //   })
  //   .offset(16) // The account data byte offset where 'owner' is stored.
  //   .instruction(),

  // const ix = getAssertAccountDataInstruction({
  //   targetAccount,
  //   assertion: dataValueAssertion('Pubkey', {
  //     value: ownerPubkey,
  //     operator: 'Equal',
  //   }),
  //   offset: 16,
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(account_key)
  //   .assertion(AccountInfoAssertion::Lamports {
  //       value: 5_000_000,
  //       operator: IntegerOperator::GreaterThan,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('Lamports', {
  //     value: 5_000_000,
  //     operator: IntegerOperator.GreaterThan,
  //   }),
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(account_key)
  //   .assertion(AccountInfoAssertion::Owner {
  //       value: system_program::ID,
  //       operator: EquatableOperator::Equal,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('Owner', {
  //     value: SystemProgram.programId,
  //     operator: 'Equal',
  //   }),
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(account_key)
  //   .assertion(AccountInfoAssertion::KnownOwner {
  //       value: KnownProgram::System,
  //       operator: EquatableOperator::Equal,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('KnownOwner', {
  //     value: KnownProgram.System,
  //     operator: EquatableOperator.Equal,
  //   }),
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(account_key)
  //   .assertion(AccountInfoAssertion::RentEpoch {
  //       value: 0,
  //       operator: IntegerOperator::Equal,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('RentEpoch', {
  //     value: 0,
  //     operator: IntegerOperator.Equal,
  //   }),
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(user_key)
  //   .log_level(LogLevel::PlaintextMessage) // Logs assertion results.
  //   .assertion(AccountInfoAssertion::IsSigner {
  //       value: true,
  //       operator: EquatableOperator::Equal,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('IsSigner', {
  //     value: true,
  //     operator: EquatableOperator.Equal,
  //   }),
  // });

  // let ix = AssertAccountInfoBuilder::new()
  //   .target_account(account_key)
  //   .assertion(AccountInfoAssertion::IsWritable {
  //       value: true,
  //       operator: EquatableOperator::Equal,
  //   })
  //   .instruction();

  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('IsWritable', {
  //     value: true,
  //     operator: EquatableOperator.Equal,
  //   }),
  // });

  //   import { keccak_256 } from 'js-sha3'

  // const accountDataHash = Buffer.from(keccak_256.digest(accountDataBuffer))
  // const tx = assertAccountInfo(umi, {
  //   targetAccount,
  //   assertion: {
  //     __kind: 'VerifyDatahash',
  //     expectedHash: accountDataHash,
  //     start: null,
  //     length: null,
  //   },
  // }).build(umi)

  //   let hash = keccak::hashv(&[&account.data]).0;

  // let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertAccountInfoBuilder::new()
  //             .target_account(account_key)
  //             .assertion(AccountInfoAssertion::VerifyDatahash {
  //               expected_hash: hash,
  //                 start: None,
  //                 length: None,
  //             })
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const accountDataHash = Buffer.from(keccak_256.digest(accountDataBuffer));
  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('VerifyDatahash', {
  //     expectedHash: Buffer.from(accountDataHash),
  //     start: null,
  //     length: null,
  //   }),
  // });

  // const transaction = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) => setTransactionFeePayer(signer.address, tx),
  //   (tx) => appendTransactionInstructions([ix], tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([signer.keyPair], tx)
  // );

  //   let hash = keccak::hashv(&[&account.data[128..256]]).0;

  // let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertAccountInfoBuilder::new()
  //             .target_account(account_key)
  //             .assertion(AccountInfoAssertion::VerifyDatahash {
  //                 expected_hash: hash,
  //                 start: Some(128),
  //                 length: Some(128),
  //             })
  //             .instruction()
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const accountDataHash = Buffer.from(
  //   keccak_256.digest(accountDataBuffer.subarray(128, 256))
  // );
  // const ix = getAssertAccountInfoInstruction({
  //   targetAccount,
  //   assertion: accountInfoAssertion('VerifyDatahash', {
  //     expectedHash: Buffer.from(accountDataHash),
  //     start: 128,
  //     length: 128,
  //   }),
  // });

  // const transaction = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) => setTransactionFeePayer(signer.address, tx),
  //   (tx) => appendTransactionInstructions([ix], tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([signer.keyPair], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[AssertAccountInfoMultiBuilder::new()
  //         .target_account(user_key)
  //         .log_level(lighthouse_sdk::types::LogLevel::PlaintextMessage)
  //         .assertions(vec![
  //             AccountInfoAssertion::Owner {
  //                 value: system_program::ID,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             AccountInfoAssertion::KnownOwner {
  //                 value: KnownProgram::System,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             AccountInfoAssertion::Lamports {
  //                 value: user_prebalance - 5000,
  //                 operator: IntegerOperator::Equal,
  //             },
  //             AccountInfoAssertion::DataLength {
  //                 value: 0,
  //                 operator: IntegerOperator::Equal,
  //             },
  //             AccountInfoAssertion::Executable {
  //                 value: true,
  //                 operator: EquatableOperator::NotEqual,
  //             },
  //             AccountInfoAssertion::Executable {
  //                 value: false,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             AccountInfoAssertion::Executable {
  //                 value: true,
  //                 operator: EquatableOperator::NotEqual,
  //             },
  //             AccountInfoAssertion::RentEpoch {
  //                 value: account.rent_epoch,
  //                 operator: IntegerOperator::Equal,
  //             },
  //         ])
  //     .instruction()],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const ix = getAssertAccountInfoMultiInstruction({
  //   targetAccount,
  //   logLevel: LogLevel.PlaintextMessage,
  //   assertions: [
  //     accountInfoAssertion('Owner', {
  //       value: SystemProgram.programId,
  //       operator: 'Equal',
  //     }),
  //     accountInfoAssertion('KnownOwner', {
  //       value: KnownProgram.System,
  //       operator: 'Equal',
  //     }),
  //     accountInfoAssertion('Lamports', {
  //       value: userPrebalance - 5000,
  //       operator: 'Equal',
  //     }),
  //     accountInfoAssertion('DataLength', {
  //       value: 0,
  //       operator: 'Equal',
  //     }),
  //     accountInfoAssertion('Executable', {
  //       value: true,
  //       operator: 'NotEqual',
  //     }),
  //     accountInfoAssertion('Executable', {
  //       value: false,
  //       operator: 'Equal',
  //     }),
  //     accountInfoAssertion('Executable', {
  //       value: true,
  //       operator: 'NotEqual',
  //     }),
  //     accountInfoAssertion('RentEpoch', {
  //       value: account.rentEpoch,
  //       operator: 'Equal',
  //     }),
  //   ],
  // });

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) => setTransactionFeePayer(signer.address, tx),
  //   (tx) => appendTransactionInstructions([ix], tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([signer.keyPair], tx)
  // );

  // import {
  //   createLighthouseProgram,
  //   IntegerOperator,
  //   assertAccountDelta,
  //   memoryWrite,
  //   findMemoryPda,
  //   AccountInfoField,
  // } from 'lighthouse-sdk-legacy'
  // import { toWeb3JsInstruction } from '@metaplex-foundation/umi-web3js-adapters'

  // const umi = createUmi('https://api.mainnet-beta.solana.com')
  // umi.programs.add(createLighthouseProgram())

  // let tx = new Transaction()

  // let [memory, memoryBump] = findMemoryPda({
  //   payer: userPubkey,
  //   memoryId: 0,
  // })

  // tx.add(
  //   toWeb3JsInstruction(
  //     memoryWrite(umi, {
  //       memory: publicKey(memory),
  //       sourceAccount: publicKey(userPubkey),
  //       writeType: {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.Lamports],
  //       },
  //       memoryId: 0,
  //       writeOffset: 0,
  //       memoryBump,
  //     }).getInstructions()[0]
  //   )
  // )

  // tx.add(
  //   SystemProgram.transfer({
  //     fromPubkey: userPubkey,
  //     toPubkey: destinationPubkey,
  //     lamports: 1e9,
  //   })
  // )

  // tx.add(
  //   toWeb3JsInstruction(
  //     assertAccountDelta(umi, {
  //       accountA: publicKey(memory),
  //       accountB: publicKey(userPubkey),
  //       assertion: {
  //         __kind: 'AccountInfo',
  //         aOffset: 0,
  //         assertion: {
  //           __kind: 'Lamports',
  //           value: -1e9,
  //           operator: IntegerOperator.Equal,
  //         },
  //       },
  //     }).getInstructions()[0]
  //   )
  // )

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getMemoryWriteInstruction({
  //           memory,
  //           payer: userPubkey,
  //           sourceAccount: userPubkey,
  //           writeType: writeType('AccountInfoField', {
  //             fields: ['Lamports'],
  //           }),
  //           memoryId: 0,
  //           writeOffset: 0,
  //           memoryBump,
  //         }),
  //         SystemProgram.transfer({
  //           fromPubkey: userPubkey,
  //           toPubkey: destinationPubkey,
  //           lamports: 1e9,
  //         }),
  //         getAssertAccountDeltaInstruction({
  //           accountA: memory,
  //           accountB: userPubkey,
  //           assertion: {
  //             __kind: 'AccountInfo',
  //             aOffset: 0,
  //             assertion: {
  //               __kind: 'Lamports',
  //               value: -1e9,
  //               operator: IntegerOperator.Equal,
  //             },
  //           },
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userPubkey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // let ix = AssertAccountDeltaBuilder::new()
  //       .account_a(test_account_a)
  //       .account_b(test_account_b)
  //       .assertion(AccountDeltaAssertion::Data {
  //           a_offset: 8,    // The byte offset in account A to deserialize into a u64 (vault_balance).
  //           b_offset: 8,    // The byte offset in account B to deserialize into a u64 (vault_balance).
  //           assertion: DataValueDeltaAssertion::U64 {
  //               value: expected_diff_bound, // b.vault_balance - a.vault_balance
  //               operator: IntegerOperator::LessThan,
  //           },
  //       })
  //       .instruction()

  // const ix = getAssertAccountDeltaInstruction({
  //   accountA: testAccountA,
  //   accountB: testAccountB,
  //   assertion: {
  //     __kind: 'Data',
  //     aOffset: 8, // The byte offset in account A to deserialize into a u64 (vault_balance).
  //     bOffset: 8, // The byte offset in account B to deserialize into a u64 (vault_balance).
  //     assertion: {
  //       __kind: 'U64',
  //       value: expectedDiffBound, // b.vault_balance - a.vault_balance
  //       operator: IntegerOperator.LessThan,
  //     },
  //   },
  // });

  // const proofPathMetas: AccountMeta[] = proofPath.map((hash) => ({
  //   pubkey: publicKey(hash),
  //   isWritable: false,
  //   isSigner: false,
  // }))

  // const tx = assertMerkleTreeAccount(umi, {
  //   targetMerkleTree,
  //   root,
  //   splAccountCompression,
  //   assertion: {
  //     __kind: 'VerifyLeaf',
  //     leafIndex: leafIndex,
  //     leafHash: expectedLeafHash,
  //   },
  // })
  //   .addRemainingAccounts(proofPathMetas)
  //   .build(umi)

  // let ix = AssertMerkleTreeAccountBuilder::new()
  //     .target_merkle_tree(tree_pubkey)
  //     .root(Pubkey::new_from_array(tree_root)) // This can be any account, preferably one that already exists in the transaction. See the note above.
  //     .spl_account_compression(spl_account_compression::id())
  //     .assertion(MerkleTreeAssertion::VerifyLeaf {
  //         leaf_index: leaf.index,
  //         leaf_hash: expected_leaf_hash,  // This would be the leaf node with the state you expect at the end of the transaction.
  //     })
  //     .add_remaining_accounts(&proof_path_metas) // The proof path to verify the leaf node.
  //     .instruction()

  // const proofPath: IAccountMeta[] = [];

  // const ix = getAssertMerkleTreeAccountInstructionRaw(
  //   {
  //     targetMerkleTree: treePubkey,
  //     root: treeRoot,
  //     splAccountCompression: splAccountCompressionProgramId,
  //   },
  //   {
  //     assertion: {
  //       __kind: 'VerifyLeaf',
  //       leafIndex: leafIndex,
  //       leafHash: expectedLeafHash,
  //     },
  //   },
  //   LIGHTHOUSE_PROGRAM_ADDRESS,
  //   proofPath
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[AssertTokenAccountMultiBuilder::new()
  //         .target_account(token_account)
  //         .assertions(vec![
  //             TokenAccountAssertion::Mint {
  //                 value: mint_key,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             TokenAccountAssertion::Owner {
  //                 value: user_key,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             TokenAccountAssertion::Amount {
  //                 value: 100,
  //                 operator: IntegerOperator::Equal,
  //             },
  //             TokenAccountAssertion::Delegate {
  //                 value: None,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             TokenAccountAssertion::State {
  //                 value: TokenAccountState::Frozen as u8,
  //                 operator: IntegerOperator::NotEqual,
  //             },
  //             TokenAccountAssertion::IsNative {
  //                 value: None,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             TokenAccountAssertion::DelegatedAmount {
  //                 value: 0,
  //                 operator: IntegerOperator::LessThanOrEqual,
  //             },
  //             TokenAccountAssertion::CloseAuthority {
  //                 value: None,
  //                 operator: EquatableOperator::Equal,
  //             },
  //             TokenAccountAssertion::TokenAccountOwnerIsDerived,
  //         ])
  //         .instruction()],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  const mintKey = '' as any;
  const userKey = '' as any;

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertTokenAccountMultiInstruction({
  //           targetAccount,
  //           assertions: [
  //             tokenAccountAssertion('Mint', {
  //               value: mintKey,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             tokenAccountAssertion('Owner', {
  //               value: userKey,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             tokenAccountAssertion('Amount', {
  //               value: 100,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             tokenAccountAssertion('Delegate', {
  //               value: null,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             tokenAccountAssertion('State', {
  //               value: 3,
  //               operator: IntegerOperator.NotEqual,
  //             }),
  //             tokenAccountAssertion('IsNative', {
  //               value: null,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             tokenAccountAssertion('DelegatedAmount', {
  //               value: 0,
  //               operator: IntegerOperator.LessThanOrEqual,
  //             }),
  //             tokenAccountAssertion('CloseAuthority', {
  //               value: null,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             tokenAccountAssertion('TokenAccountOwnerIsDerived'),
  //           ],
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userPubkey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertMintAccountBuilder::new()
  //             .target_account(mint_key)
  //             .assertion(MintAccountAssertion::MintAuthority {
  //                 value: Some(user_key),
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertMintAccountBuilder::new()
  //             .target_account(mint_key)
  //             .assertion(MintAccountAssertion::Supply {
  //                 value: 69_000,
  //                 operator: IntegerOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertMintAccountBuilder::new()
  //             .target_account(mint_key)
  //             .assertion(MintAccountAssertion::Decimals {
  //                 value: 9,
  //                 operator: IntegerOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertMintAccountBuilder::new()
  //             .target_account(mint_key)
  //             .assertion(MintAccountAssertion::IsInitialized {
  //                 value: true,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertMintAccountBuilder::new()
  //             .target_account(mint_key)
  //             .assertion(MintAccountAssertion::FreezeAuthority {
  //                 value: None,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertTokenAccountMultiInstruction({
  //           targetAccount: mintKey,
  //           assertions: [
  //             mintAccountAssertion('MintAuthority', {
  //               value: userKey,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             mintAccountAssertion('Supply', {
  //               value: 69_000,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             mintAccountAssertion('Decimals', {
  //               value: 9,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             mintAccountAssertion('IsInitialized', {
  //               value: true,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             mintAccountAssertion('FreezeAuthority', {
  //               value: null,
  //               operator: EquatableOperator.Equal,
  //             }),
  //           ],
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx: Transaction = Transaction::new_signed_with_payer(
  //     &[
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::LockupCustodian {
  //                     value: meta.lockup.custodian,
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::LockupEpoch {
  //                     value: meta.lockup.epoch,
  //                     operator: IntegerOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::LockupUnixTimestamp {
  //                     value: meta.lockup.unix_timestamp,
  //                     operator: IntegerOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::AuthorizedStaker {
  //                     value: meta.authorized.staker,
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::AuthorizedWithdrawer {
  //                     value: meta.authorized.withdrawer,
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::MetaAssertion(
  //                 MetaAssertion::RentExemptReserve {
  //                     value: meta.rent_exempt_reserve,
  //                     operator: IntegerOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertTokenAccountMultiInstruction({
  //           targetAccount: stakeAccount,
  //           assertions: [
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupCustodian',
  //               value: meta.lockup.custodian,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupEpoch',
  //               value: meta.lockup.epoch,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupUnixTimestamp',
  //               value: meta.lockup.unixTimestamp,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'AuthorizedStaker',
  //               value: meta.authorized.staker,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'AuthorizedWithdrawer',
  //               value: meta.authorized.withdrawer,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'RentExemptReserve',
  //               value: meta.rentExemptReserve,
  //               operator: IntegerOperator.Equal,
  //             }),
  //           ],
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx: Transaction = Transaction::new_signed_with_payer(
  //     &[
  //         AssertStakeAccountBuilder::new()
  //             .target_account(stake_account)
  //             .assertion(StakeAccountAssertion::State {
  //                 value: StakeStateType::Stake,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertTokenAccountMultiInstruction({
  //           targetAccount: stakeAccount,
  //           assertions: [
  //             stakeAccountAssertion('State', {
  //               value: StakeStateType.Stake,
  //               operator: EquatableOperator.Equal,
  //             }),
  //           ],
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // let tx: Transaction = Transaction::new_signed_with_payer(
  //   &[
  //       AssertStakeAccountBuilder::new()
  //           .target_account(stake_account)
  //           .assertion(StakeAccountAssertion::StakeAssertion(
  //               StakeAssertion::CreditsObserved {
  //                   value: stake.credits_observed,
  //                   operator: IntegerOperator::Equal,
  //               },
  //           ))
  //           .instruction(),
  //       AssertStakeAccountBuilder::new()
  //           .target_account(stake_account)
  //           .assertion(StakeAccountAssertion::StakeAssertion(
  //               StakeAssertion::DelegationStake {
  //                   value: stake.delegation.stake,
  //                   operator: IntegerOperator::Equal,
  //               },
  //           ))
  //           .instruction(),
  //       AssertStakeAccountBuilder::new()
  //           .target_account(stake_account)
  //           .assertion(StakeAccountAssertion::StakeAssertion(
  //               StakeAssertion::DelegationDeactivationEpoch {
  //                   value: stake.delegation.deactivation_epoch,
  //                   operator: IntegerOperator::Equal,
  //               },
  //           ))
  //           .instruction(),
  //       AssertStakeAccountBuilder::new()
  //           .target_account(stake_account)
  //           .assertion(StakeAccountAssertion::StakeAssertion(
  //               StakeAssertion::DelegationActivationEpoch {
  //                   value: stake.delegation.activation_epoch,
  //                   operator: IntegerOperator::Equal,
  //               },
  //           ))
  //           .instruction(),
  //       AssertStakeAccountBuilder::new()
  //           .target_account(stake_account)
  //           .assertion(StakeAccountAssertion::StakeAssertion(
  //               StakeAssertion::DelegationVoterPubkey {
  //                   value: stake.delegation.voter_pubkey,
  //                   operator: EquatableOperator::Equal,
  //               },
  //           ))
  //           .instruction(),
  //   ],
  //   Some(&user_key),
  //   &[&user_keypair],
  //   blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeAssertion', {
  //             __kind: 'CreditsObserved',
  //             value: stake.creditsObserved,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeAssertion', {
  //             __kind: 'DelegationStake',
  //             value: stake.delegation.stake,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeAssertion', {
  //             __kind: 'DelegationDeactivationEpoch',
  //             value: stake.delegation.deactivationEpoch,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeAssertion', {
  //             __kind: 'DelegationActivationEpoch',
  //             value: stake.delegation.activationEpoch,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeAssertion', {
  //             __kind: 'DelegationVoterPubkey',
  //             value: stake.delegation.voterPubkey,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertTokenAccountMultiInstruction({
  //           targetAccount: stakeAccount,
  //           assertions: [
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupCustodian',
  //               value: meta.lockup.custodian,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupEpoch',
  //               value: meta.lockup.epoch,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'LockupUnixTimestamp',
  //               value: meta.lockup.unixTimestamp,
  //               operator: IntegerOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'AuthorizedStaker',
  //               value: meta.authorized.staker,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'AuthorizedWithdrawer',
  //               value: meta.authorized.withdrawer,
  //               operator: EquatableOperator.Equal,
  //             }),
  //             stakeAccountAssertion('MetaAssertion', {
  //               __kind: 'RentExemptReserve',
  //               value: meta.rentExemptReserve,
  //               operator: IntegerOperator.Equal,
  //             }),
  //           ],
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('LockupCustodian', {
  //             value: meta.lockup.custodian,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('LockupEpoch', {
  //             value: meta.lockup.epoch,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('LockupUnixTimestamp', {
  //             value: meta.lockup.unixTimestamp,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('AuthorizedStaker', {
  //             value: meta.authorized.staker,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('AuthorizedWithdrawer', {
  //             value: meta.authorized.withdrawer,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: metaAssertion('RentExemptReserve', {
  //             value: meta.rentExemptReserve,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // const tx = assertStakeAccount(umi, {
  //   targetAccount: publicKey(stakeAccount),
  //   assertion: {
  //     __kind: 'StakeFlags',
  //     value: 255,
  //     operator: IntegerOperator.DoesNotContain,
  //   },
  // })
  //   .append(
  //     assertStakeAccount(umi, {
  //       targetAccount: publicKey(stakeAccount),
  //       assertion: {
  //         __kind: 'StakeFlags',
  //         value: 0,
  //         operator: IntegerOperator.Contains,
  //       },
  //     })
  //   )
  //   .append(
  //     assertStakeAccount(umi, {
  //       targetAccount: publicKey(stakeAccount),
  //       assertion: {
  //         __kind: 'StakeFlags',
  //         value: 0,
  //         operator: IntegerOperator.Equal,
  //       },
  //     })
  //   )
  //   .build(umi)

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeFlags', {
  //             value: 255,
  //             operator: IntegerOperator.DoesNotContain,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeFlags', {
  //             value: 0,
  //             operator: IntegerOperator.Contains,
  //           }),
  //         }),
  //         getAssertStakeAccountInstruction({
  //           targetAccount: stakeAccount,
  //           assertion: stakeAccountAssertion('StakeFlags', {
  //             value: 0,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx: Transaction = Transaction::new_signed_with_payer(
  //     &[
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(upgradeable_loader_account)
  //             .assertion(UpgradeableLoaderStateAssertion::State {
  //                 value: UpgradeableLoaderStateType::Buffer,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: upgradeableLoaderAccount,
  //           assertion: upgradeableLoaderStateAssertion('State', {
  //             value: UpgradeableLoaderStateType.Buffer,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(program_key)
  //             .assertion(UpgradeableLoaderStateAssertion::State {
  //                 value: UpgradeableLoaderStateType::Buffer,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(program_key)
  //             .assertion(UpgradeableLoaderStateAssertion::Buffer(
  //                 UpgradableBufferAssertion::Authority {
  //                     value: Some(authority_key),
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programKey,
  //           assertion: upgradeableLoaderStateAssertion('State', {
  //             value: UpgradeableLoaderStateType.Buffer,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programKey,
  //           assertion: upgradeableLoaderStateAssertion('Buffer', {
  //             __kind: 'Authority',
  //             value: authorityKey,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(program_key)
  //             .assertion(UpgradeableLoaderStateAssertion::State {
  //                 value: UpgradeableLoaderStateType::Program,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(program_key)
  //             .assertion(UpgradeableLoaderStateAssertion::Program(
  //                 UpgradeableProgramAssertion::ProgramDataAddress {
  //                     value: programdata_key,
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programKey,
  //           assertion: upgradeableLoaderStateAssertion('State', {
  //             value: UpgradeableLoaderStateType.Program,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programKey,
  //           assertion: upgradeableLoaderStateAssertion('Program', {
  //             __kind: 'ProgramDataAddress',
  //             value: programDataKey,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(programdata_key)
  //             .assertion(UpgradeableLoaderStateAssertion::State {
  //                 value: UpgradeableLoaderStateType::ProgramData,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .instruction(),
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(programdata_key)
  //             .assertion(UpgradeableLoaderStateAssertion::ProgramData(
  //                 UpgradeableProgramDataAssertion::UpgradeAuthority {
  //                     value: Some(upgrade_authority),
  //                     operator: EquatableOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //         AssertUpgradeableLoaderAccountBuilder::new()
  //             .target_account(programdata_key)
  //             .assertion(UpgradeableLoaderStateAssertion::ProgramData(
  //                 UpgradeableProgramDataAssertion::Slot {
  //                     value: expected_slot,
  //                     operator: IntegerOperator::Equal,
  //                 },
  //             ))
  //             .instruction(),
  //     ],
  //     Some(&user_key),
  //     &[&user_keypair],
  //     blockhash,
  // );

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programDataKey,
  //           assertion: upgradeableLoaderStateAssertion('State', {
  //             value: UpgradeableLoaderStateType.ProgramData,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programDataKey,
  //           assertion: upgradeableLoaderStateAssertion('ProgramData', {
  //             __kind: 'UpgradeAuthority',
  //             value: upgradeAuthority,
  //             operator: EquatableOperator.Equal,
  //           }),
  //         }),
  //         getAssertUpgradeableLoaderAccountInstruction({
  //           targetAccount: programDataKey,
  //           assertion: upgradeableLoaderStateAssertion('ProgramData', {
  //             __kind: 'Slot',
  //             value: expectedSlot,
  //             operator: IntegerOperator.Equal,
  //           }),
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userKey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([userKeyPair], tx)
  // );

  // MemoryWriteBuilder::new()
  // .payer(user.encodable_pubkey())
  // .source_account(token_account_key)
  // .program_id(lighthouse_sdk::ID)
  // .memory(memory_key)
  // .memory_id(0)
  // .memory_bump(memory_bump)
  // .write_offset(0)
  // .write_type(WriteType::AccountData {
  //     offset: 0,
  //     data_length: 72,
  // })
  // .instruction()

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const ix = getMemoryWriteInstruction({
  //   payer: userPubkey,
  //   sourceAccount: tokenAccountKey,
  //   programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //   memory,
  //   memoryId: 0,
  //   memoryBump: memoryBump,
  //   writeOffset: 0,
  //   writeType: {
  //     __kind: 'AccountData',
  //     offset: 0,
  //     dataLength: 72,
  //   },
  // });

  //   let ixs = vec![
  //     AssertAccountDeltaBuilder::new()
  //         .account_a(memory_key)
  //         .account_b(token_account_key)
  //         .assertion(AccountDeltaAssertion::Data {
  //             a_offset: 0,
  //             b_offset: 0,
  //             assertion: DataValueDeltaAssertion::Bytes {
  //                 operator: ByteSliceOperator::Equal,
  //                 length: 64,
  //             },
  //         })
  //         .instruction(),
  //     AssertAccountDeltaBuilder::new()
  //         .account_a(memory_key)
  //         .account_b(token_account_key)
  //         .assertion(AccountDeltaAssertion::Data {
  //             a_offset: 64,
  //             b_offset: 64,
  //             assertion: DataValueDeltaAssertion::U64 {
  //                 value: -50,
  //                 operator: IntegerOperator::GreaterThan,
  //             },
  //         })
  //         .instruction(),
  // ];

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const ixs = [
  //   getAssertAccountDeltaInstruction({
  //     accountA: memory,
  //     accountB: tokenAccountKey,
  //     assertion: accountDeltaAssertion('Data', {
  //       aOffset: 0,
  //       bOffset: 0,
  //       assertion: dataValueDeltaAssertion('Bytes', {
  //         operator: ByteSliceOperator.Equal,
  //         length: 64,
  //       }),
  //     }),
  //   }),
  //   getAssertAccountDeltaInstruction({
  //     accountA: memory,
  //     accountB: tokenAccountKey,
  //     assertion: accountDeltaAssertion('Data', {
  //       aOffset: 64,
  //       bOffset: 64,
  //       assertion: dataValueDeltaAssertion('U64', {
  //         value: -50,
  //         operator: IntegerOperator.GreaterThan,
  //       }),
  //     }),
  //   }),
  // ];

  // MemoryCloseBuilder::new()
  // .payer(user.encodable_pubkey())
  // .program_id(lighthouse_sdk::ID)
  // .memory(memory)
  // .memory_bump(memory_bump)
  // .memory_id(0)
  // .instruction(),

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const ix = getMemoryCloseInstruction({
  //   payer: userPubkey,
  //   programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //   memory,
  //   memoryBump,
  //   memoryId: 0,
  // });

  //   let tx = Transaction::new_signed_with_payer(
  //     &[
  //         MemoryWriteBuilder::new()
  //             .payer(user.encodable_pubkey())
  //             .source_account(lighthouse_sdk::ID)
  //             .program_id(lighthouse_sdk::ID)
  //             .memory(memory)
  //             .memory_id(0)
  //             .memory_bump(memory_bump)
  //             .write_offset(0)
  //             .system_program(system_program::id())
  //             .write_type(WriteType::DataValue(DataValue::U128(u128::MAX)))
  //             .instruction(),
  //         MemoryWriteBuilder::new()
  //             .payer(user.encodable_pubkey())
  //             .source_account(lighthouse_sdk::ID)
  //             .program_id(lighthouse_sdk::ID)
  //             .memory(memory)
  //             .memory_id(0)
  //             .memory_bump(memory_bump)
  //             .write_offset(32)
  //             .system_program(system_program::id())
  //             .write_type(WriteType::DataValue(DataValue::Pubkey(
  //                 some_key,
  //             )))
  //             .instruction(),
  //         AssertAccountDataBuilder::new()
  //             .target_account(memory)
  //             .assertion(DataValueAssertion::U128 {
  //                 value: u128::MAX,
  //                 operator: IntegerOperator::Equal,
  //             })
  //             .offset(0)
  //             .instruction(),
  //         AssertAccountDataBuilder::new()
  //             .target_account(memory)
  //             .assertion(DataValueAssertion::Pubkey {
  //                 value: some_key,
  //                 operator: EquatableOperator::Equal,
  //             })
  //             .offset(32)
  //             .instruction(),
  //     ],
  //     Some(&user.encodable_pubkey()),
  //     &[&user],
  //     context.get_blockhash().await,
  // );

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getMemoryWriteInstruction({
  //           payer: userPubkey,
  //           sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           memory,
  //           memoryId: 0,
  //           memoryBump,
  //           writeOffset: 0,
  //           writeType: {
  //             __kind: 'DataValue',
  //             fields: [
  //               {
  //                 __kind: 'U128',
  //                 fields: [BigInt('340282366920938463463374607431768211455')],
  //               },
  //             ],
  //           },
  //         }),
  //         getMemoryWriteInstruction({
  //           payer: userPubkey,
  //           sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           memory,
  //           memoryId: 0,
  //           memoryBump,
  //           writeOffset: 32,
  //           writeType: {
  //             __kind: 'DataValue',
  //             fields: [
  //               {
  //                 __kind: 'Pubkey',
  //                 fields: [someKey],
  //               },
  //             ],
  //           },
  //         }),
  //         getAssertAccountDataInstruction({
  //           targetAccount: memory,
  //           assertion: dataValueAssertion('U128', {
  //             value: BigInt('340282366920938463463374607431768211455'),
  //             operator: IntegerOperator.Equal,
  //           }),
  //           offset: 0,
  //         }),
  //         getAssertAccountDataInstruction({
  //           targetAccount: memory,
  //           assertion: dataValueAssertion('Pubkey', {
  //             value: someKey,
  //             operator: EquatableOperator.Equal,
  //           }),
  //           offset: 32,
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userPubkey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([user], tx)
  // );

  // const [memory, memoryBump] = findMemoryPda({
  //   payer: publicKey(userPubkey),
  //   memoryId: 0,
  // })

  // const builderFn = (writeType: WriteType, offset: number) => {
  //   return memoryWrite(umi, {
  //     payer: publicKey(userPubkey),
  //     sourceAccount: publicKey(testAccountKey),
  //     programId: LIGHTHOUSE_PROGRAM_ID,
  //     memory: publicKey(memory),
  //     memoryId: 0,
  //     memoryBump,
  //     writeOffset: offset,
  //     writeType,
  //   })
  // }

  // const tx = builderFn(
  //   {
  //     __kind: 'AccountInfoField',
  //     fields: [AccountInfoField.DataLength],
  //   },
  //   0
  // )
  //   .append(
  //     builderFn(
  //       {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.Executable],
  //       },
  //       8
  //     )
  //   )
  //   .append(
  //     builderFn(
  //       {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.Owner],
  //       },
  //       16
  //     )
  //   )
  //   .append(
  //     builderFn(
  //       {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.Lamports],
  //       },
  //       48
  //     )
  //   )
  //   .append(
  //     builderFn(
  //       {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.RentEpoch],
  //       },
  //       56
  //     )
  //   )
  //   .append(
  //     builderFn(
  //       {
  //         __kind: 'AccountInfoField',
  //         fields: [AccountInfoField.Key],
  //       },
  //       64
  //     )
  //   )
  //   .build(umi)

  // let builder_fn = |write_type: WriteType, offset: u16| {
  //     MemoryWriteBuilder::new()
  //         .payer(user_key)
  //         .source_account(test_account_key)
  //         .program_id(lighthouse_sdk::ID)
  //         .memory(memory)
  //         .memory_id(0)
  //         .memory_bump(memory_bump)
  //         .write_offset(offset)
  //         .system_program(system_program::id())
  //         .write_type(write_type)
  //         .instruction()
  // };

  // let tx = Transaction::new_signed_with_payer(
  //     &[
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::DataLength), 0),
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::Executable), 8),
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::Owner), 16),
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::Lamports), 48),
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::RentEpoch), 56),
  //         builder_fn(WriteType::AccountInfoField(AccountInfoField::Key), 64),
  //     ],
  //     Some(&user.encodable_pubkey()),
  //     &[&user],
  //     context.get_blockhash().await,
  // );

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 0,
  //   payer: userPubkey,
  // });

  // const builderFn = (writeType: WriteType, offset: number) => {
  //   return getMemoryWriteInstruction({
  //     payer: userPubkey,
  //     sourceAccount: testAccountKey,
  //     programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //     memory,
  //     memoryId: 0,
  //     memoryBump,
  //     writeOffset: offset,
  //     writeType,
  //   });
  // };

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.DataLength],
  //           },
  //           0
  //         ),
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.Executable],
  //           },
  //           8
  //         ),
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.Owner],
  //           },
  //           16
  //         ),
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.Lamports],
  //           },
  //           48
  //         ),
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.RentEpoch],
  //           },
  //           56
  //         ),
  //         builderFn(
  //           {
  //             __kind: 'AccountInfoField',
  //             fields: [AccountInfoField.Key],
  //           },
  //           64
  //         ),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userPubkey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([user], tx)
  // );

  //   let tx = Transaction::new_signed_with_payer(
  //     &[MemoryWriteBuilder::new()
  //         .memory(memory_key)
  //         .memory_id(4)   // You can write to multiple memory accounts in a single transaction
  //         .memory_bump(memory_bump)
  //         .program_id(lighthouse_sdk::ID)
  //         .payer(user.encodable_pubkey())
  //         .source_account(lighthouse_sdk::ID) // This is ignore so should be an account already in the transaction to save transaction space.
  //         .write_offset(0)
  //         .write_type(WriteType::Clock(ClockField::Slot))
  //         .instruction()],
  //     Some(&user.encodable_pubkey()),
  //     &[&user],
  //     ctx.get_blockhash().await,
  // );

  // const [memory, memoryBump] = await findMemoryPda({
  //   memoryId: 4,
  //   payer: userPubkey,
  // });

  // const tx = await pipe(
  //   createTransaction({ version: 0 }),
  //   (tx) =>
  //     appendTransactionInstructions(
  //       [
  //         getMemoryWriteInstruction({
  //           memory,
  //           memoryId: 4,
  //           memoryBump,
  //           programId: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           payer: userPubkey,
  //           sourceAccount: LIGHTHOUSE_PROGRAM_ADDRESS,
  //           writeOffset: 0,
  //           writeType: {
  //             __kind: 'Clock',
  //             fields: [ClockField.Slot],
  //           },
  //         }),
  //       ],
  //       tx
  //     ),
  //   (tx) => setTransactionFeePayer(userPubkey, tx),
  //   (tx) => setTransactionLifetimeUsingBlockhash(recentBlockhash, tx),
  //   (tx) => signTransaction([user], tx)
  // );
})();
