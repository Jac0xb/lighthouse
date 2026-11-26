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
  InstructionNode,
} from 'codama';
import { rootNodeFromAnchor } from '@codama/nodes-from-anchor';
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js';
import { renderVisitor as renderJavaScriptUmiVisitor } from '@codama/renderers-js-umi';
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust';

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

// Ensure every instruction has an accounts array (renderers-rust accesses `instruction.accounts.length + 1`).
codama.update(
  bottomUpTransformerVisitor([
    {
      select: '[instructionNode]',
      transform: (node) => {
        const ix = node as InstructionNode;
        return ix.accounts ? ix : { ...ix, accounts: [] };
      },
    },
  ])
);

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

async function main() {
  // Render JavaScript SDK (@solana/kit).
  const kitJsDir = path.join(clientDir, 'kit-js');
  const { parser: _kitParser, ...kitPrettier } = require(
    path.join(kitJsDir, '.prettierrc.json')
  );
  await codama.accept(
    renderJavaScriptVisitor(kitJsDir, {
      prettierOptions: kitPrettier,
      linkOverrides,
      kitImportStrategy: 'preferRoot',
      syncPackageJson: true,
    })
  );

  // Render JavaScript Umi SDK (legacy).
  const jsDir = path.join(clientDir, 'js', 'src', 'generated');
  const prettier = require(path.join(clientDir, 'js', '.prettierrc.json'));
  await codama.accept(
    renderJavaScriptUmiVisitor(jsDir, {
      prettierOptions: prettier,
      linkOverrides,
    })
  );

  // Render Rust SDK.
  const crateDir = path.join(clientDir, 'rust');
  codama.accept(
    renderRustVisitor(crateDir, {
      formatCode: true,
      syncCargoToml: false,
      linkOverrides,
      dependencyVersions: {
        borsh: { version: '^1.0', features: ['derive'] },
        'num-derive': '^0.4',
        'num-traits': '^0.2',
        serde: { version: '^1.0', features: ['derive'], optional: true },
        serde_with: { version: '^3.21', optional: true },
        'solana-account-info': '^3.1',
        'solana-address': {
          version: '^2.2',
          features: ['borsh', 'copy', 'curve25519', 'decode'],
        },
        'solana-cpi': '^3.1',
        'solana-instruction': '^3.2',
        'solana-program-error': '^3.0',
        thiserror: '^1.0',
      },
      traitOptions: {
        baseDefaults: [
          'borsh::BorshSerialize',
          'borsh::BorshDeserialize',
          'Clone',
          'Debug',
          'Eq',
          'PartialEq',
          'serde::Serialize',
          'serde::Deserialize',
        ],
        featureFlags: {
          serde: ['serde::Serialize', 'serde::Deserialize'],
        },
      },
    })
  );
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
