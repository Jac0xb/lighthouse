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
  definedTypeLinkNode,
  fillDefaultPdaSeedValuesVisitor,
  updateInstructionsVisitor,
  publicKeyValueNode,
  pdaValueNode,
  setInstructionAccountDefaultValuesVisitor,
  updateAccountsVisitor,
  pdaLinkNode,
  structTypeNode,
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

// Paths.
const clientDir = path.join(__dirname, '..', 'clients');
const programDir = path.join(__dirname, '..', 'programs', 'lighthouse');

const lighthouseIdl = require(path.join(programDir, 'lighthouse.json'));
const codama = createFromRoot(rootNodeFromAnchor(lighthouseIdl));

// Memory account PDA
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

// How to set a default value for an account in an instruction.
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

codama.update(deleteNodesVisitor(['testAccountV1']));

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

// Render preview JavaScript.
const previewJsDir = path.join(clientDir, 'preview-js', 'src', 'generated');
const previewPrettier = require(
  path.join(clientDir, 'preview-js', '.prettierrc.json')
);
codama.accept(
  renderJavaScriptVisitor(previewJsDir, {
    prettierOptions: previewPrettier,
    linkOverrides,
  })
);

// Render JavaScript umi.
const jsDir = path.join(clientDir, 'js', 'src', 'generated');
const prettier = require(path.join(clientDir, 'js', '.prettierrc.json'));
codama.accept(
  renderJavaScriptUmiVisitor(jsDir, {
    prettierOptions: prettier,
    linkOverrides,
  })
);

// Render Rust.
const crateDir = path.join(clientDir, 'rust');
const rustDir = path.join(clientDir, 'rust', 'src', 'generated');
codama.accept(
  renderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
    linkOverrides,
  })
);
