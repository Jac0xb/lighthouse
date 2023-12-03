#!/bin/bash

# Check if the first parameter (RPC URL) is provided
if [ -z "$1" ]; then
  echo "Error: RPC URL is required."
  echo "Usage: $0 <RPC_URL> <WALLET_PATH>"
  exit 1
fi

# Check if the second parameter (wallet path) is provided
if [ -z "$2" ]; then
  echo "Error: Wallet path is required."
  echo "Usage: $0 <RPC_URL> <WALLET_PATH>"
  exit 1
fi

echo "RPC URL: $1"
echo "Wallet Path: $2"

# Prompt the user for input
echo "Type CONFIRM to proceed with deployment:"
read user_input

# Check if the input is "CONFIRM"
if [ "$user_input" = "CONFIRM" ]; then
  echo "Proceeding with deployment to mainnet..."
  anchor deploy --program-name="lighthouse"  --provider.cluster="$1" --provider.wallet="$2"
else
  echo "Deployment cancelled."
fi