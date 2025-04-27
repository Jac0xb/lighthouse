import path from 'path';
import {
  createFromRoot,
  pdaNode,
  addPdasVisitor,
  constantPdaSeedNodeFromString,
  variablePdaSeedNode,
  publicKeyTypeNode,
  numberTypeNode,
  bottomUpTransformerVisitor,
  updateInstructionsVisitor,
  numberValueNode,
  instructionArgumentNode,
  enumValueNode,
  deleteNodesVisitor,
  InstructionArgumentNode,
} from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import {
  renderJavaScriptUmiVisitor,
  renderJavaScriptVisitor,
  renderRustVisitor,
} from '@codama/renderers';

// Paths to clients and lighthouse program directory.
const clientDir = path.join(__dirname, '..', 'clients');
const programDir = path.join(__dirname, '..', 'programs', 'lighthouse');

// Load Lighthouse IDL.
const lighthouseIdl = require(path.join(programDir, 'lighthouse.json'));

// Create Codama root node from Lighthouse IDL.
const codama = createFromRoot(rootNodeFromAnchor(lighthouseIdl));

// Add memory account PDA visitor (this will derive the PDA address using nodes in the instruction).
codama.update(
  addPdasVisitor({
    lighthouse: [
      pdaNode({
        name: 'memory',
        seeds: [
          constantPdaSeedNodeFromString('utf8', 'memory'),
          variablePdaSeedNode('payer', publicKeyTypeNode()),
          variablePdaSeedNode('memory_id', numberTypeNode('u8')),
        ],
      }),
    ],
  })
);

// Default memoryId to 0 for MemoryWrite instruction.
codama.update(
  updateInstructionsVisitor({
    memoryWrite: {
      arguments: {
        memoryId: {
          defaultValue: numberValueNode(0),
        },
      },
    },
  })
);

// Default logLevel to Silent for Log instruction.
codama.update(
  bottomUpTransformerVisitor([
    {
      select: '[instructionArgumentNode]logLevel',
      transform: (node) => {
        return instructionArgumentNode({
          ...(node as InstructionArgumentNode),
          defaultValue: enumValueNode('logLevel', 'Silent'),
        });
      },
    },
  ])
);

// Delete testAccountV1 type. This is used in testing but picked up by shank.
codama.update(deleteNodesVisitor(['testAccountV1']));

// Link overrides for hooked types. The hooked types are then defined in the hooked.(ts/rs) file.
const linkOverrides = {
  definedTypes: {
    accountDataAssertions: 'hooked',
    accountInfoAssertions: 'hooked',
    mintAccountAssertions: 'hooked',
    tokenAccountAssertions: 'hooked',
    stakeAccountAssertions: 'hooked',
    upgradeableLoaderStateAssertions: 'hooked',
    compactU64: 'hooked',
    bytes: 'hooked',
    compactBytes: 'hooked',
  },
};

// Render JavaScript SDK (@solana/kit v2.x.x).
const previewJsDir = path.join(clientDir, 'kit-js', 'src', 'generated');
const previewPrettier = require(
  path.join(clientDir, 'kit-js', '.prettierrc.json')
);
codama.accept(
  renderJavaScriptVisitor(previewJsDir, {
    prettierOptions: previewPrettier,
    linkOverrides,
  })
);

// Render JavaScript umi SDK.
const jsDir = path.join(clientDir, 'js', 'src', 'generated');
const prettier = require(path.join(clientDir, 'js', '.prettierrc.json'));
codama.accept(
  renderJavaScriptUmiVisitor(jsDir, {
    prettierOptions: prettier,
    linkOverrides,
  })
);

// Render Rust SDK.
const crateDir = path.join(clientDir, 'rust');
const rustDir = path.join(clientDir, 'rust', 'src', 'generated');
codama.accept(
  renderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
    linkOverrides,
  })
);
