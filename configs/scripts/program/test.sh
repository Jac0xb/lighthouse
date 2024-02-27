#!/bin/bash

# Import utils.
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source $(dirname $SCRIPT_DIR)/utils.sh

# Save external programs binaries to the output directory.
source ${SCRIPT_DIR}/dump.sh

# Go to the working directory.
cd $WORKING_DIR

if [ ! -z "$PROGRAM" ]; then
    PROGRAMS='["'${PROGRAM}'"]'
fi

if [ -z "$PROGRAMS" ]; then
    PROGRAMS="$(cat .github/.env | grep "PROGRAMS" | cut -d '=' -f 2)"
fi

# Get all command-line arguments.
ARGS=$*

# command-line arguments override env variable
if [ ! -z "$ARGS" ]; then
    PROGRAMS="[\"${1}\"]"
    shift
    ARGS=$*
fi

PROGRAMS=$(echo $PROGRAMS | jq -c '.[]' | sed 's/"//g')

WORKING_DIR=$(pwd)
SOLFMT="solfmt"
export SBF_OUT_DIR="${WORKING_DIR}/${OUTPUT}"
# npm run programs:build && npm run programs:test

# for p in ${PROGRAMS[@]}; do
#     cd ${WORKING_DIR}/programs/${p}

#     # if [ ! "$(command -v $SOLFMT)" = "" ]; then
#     #     echo "Running solana test-sbf for ${p}..."
#     #     RUST_LOG=error CARGO_TERM_COLOR=always cargo test ${ARGS} 2>&1 | ${SOLFMT}
#     # else
#     echo "Running solana test-sbf for ${p}..."
#     RUST_LOG=error cargo test --nocapture
#     # fi
# done

cd ${WORKING_DIR}/tests/lighthouse
RUST_LOG=error cargo test 2>&1