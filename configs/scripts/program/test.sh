#!/bin/bash

# Import utils.
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source $(dirname $SCRIPT_DIR)/utils.sh

# Save external programs binaries to the output directory.
source ${SCRIPT_DIR}/dump.sh

# Go to the working directory.
cd $WORKING_DIR

# Get all command-line arguments.
ARGS=$*
if [ ! -z "$ARGS" ]; then
    PROGRAMS="[\"${1}\"]"
    shift
    ARGS=$*
fi

SOLFMT="solfmt"
export SBF_OUT_DIR="${WORKING_DIR}/${OUTPUT}"

echo "Building programs..."

for p in ${PROGRAMS[@]}; do
    cd ${WORKING_DIR}/${p}

    echo "Running solana test-sbf for ${p}..."
    RUST_LOG=error BPF_OUT_DIR=${WORKING_DIR}/.bin cargo test 2>&1
done

cd ${WORKING_DIR}/tests/lighthouse
RUST_LOG=error cargo test 2>&1