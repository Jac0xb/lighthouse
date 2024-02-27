const path = require('path');
const k = require('@metaplex-foundation/kinobi');

// Paths.
const clientDir = path.join(__dirname, '..', 'clients');
const programDir = path.join(
  __dirname,
  '..',
  'programs',
  'lighthouse',
  'program'
);

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(programDir, 'lighthouse.json')]);

// Update accounts.
// kinobi.update(
//   k.updateAccountsVisitor({
//     counter: {
//       seeds: [
//         k.constantPdaSeedNodeFromString('counter'),
//         k.variablePdaSeedNode(
//           'authority',
//           k.publicKeyTypeNode(),
//           'The authority of the counter account'
//         ),
//       ],
//     },
//   })
// );

// Update instructions.
// kinobi.update(
//   k.updateInstructionsVisitor({
//     create: {
//       byteDeltas: [k.instructionByteDeltaNode(k.accountLinkNode('counter'))],
//       accounts: {
//         counter: { defaultValue: k.pdaValueNode('counter') },
//         payer: { defaultValue: k.accountValueNode('authority') },
//       },
//     },
//   })
// );

// Set ShankAccount discriminator.
// const key = (name) => ({ field: 'key', value: k.enumValueNode('Key', name) });
// kinobi.update(
//   k.setAccountDiscriminatorFromFieldVisitor({
//     counter: key('counter'),
//   })
// );

// Render JavaScript.
const jsDir = path.join(clientDir, 'js', 'src', 'generated');
const prettier = require(path.join(clientDir, 'js', '.prettierrc.json'));
kinobi.accept(k.renderJavaScriptExperimentalVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, 'rust');
const rustDir = path.join(clientDir, 'rust', 'src', 'generated');
kinobi.accept(
  k.renderRustVisitor(rustDir, { formatCode: true, crateFolder: crateDir })
);
