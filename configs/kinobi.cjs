const path = require('path');
const k = require('@metaplex-foundation/kinobi');

// Paths.
const clientDir = path.join(__dirname, '..', 'clients');
const programDir = path.join(__dirname, '..', 'programs', 'lighthouse');

// Instanciate Kinobi.
const kinobi = k.createFromIdl(path.join(programDir, 'lighthouse.json'));

// Memory account PDA
kinobi.update(
  k.addPdasVisitor({
    lighthouse: [
      k.pdaNode('memory', [
        k.constantPdaSeedNodeFromString('utf8', 'memory'),
        k.variablePdaSeedNode('payer', k.publicKeyTypeNode()),
        k.variablePdaSeedNode('memory_id', k.numberTypeNode('u8')),
      ]),
    ],
  })
);

// How to set a default value for an account in an instruction.
kinobi.update(
  k.updateInstructionsVisitor({
    memoryWrite: {
      // accounts: {
      //   systemProgram: {
      //     defaultValue: k.publicKeyValueNode('<pubkey>'),
      //   },
      //   memoryAccount: {
      //     defaultValue: k.pdaValueNode('memory'),
      //   },
      // },
      arguments: {
        memoryId: {
          defaultValue: k.numberValueNode(0),
        },
      },
    },
  })
);

kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: '[instructionArgumentNode]logLevel',
      transform: (node) => {
        return k.instructionArgumentNode({
          ...node,
          defaultValue: k.enumValueNode('logLevel', 'Silent'),
        });
      },
    },
  ])
);

kinobi.update(k.deleteNodesVisitor(['testAccountV1']));

for (const definedType of [
  'accountInfoAssertions',
  'mintAccountAssertions',
  'tokenAccountAssertions',
  'stakeAccountAssertions',
  'upgradeableLoaderStateAssertions',
  'compactU64',
  'bytes',
  'compactBytes',
]) {
  kinobi.update(
    k.bottomUpTransformerVisitor([
      {
        select: ['[definedTypeLinkNode]', definedType],
        transform: (node) => {
          return k.definedTypeLinkNode(node.name, 'hooked');
        },
      },
    ])
  );
}

// How to log the kinobi tree
// kinobi.accept(k.consoleLogVisitor(k.getDebugStringVisitor({ indent: true })));

//
// Setting a default value for an instruction arg.
//
// kinobi.update(
//   k.setStructDefaultValuesVisitor({
//     memoryWrite: {
//       memory_id: k.numberValueNode(0),
//     },
//   })
// );

// Render preview JavaScript.
const previewJsDir = path.join(clientDir, 'preview-js', 'src', 'generated');
const previewPrettier = require(
  path.join(clientDir, 'preview-js', '.prettierrc.json')
);
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(previewJsDir, {
    prettier: previewPrettier,
  })
);

// Render preview JavaScript.
const jsDir = path.join(clientDir, 'js', 'src', 'generated');
const prettier = require(path.join(clientDir, 'js', '.prettierrc.json'));
kinobi.accept(k.renderJavaScriptVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, 'rust');
const rustDir = path.join(clientDir, 'rust', 'src', 'generated');
kinobi.accept(
  k.renderRustVisitor(rustDir, { formatCode: true, crateFolder: crateDir })
);
