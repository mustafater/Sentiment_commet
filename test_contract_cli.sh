#!/bin/bash
set -e

CONTRACT_ID="CBI67BL6QOBUCP2NCFFPIQAG2RZQFFRROWSQFJH7EFRVGGNC6GRSBHSW"
IDENTITY="deployer"
NETWORK="testnet"

echo "--- Testing Contract: $CONTRACT_ID ---"

# 1. Get initial stats
echo "1. Fetching initial stats..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  get_stats

# 2. Submit a negative comment
echo "2. Submitting negative comment (CLI)..."
# args: comment_id (String), score (u32), content_hash (String)
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  submit_negative \
  --comment_id "cli_test_comment_1" \
  --score 10 \
  --content_hash "hash_cli_1"

# 3. Get stats again
echo "3. Fetching updated stats..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  get_stats

echo "--- Done ---"
