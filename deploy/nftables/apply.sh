#!/usr/bin/env bash
set -euo pipefail

if ! command -v nft >/dev/null 2>&1; then
  echo "nft not found; please install nftables" >&2
  exit 1
fi

sudo nft -f "$(dirname "$0")/allow-10443.nft"
echo "Applied nftables rules to allow only port 10443 inbound."
