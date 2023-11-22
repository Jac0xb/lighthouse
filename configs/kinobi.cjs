const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([
  path.join(idlDir, "bubblegum.json"),
  path.join(idlDir, "spl_account_compression.json"),
  path.join(idlDir, "spl_noop.json"),
]);

// Update programs.
kinobi.update(
  new k.UpdateProgramsVisitor({
    bubblegum: { name: "mplBubblegum" },
  })
);

// Update accounts.
kinobi.update(
  new k.UpdateAccountsVisitor({
    treeConfig: {
      seeds: [k.publicKeySeed("merkleTree")],
      size: 96,
    },
    voucher: {
      seeds: [
        k.stringConstantSeed("voucher"),
        k.publicKeySeed("merkleTree"),
        k.variableSeed("nonce", k.numberTypeNode("u64")),
      ],
    },
  })
);

// Update types.
kinobi.update(
  new k.UpdateDefinedTypesVisitor({
    // Remove unnecessary types.
    InstructionName: { delete: true },
    // Remove unnecessary spl_account_compression type.
    ApplicationDataEventV1: { delete: true },
    ChangeLogEventV1: { delete: true },
    PathNode: { delete: true },
    ApplicationDataEvent: { delete: true },
    ChangeLogEvent: { delete: true },
    AccountCompressionEvent: { delete: true },
  })
);

// Custom tree updates.
kinobi.update(
  new k.TransformNodesVisitor([
    {
      // Rename `treeAuthority` instruction account to `treeConfig`.
      selector: { kind: "instructionAccountNode", name: "treeAuthority" },
      transformer: (node) => {
        k.assertInstructionAccountNode(node);
        return k.instructionAccountNode({ ...node, name: "treeConfig" });
      },
    },
    {
      // Rename `treeDelegate` instruction account to `treeCreatorOrDelegate`.
      selector: { kind: "instructionAccountNode", name: "treeDelegate" },
      transformer: (node) => {
        k.assertInstructionAccountNode(node);
        return k.instructionAccountNode({
          ...node,
          name: "treeCreatorOrDelegate",
        });
      },
    },
    {
      // Rename `editionAccount` instruction account to `collectionEdition`.
      selector: { kind: "instructionAccountNode", name: "editionAccount" },
      transformer: (node) => {
        k.assertInstructionAccountNode(node);
        return k.instructionAccountNode({ ...node, name: "collectionEdition" });
      },
    },
    {
      // Rename `message` arg to `metadata`.
      selector: { kind: "structFieldTypeNode", name: "message" },
      transformer: (node) => {
        k.assertStructFieldTypeNode(node);
        return k.structFieldTypeNode({ ...node, name: "metadata" });
      },
    },
    {
      // Update `collectionAuthorityRecordPda` account as `optional`.
      selector: {
        kind: "instructionAccountNode",
        name: "collectionAuthorityRecordPda",
      },
      transformer: (node) => {
        k.assertInstructionAccountNode(node);
        return k.instructionAccountNode({
          ...node,
          isOptional: true,
        });
      },
    },
  ])
);

// Set default account values accross multiple instructions.
kinobi.update(
  new k.SetInstructionAccountDefaultValuesVisitor([
    {
      account: "associatedTokenProgram",
      ignoreIfOptional: true,
      ...k.programDefault(
        "splAssociatedToken",
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
      ),
    },
    {
      account: "logWrapper",
      ignoreIfOptional: true,
      ...k.programDefault(
        "splNoop",
        "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV"
      ),
    },
    {
      account: "compressionProgram",
      ignoreIfOptional: true,
      ...k.programDefault(
        "splAccountCompression",
        "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK"
      ),
    },
    {
      account: "treeCreator",
      ignoreIfOptional: true,
      ...k.identityDefault(),
    },
    {
      account: "treeCreatorOrDelegate",
      ignoreIfOptional: true,
      ...k.identityDefault(),
    },
    {
      account: "leafDelegate",
      ignoreIfOptional: true,
      ...k.accountDefault("leafOwner"),
    },
    {
      account: "treeConfig",
      ignoreIfOptional: true,
      ...k.pdaDefault("treeConfig"),
    },
    {
      account: "bubblegumSigner",
      ignoreIfOptional: true,
      ...k.publicKeyDefault("4ewWZC5gT6TGpm5LZNDs9wVonfUT2q5PP5sc9kVbwMAK"),
    },
    {
      account: "collectionMetadata",
      ignoreIfOptional: true,
      ...k.pdaDefault("metadata", {
        importFrom: "mplTokenMetadata",
        seeds: { mint: k.accountDefault("collectionMint") },
      }),
    },
    {
      account: "collectionEdition",
      ignoreIfOptional: true,
      ...k.pdaDefault("masterEdition", {
        importFrom: "mplTokenMetadata",
        seeds: { mint: k.accountDefault("collectionMint") },
      }),
    },
    {
      account: "collectionAuthorityRecordPda",
      ignoreIfOptional: true,
      ...k.programIdDefault(),
    },
    {
      account: "collectionAuthority",
      ignoreIfOptional: true,
      ...k.identityDefault(),
    },
  ])
);

// Update instructions.
const hashDefaults = {
  dataHash: {
    defaultsTo: k.resolverDefault("resolveDataHash", [
      k.dependsOnArg("metadata"),
    ]),
  },
  creatorHash: {
    defaultsTo: k.resolverDefault("resolveCreatorHash", [
      k.dependsOnArg("metadata"),
    ]),
  },
};
kinobi.update(
  new k.UpdateInstructionsVisitor({
    createTree: {
      name: "createTreeConfig",
      bytesCreatedOnChain: k.bytesFromAccount("treeConfig"),
    },
    mintToCollectionV1: {
      args: {
        metadataArgs: { name: "metadata" },
      },
    },
    transfer: {
      accounts: {
        leafOwner: { isSigner: "either" },
        leafDelegate: { isSigner: "either" },
      },
    },
    burn: {
      accounts: {
        leafOwner: { isSigner: "either" },
        leafDelegate: { isSigner: "either" },
      },
    },
    redeem: {
      accounts: {
        voucher: {
          defaultsTo: k.pdaDefault("voucher", {
            seeds: {
              merkleTree: k.accountDefault("merkleTree"),
              nonce: k.argDefault("nonce"),
            },
          }),
        },
      },
    },
    decompressV1: {
      accounts: {
        metadata: {
          name: "metadataAccount",
          defaultsTo: k.pdaDefault("metadata", {
            importFrom: "mplTokenMetadata",
            seeds: { mint: k.accountDefault("mint") },
          }),
        },
        masterEdition: {
          defaultsTo: k.pdaDefault("masterEdition", {
            importFrom: "mplTokenMetadata",
            seeds: { mint: k.accountDefault("mint") },
          }),
        },
        tokenAccount: {
          defaultsTo: k.pdaDefault("associatedToken", {
            importFrom: "mplToolbox",
            seeds: {
              mint: k.accountDefault("mint"),
              owner: k.accountDefault("leafOwner"),
            },
          }),
        },
        mintAuthority: {
          defaultsTo: k.pdaDefault("mintAuthority", {
            importFrom: "hooked",
            seeds: { mint: k.accountDefault("mint") },
          }),
        },
      },
    },
    setAndVerifyCollection: {
      accounts: {
        treeCreatorOrDelegate: { isSigner: "either" },
      },
      args: {
        ...hashDefaults,
        collection: {
          defaultsTo: k.accountDefault("collectionMint"),
        },
      },
    },
    verifyCollection: { args: { ...hashDefaults } },
    unverifyCollection: { args: { ...hashDefaults } },
    verifyCreator: { args: { ...hashDefaults } },
    unverifyCreator: { args: { ...hashDefaults } },
    // Remove deprecated instructions.
    setDecompressableState: { delete: true },
    // Remove unnecessary spl_account_compression instructions.
    append: { delete: true },
    closeEmptyTree: { delete: true },
    compress: { delete: true },
    initEmptyMerkleTree: { delete: true },
    insertOrAppend: { delete: true },
    noopInstruction: { delete: true },
    replaceLeaf: { delete: true },
    transferAuthority: { delete: true },
  })
);

// Set default values for structs.
kinobi.update(
  new k.SetStructDefaultValuesVisitor({
    createTreeConfigInstructionData: {
      public: k.vNone(),
    },
    metadataArgs: {
      symbol: k.vScalar(""),
      primarySaleHappened: k.vScalar(false),
      isMutable: k.vScalar(true),
      editionNonce: k.vNone(),
      tokenStandard: k.vSome(k.vEnum("TokenStandard", "NonFungible")),
      uses: k.vNone(),
      tokenProgramVersion: k.vEnum("TokenProgramVersion", "Original"),
    },
  })
);

// Custom tree updates.
kinobi.update(
  new k.TransformNodesVisitor([
    {
      // Add nodes to the splAccountCompression program.
      selector: { kind: "programNode", name: "splAccountCompression" },
      transformer: (node) => {
        k.assertProgramNode(node);
        return k.programNode({
          ...node,
          accounts: [
            ...node.accounts,
            k.accountNode({
              name: "merkleTree",
              data: k.accountDataNode({
                name: "merkleTreeAccountData",
                link: k.linkTypeNode("merkleTreeAccountData", {
                  importFrom: "hooked",
                }),
                struct: k.structTypeNode([
                  k.structFieldTypeNode({
                    name: "discriminator",
                    child: k.linkTypeNode("compressionAccountType"),
                  }),
                  k.structFieldTypeNode({
                    name: "treeHeader",
                    child: k.linkTypeNode("concurrentMerkleTreeHeaderData"),
                  }),
                  k.structFieldTypeNode({
                    name: "serializedTree",
                    child: k.bytesTypeNode(k.remainderSize()),
                  }),
                ]),
              }),
            }),
          ],
        });
      },
    },
    {
      // Use extra "proof" arg as remaining accounts.
      selector: (node) =>
        k.isInstructionNode(node) &&
        [
          "burn",
          "transfer",
          "redeem",
          "delegate",
          "setAndVerifyCollection",
          "verifyCollection",
          "unverifyCollection",
          "verifyCreator",
          "unverifyCreator",
          "verifyLeaf",
        ].includes(node.name),
      transformer: (node) => {
        k.assertInstructionNode(node);
        return k.instructionNode({
          ...node,
          remainingAccounts: k.remainingAccountsFromArg("proof"),
          argDefaults: {
            ...node.argDefaults,
            proof: k.valueDefault(k.vList([])),
          },
          extraArgs: k.instructionExtraArgsNode({
            ...node.extraArgs,
            struct: k.structTypeNode([
              ...node.extraArgs.struct.fields,
              k.structFieldTypeNode({
                name: "proof",
                child: k.arrayTypeNode(k.publicKeyTypeNode()),
              }),
            ]),
          }),
        });
      },
    },
  ])
);

// Transform tuple enum variants to structs.
kinobi.update(
  new k.UnwrapTupleEnumWithSingleStructVisitor([
    "ConcurrentMerkleTreeHeaderData",
  ])
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(
  new k.RenderJavaScriptVisitor(jsDir, {
    prettier,
    dependencyMap: {
      mplTokenMetadata: "@metaplex-foundation/mpl-token-metadata",
    },
  })
);

// Render Rust.
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(clientDir, "rust", "src", "generated");
kinobi.accept(
  new k.RenderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
  })
);
