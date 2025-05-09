use super::{
    clone_keypair, compute_metadata_hashes,
    tx_builder::{
        BurnBuilder, CancelRedeemBuilder, CollectionVerificationInner, CreateBuilder,
        DelegateBuilder, DelegateInner, MintToCollectionV1Builder, MintV1Builder, RedeemBuilder,
        SetTreeDelegateBuilder, TransferBuilder, TransferInner, TxBuilder, VerifyCollectionBuilder,
    },
    Error, LeafArgs, Result,
};
use anchor_lang::{self, AccountDeserialize};
use bytemuck::try_from_bytes;
use mpl_bubblegum::{
    state::{leaf_schema::LeafSchema, TreeConfig, Voucher, VOUCHER_PREFIX},
    utils::get_asset_id,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
};
use solana_program_test::{BanksClient, BanksTransactionResultWithMetadata};
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    signer::signers::Signers,
    transaction::Transaction,
};
use spl_account_compression::state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1;
use spl_concurrent_merkle_tree::concurrent_merkle_tree::ConcurrentMerkleTree;
use spl_merkle_tree_reference::{MerkleTree, Node};
use std::{convert::TryFrom, mem::size_of};

pub fn decompress_mint_auth_pda(mint_key: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[mint_key.as_ref()], &mpl_bubblegum::ID).0
}

pub struct Tree<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize> {
    pub tree_creator: Keypair,
    pub tree_delegate: Keypair,
    pub merkle_tree: Keypair,
    pub canopy_depth: u32,
    client: BanksClient,
    proof_tree: MerkleTree,
    num_minted: u64,
}

impl<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize> Tree<MAX_DEPTH, MAX_BUFFER_SIZE> {
    pub fn new(client: BanksClient) -> Self {
        Self::with_creator(&Keypair::new(), client)
    }

    pub fn with_creator(tree_creator: &Keypair, client: BanksClient) -> Self {
        let proof_tree = MerkleTree::new(vec![Node::default(); 1 << MAX_DEPTH].as_slice());

        Tree {
            tree_creator: clone_keypair(tree_creator),
            tree_delegate: clone_keypair(tree_creator),
            merkle_tree: Keypair::new(),
            canopy_depth: 0,
            client,
            proof_tree,
            num_minted: 0,
        }
    }

    pub fn creator_pubkey(&self) -> Pubkey {
        self.tree_creator.pubkey()
    }

    pub fn delegate_pubkey(&self) -> Pubkey {
        self.tree_delegate.pubkey()
    }

    pub fn clone_delegate(&self) -> Keypair {
        clone_keypair(&self.tree_delegate)
    }

    // Not to be confused with the `set_tree_delegate` below.
    pub fn replace_tree_delegate(&mut self, key: &Keypair) {
        self.tree_delegate = clone_keypair(key);
    }

    pub fn tree_pubkey(&self) -> Pubkey {
        self.merkle_tree.pubkey()
    }

    pub fn authority(&self) -> Pubkey {
        Pubkey::find_program_address(&[self.tree_pubkey().as_ref()], &mpl_bubblegum::ID).0
    }

    pub fn voucher(&self, nonce: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[
                VOUCHER_PREFIX.as_ref(),
                self.tree_pubkey().as_ref(),
                &nonce.to_le_bytes(),
            ],
            &mpl_bubblegum::ID,
        )
        .0
    }

    pub fn merkle_tree_account_size(&self) -> usize {
        CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1
            + size_of::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>()
    }

    pub async fn process_tx<T: Signers>(
        &mut self,
        instruction: Instruction,
        payer: &Pubkey,
        signing_keypairs: &T,
    ) -> Result<()> {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        self.client
            .process_transaction(Transaction::new_signed_with_payer(
                &[instruction],
                Some(payer),
                signing_keypairs,
                recent_blockhash,
            ))
            .await
            .map_err(|err| Box::new(Error::BanksClient(err)))
    }

    pub async fn rent(&mut self) -> Result<Rent> {
        self.client
            .get_rent()
            .await
            .map_err(|err| Box::new(Error::BanksClient(err)))
    }

    // Allocates and pays for an account to hold the tree.
    pub async fn alloc(&mut self, payer: &Keypair) -> Result<()> {
        let rent = self.rent().await?;
        let account_size = self.merkle_tree_account_size();

        // u64 -> usize conversion should never fail on the platforms we're running on.
        let lamports = rent.minimum_balance(account_size);

        let ix = system_instruction::create_account(
            &payer.pubkey(),
            &self.tree_pubkey(),
            lamports,
            // The `usize -> u64` conversion should never fail.
            u64::try_from(account_size).unwrap(),
            &spl_account_compression::id(),
        );

        let merkle_tree = Keypair::from_bytes(&self.merkle_tree.to_bytes()).unwrap();
        self.process_tx(ix, &payer.pubkey(), &[payer, &merkle_tree])
            .await
    }

    // Helper fn to instantiate the various `TxBuilder` based concrete types
    // associated with each operation.
    fn tx_builder<T, U, V>(
        &mut self,
        accounts: T,
        data: U,
        need_proof: Option<u32>,
        inner: V,
        payer: Pubkey,
        default_signers: &[&Keypair],
    ) -> TxBuilder<T, U, V, MAX_DEPTH, MAX_BUFFER_SIZE> {
        let def_signers = default_signers.iter().map(|k| clone_keypair(k)).collect();

        TxBuilder {
            accounts,
            additional_accounts: Vec::new(),
            data,
            payer,
            client: self.client.clone(),
            signers: def_signers,
            tree: self,
            need_proof,
            inner,
        }
    }

    pub fn create_tree_tx(
        &mut self,
        payer: &Keypair,
        public: bool,
    ) -> CreateBuilder<MAX_DEPTH, MAX_BUFFER_SIZE> {
        let accounts = mpl_bubblegum::accounts::CreateTree {
            tree_authority: self.authority(),
            payer: payer.pubkey(),
            tree_creator: self.creator_pubkey(),
            log_wrapper: spl_noop::id(),
            system_program: system_program::id(),
            compression_program: spl_account_compression::id(),
            merkle_tree: self.tree_pubkey(),
        };

        let data = mpl_bubblegum::instruction::CreateTree {
            max_depth: u32::try_from(MAX_DEPTH).unwrap(),
            max_buffer_size: u32::try_from(MAX_BUFFER_SIZE).unwrap(),
            public: Some(public),
        };

        self.tx_builder(accounts, data, None, (), payer.pubkey(), &[payer])
    }

    pub async fn create(&mut self, payer: &Keypair) -> Result<BanksTransactionResultWithMetadata> {
        self.create_tree_tx(payer, false).execute(&[], &[]).await
    }

    pub async fn create_public(
        &mut self,
        payer: &Keypair,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.create_tree_tx(payer, true).execute(&[], &[]).await
    }

    pub fn mint_v1_tx<'a>(
        &'a mut self,
        tree_delegate: &Keypair,
        args: &'a mut LeafArgs,
    ) -> MintV1Builder<MAX_DEPTH, MAX_BUFFER_SIZE> {
        let accounts = mpl_bubblegum::accounts::MintV1 {
            tree_authority: self.authority(),
            tree_delegate: tree_delegate.pubkey(),
            payer: args.owner.pubkey(),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::MintV1 {
            message: args.metadata.clone(),
        };

        let owner = clone_keypair(&args.owner);

        self.tx_builder(
            accounts,
            data,
            None,
            args,
            owner.pubkey(),
            &[tree_delegate, &owner],
        )
    }

    pub async fn mint_v1(
        &mut self,
        tree_delegate: &Keypair,
        args: &mut LeafArgs,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.mint_v1_tx(tree_delegate, args).execute(&[], &[]).await
    }

    #[allow(clippy::too_many_arguments)]
    pub fn mint_to_collection_v1_tx<'a>(
        &'a mut self,
        tree_delegate: &Keypair,
        args: &'a mut LeafArgs,
        collection_authority: &Keypair,
        collection_mint: Pubkey,
        collection_metadata: Pubkey,
        edition_account: Pubkey,
        collection_record: Option<Pubkey>,
    ) -> MintToCollectionV1Builder<MAX_DEPTH, MAX_BUFFER_SIZE> {
        let accounts = mpl_bubblegum::accounts::MintToCollectionV1 {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            payer: args.owner.pubkey(),
            tree_delegate: tree_delegate.pubkey(),
            collection_authority: collection_authority.pubkey(),
            collection_authority_record_pda: collection_record.unwrap_or(mpl_bubblegum::ID),
            collection_mint,
            collection_metadata,
            edition_account,
            bubblegum_signer: pubkey!("4ewWZC5gT6TGpm5LZNDs9wVonfUT2q5PP5sc9kVbwMAK"),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            token_metadata_program: mpl_token_metadata::ID,
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::MintToCollectionV1 {
            metadata_args: args.metadata.clone(),
        };

        let owner = clone_keypair(&args.owner);

        self.tx_builder(
            accounts,
            data,
            None,
            args,
            owner.pubkey(),
            &[tree_delegate, &owner, collection_authority],
        )
    }

    pub async fn decode_root(&mut self) -> Result<[u8; 32]> {
        let mut tree_account = self.read_account(self.tree_pubkey()).await?;

        let merkle_tree_bytes = tree_account.data.as_mut_slice();
        let (_header_bytes, rest) =
            merkle_tree_bytes.split_at_mut(CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1);

        let merkle_tree_size = size_of::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>();
        let tree_bytes = &mut rest[..merkle_tree_size];

        let tree = try_from_bytes::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>(tree_bytes)
            .map_err(Error::BytemuckPod)?;
        let root = tree.change_logs[tree.active_index as usize].root;

        Ok(root)
    }

    // This is currently async due to calling `decode_root` (same goes for a bunch of others).
    pub async fn burn_tx<'a>(
        &'a mut self,
        args: &'a LeafArgs,
    ) -> Result<BurnBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;

        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;

        let accounts = mpl_bubblegum::accounts::Burn {
            tree_authority: self.authority(),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::Burn {
            root,
            data_hash,
            creator_hash,
            nonce: args.nonce,
            index: args.index,
        };

        let need_proof = Some(args.index);

        Ok(self.tx_builder(
            accounts,
            data,
            need_proof,
            args,
            args.owner.pubkey(),
            &[&args.owner],
        ))
    }

    pub async fn verify_collection_tx<'a>(
        &'a mut self,
        args: &'a mut LeafArgs,
        collection_authority: &Keypair,
        collection_mint: Pubkey,
        collection_metadata: Pubkey,
        edition_account: Pubkey,
        collection_record: Option<Pubkey>,
    ) -> Result<VerifyCollectionBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;
        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;

        let accounts = mpl_bubblegum::accounts::CollectionVerification {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            payer: collection_authority.pubkey(),
            tree_delegate: self.tree_creator.pubkey(),
            collection_authority: collection_authority.pubkey(),
            collection_authority_record_pda: collection_record.unwrap_or(mpl_bubblegum::ID),
            collection_mint,
            collection_metadata,
            edition_account,
            bubblegum_signer: pubkey!("4ewWZC5gT6TGpm5LZNDs9wVonfUT2q5PP5sc9kVbwMAK"),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            token_metadata_program: mpl_token_metadata::ID,
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::VerifyCollection {
            root,
            data_hash,
            creator_hash,
            nonce: args.nonce,
            index: args.index,
            message: args.metadata.clone(),
        };

        let need_proof = Some(args.index);

        let inner = CollectionVerificationInner {
            args,
            collection_authority: collection_authority.pubkey(),
            collection_mint,
            collection_metadata,
            edition_account,
        };

        Ok(self.tx_builder(
            accounts,
            data,
            need_proof,
            inner,
            collection_authority.pubkey(),
            &[collection_authority],
        ))
    }

    pub async fn verify_collection(
        &mut self,
        args: &mut LeafArgs,
        collection_authority: &Keypair,
        collection_mint: Pubkey,
        collection_metadata: Pubkey,
        edition_account: Pubkey,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.verify_collection_tx(
            args,
            collection_authority,
            collection_mint,
            collection_metadata,
            edition_account,
            None,
        )
        .await?
        .execute(&[], &[])
        .await
    }

    pub async fn delegate_verify_collection(
        &mut self,
        args: &mut LeafArgs,
        collection_authority: &Keypair,
        collection_mint: Pubkey,
        collection_metadata: Pubkey,
        edition_account: Pubkey,
        collection_record: Pubkey,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.verify_collection_tx(
            args,
            collection_authority,
            collection_mint,
            collection_metadata,
            edition_account,
            Some(collection_record),
        )
        .await?
        .execute(&[], &[])
        .await
    }

    pub async fn transfer_tx<'a>(
        &'a mut self,
        args: &'a mut LeafArgs,
        new_leaf_owner: &Keypair,
    ) -> Result<TransferBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;
        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;

        let accounts = mpl_bubblegum::accounts::Transfer {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            new_leaf_owner: new_leaf_owner.pubkey(),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            merkle_tree: self.tree_pubkey(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::Transfer {
            root,
            data_hash,
            creator_hash,
            nonce: args.nonce,
            index: args.index,
        };

        let owner = clone_keypair(&args.owner);
        let new_owner = clone_keypair(new_leaf_owner);

        let need_proof = Some(args.index);
        let inner = TransferInner { args, new_owner };

        Ok(self.tx_builder(accounts, data, need_proof, inner, owner.pubkey(), &[&owner]))
    }

    pub async fn transfer(
        &mut self,
        args: &mut LeafArgs,
        new_owner: &Keypair,
        additonal_instructions: &[Instruction],
        additional_signers: &[Keypair],
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.transfer_tx(args, new_owner)
            .await?
            .execute(additonal_instructions, additional_signers)
            .await
    }

    pub async fn delegate_tx<'a>(
        &'a mut self,
        args: &'a mut LeafArgs,
        new_leaf_delegate: &Keypair,
    ) -> Result<DelegateBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;
        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;

        let accounts = mpl_bubblegum::accounts::Delegate {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            previous_leaf_delegate: args.delegate.pubkey(),
            new_leaf_delegate: new_leaf_delegate.pubkey(),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            merkle_tree: self.tree_pubkey(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::Delegate {
            root,
            data_hash,
            creator_hash,
            nonce: args.nonce,
            index: args.index,
        };

        let owner = clone_keypair(&args.owner);
        let need_proof = Some(args.index);
        let inner = DelegateInner {
            args,
            new_delegate: clone_keypair(new_leaf_delegate),
        };

        Ok(self.tx_builder(accounts, data, need_proof, inner, owner.pubkey(), &[&owner]))
    }

    // Does the prev delegate need to sign as well?
    pub async fn delegate(
        &mut self,
        args: &mut LeafArgs,
        new_delegate: &Keypair,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.delegate_tx(args, new_delegate)
            .await?
            .execute(&[], &[])
            .await
    }

    pub async fn redeem_tx<'a>(
        &'a mut self,
        args: &'a LeafArgs,
    ) -> Result<RedeemBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;
        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;

        let accounts = mpl_bubblegum::accounts::Redeem {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            leaf_delegate: args.delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            voucher: self.voucher(args.nonce),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::Redeem {
            root,
            data_hash,
            creator_hash,
            nonce: args.nonce,
            index: args.index,
        };

        Ok(self.tx_builder(
            accounts,
            data,
            Some(args.index),
            args,
            args.owner.pubkey(),
            &[&args.owner],
        ))
    }

    pub async fn redeem(&mut self, args: &LeafArgs) -> Result<BanksTransactionResultWithMetadata> {
        self.redeem_tx(args).await?.execute(&[], &[]).await
    }

    pub async fn cancel_redeem_tx<'a>(
        &'a mut self,
        args: &'a LeafArgs,
    ) -> Result<CancelRedeemBuilder<MAX_DEPTH, MAX_BUFFER_SIZE>> {
        let root = self.decode_root().await?;

        let accounts = mpl_bubblegum::accounts::CancelRedeem {
            tree_authority: self.authority(),
            leaf_owner: args.owner.pubkey(),
            merkle_tree: self.tree_pubkey(),
            voucher: self.voucher(args.nonce),
            log_wrapper: spl_noop::id(),
            compression_program: spl_account_compression::id(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::CancelRedeem { root };

        Ok(self.tx_builder(
            accounts,
            data,
            Some(args.index),
            args,
            args.owner.pubkey(),
            &[&args.owner],
        ))
    }

    pub async fn cancel_redeem(
        &mut self,
        args: &LeafArgs,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.cancel_redeem_tx(args).await?.execute(&[], &[]).await
    }

    pub fn set_tree_delegate_tx(
        &mut self,
        new_tree_delegate: &Keypair,
    ) -> SetTreeDelegateBuilder<MAX_DEPTH, MAX_BUFFER_SIZE> {
        let accounts = mpl_bubblegum::accounts::SetTreeDelegate {
            tree_creator: self.creator_pubkey(),
            new_tree_delegate: new_tree_delegate.pubkey(),
            merkle_tree: self.tree_pubkey(),
            tree_authority: self.authority(),
            system_program: system_program::id(),
        };

        let data = mpl_bubblegum::instruction::SetTreeDelegate;

        let tree_creator = Keypair::from_bytes(&self.tree_creator.to_bytes()).unwrap();
        self.tx_builder(
            accounts,
            data,
            None,
            clone_keypair(new_tree_delegate),
            self.creator_pubkey(),
            &[&tree_creator],
        )
    }

    pub async fn set_tree_delegate(
        &mut self,
        new_tree_delegate: &Keypair,
    ) -> Result<BanksTransactionResultWithMetadata> {
        self.set_tree_delegate_tx(new_tree_delegate)
            .execute(&[], &[])
            .await
    }

    // The following methods provide convenience when reading data from accounts.
    pub async fn read_account(&mut self, key: Pubkey) -> Result<Account> {
        self.client
            .get_account(key)
            .await
            .map_err(Error::BanksClient)?
            .ok_or_else(|| Box::new(Error::AccountNotFound(key)))
    }

    // This reads the `Account`, but also deserializes the data to return
    // the strongly typed inner contents.
    pub async fn read_account_data<T>(&mut self, key: Pubkey) -> Result<T>
    where
        T: AccountDeserialize,
    {
        self.read_account(key).await.and_then(|acc| {
            T::try_deserialize(&mut acc.data.as_slice()).map_err(|err| Box::new(Error::Anchor(err)))
        })
    }

    pub async fn read_tree_config(&mut self) -> Result<TreeConfig> {
        self.read_account_data(self.authority()).await
    }

    pub async fn read_voucher(&mut self, nonce: u64) -> Result<Voucher> {
        self.read_account_data(self.voucher(nonce)).await
    }

    pub fn leaf_node(&self, args: &LeafArgs) -> Result<Node> {
        let (data_hash, creator_hash) = compute_metadata_hashes(&args.metadata)?;
        let asset_id = get_asset_id(&self.tree_pubkey(), args.nonce);

        let leaf = LeafSchema::new_v0(
            asset_id,
            args.owner.pubkey(),
            args.delegate.pubkey(),
            args.nonce,
            data_hash,
            creator_hash,
        );

        Ok(leaf.to_node())
    }

    pub fn num_minted(&self) -> u64 {
        self.num_minted
    }

    pub fn inc_num_minted(&mut self) {
        self.num_minted += 1;
    }

    // Return a `LeafSchema` object for the given arguments.
    pub fn leaf_schema(&self, leaf: &LeafArgs) -> LeafSchema {
        let id = get_asset_id(&self.tree_pubkey(), leaf.nonce);
        let (data_hash, creator_hash) = compute_metadata_hashes(&leaf.metadata).unwrap();
        LeafSchema::new_v0(
            id,
            leaf.owner.pubkey(),
            leaf.delegate.pubkey(),
            leaf.nonce,
            data_hash,
            creator_hash,
        )
    }

    // Return a `Voucher` object with the field values we expect for the
    // given leaf arguments.
    pub fn expected_voucher(&self, leaf: &LeafArgs) -> Voucher {
        Voucher::new(self.leaf_schema(leaf), leaf.index, self.tree_pubkey())
    }

    // Return the expected value of the on-chain merkle tree root, based on the locally
    // computed proof generated by `self.proof_tree`.
    pub fn expected_root(&self) -> [u8; 32] {
        self.proof_tree.get_root()
    }

    pub async fn check_expected_root(&mut self) -> Result<()> {
        let root = self.decode_root().await?;

        if root != self.expected_root() {
            return Err(Box::new(Error::RootMismatch));
        }

        Ok(())
    }

    // Updates the inner `MerkleTree` when the given leaf has changed.
    pub fn update_leaf(&mut self, args: &LeafArgs) -> Result<()> {
        let node = self.leaf_node(args)?;
        self.proof_tree
            // The conversion below should never fail.
            .add_leaf(node, usize::try_from(args.index).unwrap());
        Ok(())
    }

    // Updates the inner `MerkleTree` with the fact that we zeroed the leaf present
    // at the given index.
    pub fn zero_leaf(&mut self, index: u32) -> Result<()> {
        let node = [0u8; 32];
        // The conversion below should never fail.
        self.proof_tree
            .add_leaf(node, usize::try_from(index).unwrap());
        Ok(())
    }

    // Using `u32` for the idx because that's the datatype chosen for the index upstream
    // for some reason.
    pub fn proof_of_leaf(&self, index: u32) -> Vec<Node> {
        // The conversion below should not fail.
        self.proof_tree
            .get_proof_of_leaf(usize::try_from(index).unwrap())
    }

    // Useful when adding proofs as additional accounts to an instruction.
    pub fn proof_of_leaf_metas(&self, index: u32) -> Vec<AccountMeta> {
        let nodes = self.proof_of_leaf(index);
        nodes
            .into_iter()
            .map(|node| AccountMeta::new_readonly(Pubkey::new_from_array(node), false))
            .collect()
    }
}
