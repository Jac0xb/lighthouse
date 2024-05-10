#!/bin/bash

# Import utils.
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source $(dirname $SCRIPT_DIR)/utils.sh

# Save external programs binaries to the output directory.
source ${SCRIPT_DIR}/dump.sh

# Go to the working directory.
cd $WORKING_DIR

# Get command-line arguments.
ARGS=$*
if [ ! -z "$ARGS" ]; then
    PROGRAMS=$1
    shift
    ARGS=$*
fi

# Create the output directory if it doesn't exist.
if [ ! -d ${PROGRAMS_OUTPUT_DIR} ]; then
    mkdir ${PROGRAMS_OUTPUT_DIR}
fi

for p in ${PROGRAMS[@]}; do
    printf "\nBuilding program: ${p}...\n"

    cd ${WORKING_DIR}/${p}
    cargo build-sbf --sbf-out-dir ${PROGRAMS_OUTPUT_DIR} $ARGS
done
