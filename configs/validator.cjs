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
        label: 'Mpl Bubblegum',
        programId: 'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY',
        deployPath: getProgram('bubblegum.so'),
      },
      {
        label: 'Token Metadata',
        programId: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
        deployPath: getProgram('mpl_token_metadata.so'),
      },
      {
        label: 'SPL Account Compression',
        programId: 'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK',
        deployPath: getProgram('spl_account_compression.so'),
      },
      {
        label: 'SPL Noop',
        programId: 'noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV',
        deployPath: getProgram('spl_noop.so'),
      },
      {
        label: 'Blackhat',
        programId: 'blackhat1KfeRB3vL7UAj5rj9TUyLhuzQWqfDD3YsEhDuE',
        deployPath: getProgram('blackhat.so'),
      },
    ],
  },
};
