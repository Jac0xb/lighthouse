const fs = require('node:fs');
const path = require('node:path');

// The pinned Codama renderer emits the type names from Kit 2.
function updateKitTypes(directory) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const filename = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      updateKitTypes(filename);
    } else if (filename.endsWith('.ts')) {
      const source = fs.readFileSync(filename, 'utf8');
      const updated = source.replace(
        /\bI(AccountMeta|AccountSignerMeta|Instruction|InstructionWithAccounts|InstructionWithData)\b/g,
        '$1'
      );
      if (updated !== source) fs.writeFileSync(filename, updated);
    }
  }
}

updateKitTypes(path.resolve(__dirname, '../../clients/kit-js/src/generated'));
