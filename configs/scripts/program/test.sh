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


cd ${WORKING_DIR}/programs/lighthaus
echo "Running solana test-sbf for ${p}..."
RUST_LOG=error RUST_BACKTRACE=1 BPF_OUT_DIR=${WORKING_DIR}/configs/.programs cargo test 2>&1

cd ${WORKING_DIR}/tests/lighthaus/
RUST_LOG=error RUST_BACKTRACE=1 cargo test 2>&1