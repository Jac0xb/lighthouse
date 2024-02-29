#!/bin/bash
# This script simplifies running commands with dotenvx and common environment flags.

# Check if at least one argument is provided
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <command>"
  exit 1
fi

# Run the command with dotenvx, adding common flags
dotenvx run -q -f .env.local -f .env -- "$@"