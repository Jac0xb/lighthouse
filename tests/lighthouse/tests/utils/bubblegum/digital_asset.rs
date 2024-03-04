#![allow(clippy::too_many_arguments)]

use mpl_token_metadata::{
    accounts::{EditionMarker, MasterEdition, Metadata, MetadataDelegateRecord, TokenRecord},
    instructions::{
        BurnBuilder, CreateBuilder, DelegateBuilder, LockBuilder, MintBuilder,
        MintNewEditionFromMasterEditionViaToken,
        MintNewEditionFromMasterEditionViaTokenInstructionArgs, RevokeBuilder, TransferBuilder,
        UnlockBuilder, UnverifyBuilder, UpdateBuilder, VerifyBuilder,
    },
    types::{
        AuthorizationData, BurnArgs, Collection, CollectionDetails, CreateArgs, Creator,
        DelegateArgs, LockArgs, MetadataDelegateRole, MintArgs,
        MintNewEditionFromMasterEditionViaTokenArgs, PrintSupply, ProgrammableConfig, RevokeArgs,
        TokenDelegateRole, TokenStandard, TransferArgs, UnlockArgs, UpdateArgs, VerificationArgs,
    },
    EDITION_MARKER_BIT_SIZE,
};
use solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey, system_program};
use solana_program_test::{BanksClientError, ProgramTestContext};
use solana_sdk::{
    account::AccountSharedData,
    compute_budget::ComputeBudgetInstruction,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use super::DirtyClone;

pub const DEFAULT_NAME: &str = "Digital Asset";
pub const DEFAULT_SYMBOL: &str = "DA";
pub const DEFAULT_URI: &str = "https://digital.asset.org";

// This represents a generic Metaplex Digital asset of various Token Standards.
// It is used to abstract away the various accounts that are created for a given
// Digital Asset. Since different asset types have different accounts, care
// should be taken that appropriate handlers update appropriate accounts, such as when
// transferring a DigitalAsset, the token account should be updated.
#[derive(Debug)]
pub struct DigitalAsset {
    pub metadata: Pubkey,
    pub mint: Keypair,
    pub token: Option<Pubkey>,
    pub edition: Option<Pubkey>,
    pub token_record: Option<Pubkey>,
    pub token_standard: Option<TokenStandard>,
    pub edition_num: Option<u64>,
}

impl DirtyClone for DigitalAsset {
    fn dirty_clone(&self) -> Self {
        Self {
            metadata: self.metadata,
            mint: self.mint.dirty_clone(),
            token: self.token,
            edition: self.edition,
            token_record: self.token_record,
            token_standard: self.token_standard.clone(),
            edition_num: self.edition_num,
        }
    }
}

impl Default for DigitalAsset {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalAsset {
    pub fn new() -> Self {
        let mint = Keypair::new();
        let mint_pubkey = mint.pubkey();

        let (metadata, _) = Metadata::find_pda(&mint_pubkey);

        Self {
            metadata,
            mint,
            token: None,
            edition: None,
            token_record: None,
            token_standard: None,
            edition_num: None,
        }
    }

    pub fn set_edition(&mut self) {
        let edition = MasterEdition::find_pda(&self.mint.pubkey()).0;
        self.edition = Some(edition);
    }

    pub async fn burn(
        &mut self,
        context: &mut ProgramTestContext,
        authority: Keypair,
        args: BurnArgs,
        parent_asset: Option<DigitalAsset>,
        collection_metadata: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        let md = self.get_metadata(context).await;
        let token_standard = md.token_standard.unwrap();

        let mut builder = BurnBuilder::new();
        builder
            .authority(authority.pubkey())
            .metadata(self.metadata)
            .mint(self.mint.pubkey())
            .token(self.token.unwrap())
            .burn_args(args);

        if let Some(parent_asset) = parent_asset {
            builder.master_edition_mint(Some(parent_asset.mint.pubkey()));
            builder.master_edition_token(Some(parent_asset.token.unwrap()));
            builder.master_edition(Some(parent_asset.edition.unwrap()));

            let edition_num = self.edition_num.unwrap();

            let marker_num = edition_num.checked_div(EDITION_MARKER_BIT_SIZE).unwrap();

            let edition_marker =
                EditionMarker::find_pda(&parent_asset.mint.pubkey(), &marker_num.to_string()).0;
            builder.edition_marker(Some(edition_marker));
        }

        if let Some(edition) = self.edition {
            println!("edition: {:?}", edition);
            builder.edition(Some(edition));
        }

        if token_standard == TokenStandard::ProgrammableNonFungible {
            builder.token_record(Some(self.token_record.unwrap()));
        }

        if let Some(collection_metadata) = collection_metadata {
            builder.collection_metadata(Some(collection_metadata));
        }

        let burn_ix = builder.instruction();

        let transaction = Transaction::new_signed_with_payer(
            &[burn_ix],
            Some(&authority.pubkey()),
            &[&authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    // Note the authority is the payer of the transaction.
    pub async fn verify(
        &mut self,
        context: &mut ProgramTestContext,
        authority: Keypair,
        args: VerificationArgs,
        metadata: Option<Pubkey>,
        delegate_record: Option<Pubkey>,
        collection_mint: Option<Pubkey>,
        collection_metadata: Option<Pubkey>,
        collection_master_edition: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        let mut builder = VerifyBuilder::new();
        builder
            .authority(authority.pubkey())
            .metadata(metadata.unwrap_or(self.metadata));

        match args {
            VerificationArgs::CreatorV1 => (),
            VerificationArgs::CollectionV1 => {
                if let Some(delegate_record) = delegate_record {
                    builder.delegate_record(Some(delegate_record));
                }

                if let Some(collection_mint) = collection_mint {
                    builder.collection_mint(Some(collection_mint));
                }

                if let Some(collection_metadata) = collection_metadata {
                    builder.collection_metadata(Some(collection_metadata));
                }

                if let Some(collection_master_edition) = collection_master_edition {
                    builder.collection_master_edition(Some(collection_master_edition));
                }
            }
        }

        let verify_ix = builder.verification_args(args).instruction();

        let transaction = Transaction::new_signed_with_payer(
            &[verify_ix],
            Some(&authority.pubkey()),
            &[&authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    // Note the authority is the payer of the transaction.
    pub async fn unverify(
        &mut self,
        context: &mut ProgramTestContext,
        authority: Keypair,
        args: VerificationArgs,
        metadata: Option<Pubkey>,
        delegate_record: Option<Pubkey>,
        collection_mint: Option<Pubkey>,
        collection_metadata: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        let mut builder = UnverifyBuilder::new();
        builder
            .authority(authority.pubkey())
            .metadata(metadata.unwrap_or(self.metadata));

        match args {
            VerificationArgs::CreatorV1 => (),
            VerificationArgs::CollectionV1 => {
                if let Some(delegate_record) = delegate_record {
                    builder.delegate_record(Some(delegate_record));
                }

                if let Some(collection_mint) = collection_mint {
                    builder.collection_mint(Some(collection_mint));
                }

                if let Some(collection_metadata) = collection_metadata {
                    builder.collection_metadata(Some(collection_metadata));
                }
            }
        }

        let unverify_ix = builder.verification_args(args).instruction();

        let transaction = Transaction::new_signed_with_payer(
            &[unverify_ix],
            Some(&authority.pubkey()),
            &[&authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(transaction).await
    }

    pub async fn create(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        let creators = Some(vec![Creator {
            address: context.payer.pubkey(),
            share: 100,
            verified: true,
        }]);

        self.create_advanced(
            context,
            token_standard,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            500,
            creators,
            None,
            None,
            authorization_rules,
            PrintSupply::Zero,
        )
        .await
    }

    pub async fn create_advanced(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        creators: Option<Vec<Creator>>,
        collection: Option<Collection>,
        collection_details: Option<CollectionDetails>,
        authorization_rules: Option<Pubkey>,
        print_supply: PrintSupply,
    ) -> Result<(), BanksClientError> {
        let payer_pubkey = context.payer.pubkey();
        let mint_pubkey = self.mint.pubkey();

        let mut builder = CreateBuilder::default();
        builder
            .metadata(self.metadata)
            .mint(self.mint.pubkey(), true)
            .authority(payer_pubkey)
            .payer(payer_pubkey)
            .update_authority(payer_pubkey, true);

        let edition = match &token_standard {
            TokenStandard::NonFungible | TokenStandard::ProgrammableNonFungible => {
                let (edition, _) = MasterEdition::find_pda(&mint_pubkey);
                // sets the master edition to the builder
                builder.master_edition(Some(edition));
                Some(edition)
            }
            _ => None,
        };
        // builds the instruction
        let create_ix = builder
            .create_args(CreateArgs::V1 {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
                collection,
                uses: None,
                decimals: Some(0),
                print_supply: Some(print_supply),
                collection_details,
                is_mutable: true,
                primary_sale_happened: false,
                rule_set: authorization_rules,
                token_standard: token_standard.clone(),
            })
            .instruction();

        let compute_ix = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

        let tx = Transaction::new_signed_with_payer(
            &[compute_ix, create_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &self.mint],
            context.last_blockhash,
        );

        self.edition = edition;
        self.token_standard = Some(token_standard);

        context.banks_client.process_transaction(tx).await
    }

    pub async fn mint(
        &mut self,
        context: &mut ProgramTestContext,
        authorization_rules: Option<Pubkey>,
        authorization_data: Option<AuthorizationData>,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        let payer_pubkey = context.payer.pubkey();
        let (token, _) = Pubkey::find_program_address(
            &[
                &payer_pubkey.to_bytes(),
                &spl_token::id().to_bytes(),
                &self.mint.pubkey().to_bytes(),
            ],
            &spl_associated_token_account::id(),
        );

        let (token_record, _) = TokenRecord::find_pda(&self.mint.pubkey(), &token);

        let token_record_opt = if self.is_pnft(context).await {
            Some(token_record)
        } else {
            None
        };

        let mint_ix = MintBuilder::new()
            .token(token)
            .token_record(Some(token_record))
            .token_owner(Some(payer_pubkey))
            .metadata(self.metadata)
            .mint(self.mint.pubkey())
            .payer(payer_pubkey)
            .authority(payer_pubkey)
            .master_edition(self.edition)
            .authorization_rules(authorization_rules)
            .mint_args(MintArgs::V1 {
                amount,
                authorization_data,
            })
            .instruction();

        let compute_ix = ComputeBudgetInstruction::set_compute_unit_limit(800_000);

        let tx = Transaction::new_signed_with_payer(
            &[compute_ix, mint_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.map(|_| {
            self.token = Some(token);
            self.token_record = token_record_opt;
        })
    }

    pub async fn create_and_mint(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
        authorization_data: Option<AuthorizationData>,
        amount: u64,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create(context, token_standard, authorization_rules)
            .await
            .unwrap();
        // mints tokens
        self.mint(context, authorization_rules, authorization_data, amount)
            .await
    }

    pub async fn create_and_mint_with_creators(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
        authorization_data: Option<AuthorizationData>,
        amount: u64,
        creators: Option<Vec<Creator>>,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create_advanced(
            context,
            token_standard,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            500,
            creators,
            None,
            None,
            authorization_rules,
            PrintSupply::Zero,
        )
        .await
        .unwrap();

        // mints tokens
        self.mint(context, authorization_rules, authorization_data, amount)
            .await
    }

    pub async fn create_and_mint_item_with_collection(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
        authorization_data: Option<AuthorizationData>,
        amount: u64,
        collection: Option<Collection>,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create_advanced(
            context,
            token_standard,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            500,
            None,
            collection,
            None,
            authorization_rules,
            PrintSupply::Zero,
        )
        .await
        .unwrap();

        // mints tokens
        self.mint(context, authorization_rules, authorization_data, amount)
            .await
    }

    pub async fn create_and_mint_collection_parent(
        &mut self,
        context: &mut ProgramTestContext,
        token_standard: TokenStandard,
        authorization_rules: Option<Pubkey>,
        authorization_data: Option<AuthorizationData>,
        amount: u64,
        collection_details: Option<CollectionDetails>,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create_advanced(
            context,
            token_standard,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            500,
            None,
            None,
            collection_details,
            authorization_rules,
            PrintSupply::Zero,
        )
        .await
        .unwrap();

        // mints tokens
        self.mint(context, authorization_rules, authorization_data, amount)
            .await
    }

    pub async fn create_and_mint_nonfungible(
        &mut self,
        context: &mut ProgramTestContext,
        print_supply: PrintSupply,
    ) -> Result<(), BanksClientError> {
        // creates the metadata
        self.create_advanced(
            context,
            TokenStandard::NonFungible,
            String::from(DEFAULT_NAME),
            String::from(DEFAULT_SYMBOL),
            String::from(DEFAULT_URI),
            500,
            None,
            None,
            None,
            None,
            print_supply,
        )
        .await
        .unwrap();

        // mints tokens
        self.mint(context, None, None, 1).await
    }

    pub async fn delegate(
        &mut self,
        context: &mut ProgramTestContext,
        payer: Keypair,
        delegate: Pubkey,
        args: DelegateArgs,
    ) -> Result<Option<Pubkey>, BanksClientError> {
        let mut builder = DelegateBuilder::new();
        builder
            .delegate(delegate)
            .mint(self.mint.pubkey())
            .metadata(self.metadata)
            .payer(payer.pubkey())
            .authority(payer.pubkey())
            .master_edition(self.edition)
            .token(self.token)
            .spl_token_program(Some(spl_token::ID));

        let mut delegate_or_token_record = None;

        match args {
            // Token delegates.
            DelegateArgs::SaleV1 { .. }
            | DelegateArgs::TransferV1 { .. }
            | DelegateArgs::UtilityV1 { .. }
            | DelegateArgs::StakingV1 { .. }
            | DelegateArgs::LockedTransferV1 { .. } => {
                let (token_record, _) =
                    TokenRecord::find_pda(&self.mint.pubkey(), &self.token.unwrap());
                builder.token_record(Some(token_record));
                delegate_or_token_record = Some(token_record);
            }
            DelegateArgs::StandardV1 { .. } => { /* nothing to add */ }

            // Metadata delegates.
            DelegateArgs::CollectionV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::Collection,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::DataV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::Data,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::ProgrammableConfigV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::ProgrammableConfig,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::AuthorityItemV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::AuthorityItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::DataItemV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::DataItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::CollectionItemV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::CollectionItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
            DelegateArgs::ProgrammableConfigItemV1 { .. } => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::ProgrammableConfigItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
                delegate_or_token_record = Some(delegate_record);
            }
        }

        // determines if we need to set the rule set
        let metadata_account = get_account(context, &self.metadata).await;
        let metadata: Metadata = Metadata::safe_deserialize(&metadata_account.data).unwrap();

        if let Some(ProgrammableConfig::V1 {
            rule_set: Some(rule_set),
        }) = metadata.programmable_config
        {
            builder.authorization_rules(Some(rule_set));
            // builder.authorization_rules_program(Some(mpl_token_auth_rules::ID));
        }

        let compute_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);

        let delegate_ix = builder.delegate_args(args.clone()).instruction();

        let tx = Transaction::new_signed_with_payer(
            &[compute_ix, delegate_ix],
            Some(&payer.pubkey()),
            &[&payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await?;
        Ok(delegate_or_token_record)
    }

    pub async fn print_edition(
        &self,
        context: &mut ProgramTestContext,
        edition_num: u64,
    ) -> Result<DigitalAsset, BanksClientError> {
        let print_mint = Keypair::new();
        let print_token = Keypair::new();
        let (print_metadata, _) = Metadata::find_pda(&print_mint.pubkey());
        let (print_edition, _) = MasterEdition::find_pda(&print_mint.pubkey());

        create_mint(
            context,
            &print_mint,
            &context.payer.pubkey(),
            Some(&context.payer.pubkey()),
            0,
        )
        .await?;
        create_token_account(
            context,
            &print_token,
            &print_mint.pubkey(),
            &context.payer.pubkey(),
        )
        .await?;
        mint_tokens(
            context,
            &print_mint.pubkey(),
            &print_token.pubkey(),
            1,
            &context.payer.pubkey(),
            None,
        )
        .await?;

        let ix = MintNewEditionFromMasterEditionViaToken {
            new_metadata: print_metadata,
            new_edition: print_edition,
            master_edition: self.edition.unwrap(),
            new_mint: print_mint.pubkey(),
            new_mint_authority: context.payer.pubkey(),
            payer: context.payer.pubkey(),
            token_account_owner: context.payer.pubkey(),
            token_account: self.token.unwrap(),
            new_metadata_update_authority: context.payer.pubkey(),
            metadata: self.metadata,
            edition_mark_pda: EditionMarker::find_pda(
                &self.mint.pubkey(),
                &edition_num.to_string(),
            )
            .0,
            system_program: system_program::ID,
            token_program: spl_token::ID,
            rent: None,
        }
        .instruction(MintNewEditionFromMasterEditionViaTokenInstructionArgs {
            mint_new_edition_from_master_edition_via_token_args:
                MintNewEditionFromMasterEditionViaTokenArgs {
                    edition: edition_num,
                },
        });

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &context.payer],
            context.last_blockhash,
        );

        context
            .banks_client
            .process_transaction_with_commitment(
                tx,
                solana_sdk::commitment_config::CommitmentLevel::Confirmed,
            )
            .await
            .unwrap();

        Ok(DigitalAsset {
            mint: print_mint,
            token: Some(print_token.pubkey()),
            metadata: print_metadata,
            edition: Some(print_edition),
            token_standard: self.token_standard.clone(),
            token_record: None,
            edition_num: Some(edition_num),
        })
    }

    pub async fn revoke(
        &mut self,
        context: &mut ProgramTestContext,
        payer: Keypair,
        approver: Keypair,
        delegate: Pubkey,
        args: RevokeArgs,
    ) -> Result<(), BanksClientError> {
        let mut builder = RevokeBuilder::new();
        builder
            .delegate(delegate)
            .mint(self.mint.pubkey())
            .metadata(self.metadata)
            .payer(approver.pubkey())
            .authority(approver.pubkey())
            .master_edition(self.edition)
            .token(self.token)
            .spl_token_program(Some(spl_token::ID));

        match args {
            // Token delegates.
            RevokeArgs::SaleV1
            | RevokeArgs::TransferV1
            | RevokeArgs::UtilityV1
            | RevokeArgs::StakingV1
            | RevokeArgs::LockedTransferV1
            | RevokeArgs::MigrationV1 => {
                let (token_record, _) =
                    TokenRecord::find_pda(&self.mint.pubkey(), &self.token.unwrap());
                builder.token_record(Some(token_record));
            }
            RevokeArgs::StandardV1 { .. } => { /* nothing to add */ }

            // Metadata delegates.
            RevokeArgs::CollectionV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::Collection,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
            RevokeArgs::DataV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::Data,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
            RevokeArgs::ProgrammableConfigV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::ProgrammableConfig,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
            RevokeArgs::AuthorityItemV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::AuthorityItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
            RevokeArgs::DataItemV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::DataItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
            RevokeArgs::CollectionItemV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::CollectionItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }

            RevokeArgs::ProgrammableConfigItemV1 => {
                let (delegate_record, _) = MetadataDelegateRecord::find_pda(
                    &self.mint.pubkey(),
                    MetadataDelegateRole::ProgrammableConfigItem,
                    &payer.pubkey(),
                    &delegate,
                );
                builder.delegate_record(Some(delegate_record));
            }
        }

        let revoke_ix = builder.revoke_args(args.clone()).instruction();

        let tx = Transaction::new_signed_with_payer(
            &[revoke_ix],
            Some(&payer.pubkey()),
            &[&approver, &payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    // This transfers a DigitalAsset from its existing Token Account to a new one
    // and should update the token account after a successful transfer, as well as the
    // token record if appropriate (for pNFTs).
    pub async fn transfer(&mut self, params: TransferParams<'_>) -> Result<(), BanksClientError> {
        let TransferParams {
            context,
            authority,
            source_owner,
            destination_owner,
            destination_token,
            authorization_rules,
            payer,
            args,
        } = params;

        let compute_ix = ComputeBudgetInstruction::set_compute_unit_limit(800_000);
        let mut instructions = vec![compute_ix];

        let destination_token = if let Some(destination_token) = destination_token {
            destination_token
        } else {
            instructions.push(create_associated_token_account(
                &authority.pubkey(),
                &destination_owner,
                &self.mint.pubkey(),
                &spl_token::id(),
            ));

            get_associated_token_address(&destination_owner, &self.mint.pubkey())
        };

        let mut builder = TransferBuilder::new();
        builder
            .authority(authority.pubkey())
            .token_owner(*source_owner)
            .token(self.token.unwrap())
            .destination_owner(destination_owner)
            .destination_token(destination_token)
            .metadata(self.metadata)
            .edition(self.edition)
            .payer(payer.pubkey())
            .mint(self.mint.pubkey())
            .token_record(self.token_record);

        // This can be optional for non pNFTs but always include it for now.
        let (destination_token_record, _bump) =
            TokenRecord::find_pda(&self.mint.pubkey(), &destination_token);
        let destination_token_record_opt = if self.is_pnft(context).await {
            builder.destination_token_record(Some(destination_token_record));
            Some(destination_token_record)
        } else {
            None
        };

        if let Some(authorization_rules) = authorization_rules {
            builder.authorization_rules(Some(authorization_rules));
            // builder.authorization_rules_program(Some(mpl_token_auth_rules::ID));
        }

        let transfer_ix = builder.transfer_args(args).instruction();

        instructions.push(transfer_ix);

        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&authority.pubkey()),
            &[authority, payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.map(|_| {
            // Update token values for new owner.
            self.token = Some(destination_token);
            self.token_record = destination_token_record_opt;
        })
    }

    pub async fn lock(
        &mut self,
        context: &mut ProgramTestContext,
        delegate: Keypair,
        token_record: Option<Pubkey>,
        payer: Keypair,
    ) -> Result<(), BanksClientError> {
        let mut builder = LockBuilder::new();
        builder
            .authority(delegate.pubkey())
            .mint(self.mint.pubkey())
            .metadata(self.metadata)
            .payer(payer.pubkey())
            .token_record(token_record)
            .edition(self.edition)
            .spl_token_program(Some(spl_token::ID));

        if let Some(token) = self.token {
            builder.token(token);
        }

        let utility_ix = builder
            .lock_args(LockArgs::V1 {
                authorization_data: None,
            })
            .instruction();

        let tx = Transaction::new_signed_with_payer(
            &[utility_ix],
            Some(&payer.pubkey()),
            &[&delegate, &payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn unlock(
        &mut self,
        context: &mut ProgramTestContext,
        delegate: Keypair,
        token_record: Option<Pubkey>,
        payer: Keypair,
    ) -> Result<(), BanksClientError> {
        let mut builder = UnlockBuilder::new();
        builder
            .authority(delegate.pubkey())
            .mint(self.mint.pubkey())
            .metadata(self.metadata)
            .payer(payer.pubkey())
            .token_record(token_record)
            .edition(self.edition)
            .spl_token_program(Some(spl_token::ID));

        if let Some(token) = self.token {
            builder.token(token);
        }

        let unlock_ix = builder
            .unlock_args(UnlockArgs::V1 {
                authorization_data: None,
            })
            .instruction();

        let tx = Transaction::new_signed_with_payer(
            &[unlock_ix],
            Some(&payer.pubkey()),
            &[&delegate, &payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn update(
        &self,
        context: &mut ProgramTestContext,
        authority: Keypair,
        update_args: UpdateArgs,
    ) -> Result<(), BanksClientError> {
        let mut builder = UpdateBuilder::new();
        builder
            .authority(authority.pubkey())
            .metadata(self.metadata)
            .edition(self.edition)
            .payer(authority.pubkey())
            .mint(self.mint.pubkey());

        let update_ix = builder.update_args(update_args).instruction();

        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&authority.pubkey()),
            &[&authority],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }

    pub async fn get_metadata(&self, context: &mut ProgramTestContext) -> Metadata {
        let metadata_account = context
            .banks_client
            .get_account(self.metadata)
            .await
            .unwrap()
            .unwrap();

        Metadata::safe_deserialize(&metadata_account.data).unwrap()
    }

    pub async fn get_token_delegate_role(
        &self,
        context: &mut ProgramTestContext,
        token: &Pubkey,
    ) -> Option<TokenDelegateRole> {
        let (delegate_record_pubkey, _) = TokenRecord::find_pda(&self.mint.pubkey(), token);
        let delegate_record_account = context
            .banks_client
            .get_account(delegate_record_pubkey)
            .await
            .unwrap();

        if let Some(account) = delegate_record_account {
            let delegate_record = TokenRecord::safe_deserialize(&account.data).unwrap();
            delegate_record.delegate_role
        } else {
            None
        }
    }

    pub async fn is_pnft(&self, context: &mut ProgramTestContext) -> bool {
        let md = self.get_metadata(context).await;
        if let Some(standard) = md.token_standard {
            if standard == TokenStandard::ProgrammableNonFungible {
                return true;
            }
        }

        false
    }

    pub async fn inject_close_authority(
        &self,
        context: &mut ProgramTestContext,
        close_authority: &Pubkey,
    ) {
        // To simulate the state where the close authority is set delegate instead of
        // the asset's master edition account, we need to inject modified token account state.
        let mut token_account = get_account(context, &self.token.unwrap()).await;
        let mut token = spl_token::state::Account::unpack(&token_account.data).unwrap();

        token.close_authority = COption::Some(*close_authority);
        let mut data = vec![0u8; spl_token::state::Account::LEN];
        spl_token::state::Account::pack(token, &mut data).unwrap();
        token_account.data = data;

        let token_account_shared_data: AccountSharedData = token_account.into();
        context.set_account(&self.token.unwrap(), &token_account_shared_data);
    }

    pub async fn assert_creators_matches_on_chain(
        &self,
        context: &mut ProgramTestContext,
        creators: &Option<Vec<Creator>>,
    ) {
        let metadata = self.get_metadata(context).await;
        let on_chain_creators = metadata.creators;
        assert_eq!(on_chain_creators, *creators);
    }

    pub async fn assert_item_collection_matches_on_chain(
        &self,
        context: &mut ProgramTestContext,
        collection: &Option<Collection>,
    ) {
        let metadata = self.get_metadata(context).await;
        let on_chain_collection = metadata.collection;
        assert_eq!(on_chain_collection, *collection);
    }

    pub async fn assert_collection_details_matches_on_chain(
        &self,
        context: &mut ProgramTestContext,
        collection_details: &Option<CollectionDetails>,
    ) {
        let metadata = self.get_metadata(context).await;
        let on_chain_collection_details = metadata.collection_details;
        assert_eq!(on_chain_collection_details, *collection_details);
    }

    pub async fn assert_burned(
        &self,
        context: &mut ProgramTestContext,
    ) -> Result<(), BanksClientError> {
        match self.token_standard.clone().unwrap() {
            TokenStandard::NonFungible => {
                self.non_fungigble_accounts_closed(context).await?;
            }
            TokenStandard::ProgrammableNonFungible => {
                self.programmable_non_fungigble_accounts_closed(context)
                    .await?;
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    async fn non_fungigble_accounts_closed(
        &self,
        context: &mut ProgramTestContext,
    ) -> Result<(), BanksClientError> {
        // Metadata, Master Edition and token account are burned.
        let md_account = context.banks_client.get_account(self.metadata).await?;
        let token_account = context
            .banks_client
            .get_account(self.token.unwrap())
            .await?;
        let edition_account = context
            .banks_client
            .get_account(self.edition.unwrap())
            .await?;

        assert!(md_account.is_none());
        assert!(token_account.is_none());
        assert!(edition_account.is_none());

        Ok(())
    }

    async fn programmable_non_fungigble_accounts_closed(
        &self,
        context: &mut ProgramTestContext,
    ) -> Result<(), BanksClientError> {
        self.non_fungigble_accounts_closed(context).await?;

        // Token record is burned.
        let token_record_account = context
            .banks_client
            .get_account(self.token_record.unwrap())
            .await?;

        assert!(token_record_account.is_none());

        Ok(())
    }

    pub async fn assert_token_record_closed(
        &self,
        context: &mut ProgramTestContext,
        token: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let (token_record_pubkey, _) = TokenRecord::find_pda(&self.mint.pubkey(), token);

        let token_record_account = context
            .banks_client
            .get_account(token_record_pubkey)
            .await?;

        assert!(token_record_account.is_none());

        Ok(())
    }
}

pub struct TransferParams<'a> {
    pub context: &'a mut ProgramTestContext,
    pub authority: &'a Keypair,
    pub source_owner: &'a Pubkey,
    pub destination_owner: Pubkey,
    pub destination_token: Option<Pubkey>,
    pub payer: &'a Keypair,
    pub authorization_rules: Option<Pubkey>,
    pub args: TransferArgs,
}

pub async fn mint_tokens(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
    owner: &Pubkey,
    additional_signer: Option<&Keypair>,
) -> Result<(), BanksClientError> {
    let mut signing_keypairs = vec![&context.payer];
    if let Some(signer) = additional_signer {
        signing_keypairs.push(signer);
    }

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_token::instruction::mint_to(&spl_token::id(), mint, account, owner, &[], amount)
                .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &signing_keypairs,
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn create_token_account(
    context: &mut ProgramTestContext,
    account: &Keypair,
    mint: &Pubkey,
    manager: &Pubkey,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &account.pubkey(),
                rent.minimum_balance(spl_token::state::Account::LEN),
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account.pubkey(),
                mint,
                manager,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, account],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    mint: &Keypair,
    manager: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                manager,
                freeze_authority,
                decimals,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn get_account(
    context: &mut ProgramTestContext,
    pubkey: &Pubkey,
) -> solana_sdk::account::Account {
    context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}
