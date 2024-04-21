const path = require('path');
const fs = require('fs');
const { generateIdl } = require('@metaplex-foundation/shank-js');

const binaryInstallDir = path.join(__dirname, '..', '.crates');
const programsDir = path.join(__dirname, '..', '');

getPrograms().forEach((program) => {
  generateIdl({
    generator: program.generator,
    programName: program.name,
    programId: program.address,
    idlDir: program.programDir,
    programDir: program.programDir,
    binaryInstallDir,
  });
});

function getPrograms() {
  const folders = process.env.PROGRAMS.split(/\s+/);
  const addresses = process.env.PROGRAMS_ADDRESSES.split(/\s+/);
  const binaries = process.env.PROGRAMS_BINARIES.split(/\s+/);

  return folders.map((folder, index) => {
    const isShank = fs
      .readFileSync(path.join(programsDir, folder, 'Cargo.toml'), 'utf8')
      .match(/shank/);

    return {
      folder,
      programDir: path.join(programsDir, folder),
      address: addresses[index],
      binary: binaries[index],
      name: binaries[index].replace(/\.so$/, ''),
      isShank,
      generator: isShank ? 'shank' : 'anchor',
    };
  });
}
