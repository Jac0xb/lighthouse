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
