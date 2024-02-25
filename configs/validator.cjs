const path = require('path');

const programDir = path.join(__dirname, '..', 'programs');

function getProgram(programName) {
  return path.join(programDir, '.bin', programName);
}

module.exports = {
  validator: {
    commitment: 'processed',
    programs: [
      {
        label: 'Blackhat',
        programId: 'blackhat1KfeRB3vL7UAj5rj9TUyLhuzQWqfDD3YsEhDuE',
        deployPath: getProgram('blackhat.so'),
      },
    ],
  },
};
