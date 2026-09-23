#!/bin/bash
set -euo pipefail

TARGET_REPO="gmh5225/awesome-web3-security"
ENTRY_TEXT='* [PreTxSim](https://github.com/Skywalkingzulu1/pretxsim_poc) - Air-gapped local EVM simulation proxy for security analysis and AI agent safety.'
BRANCH_NAME="add-pretxsim-$(date +%s)"
REPO_DIR="awesome-web3-security"
WORK_DIR="/c/Users/molel/AppData/Local/Temp/pretxsim_gtm_pr"

mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

if [ ! -d "$REPO_DIR" ]; then
  gh repo fork "$TARGET_REPO" --clone=true
fi

cd "$REPO_DIR"
git checkout main || git checkout master
git pull --ff-only

git checkout -b "$BRANCH_NAME"

TARGET_FILE="README.md"
if [ ! -f "$TARGET_FILE" ]; then
  echo "Error: README.md not found in $TARGET_REPO"
  exit 1
fi

if grep -q "PreTxSim" "$TARGET_FILE"; then
  echo "Notice: PreTxSim already referenced in $TARGET_FILE. Skipping."
  exit 0
fi

printf "\n%s\n" "$ENTRY_TEXT" >> "$TARGET_FILE"

git diff "$TARGET_FILE"

git add "$TARGET_FILE"
git commit -m "docs: add PreTxSim air-gapped EVM simulation proxy"
git push -u origin "$BRANCH_NAME"

gh pr create \
  --repo "$TARGET_REPO" \
  --base main \
  --head "$(gh api user --jq '.login'):$BRANCH_NAME" \
  --title "docs: add PreTxSim (local air-gapped EVM simulation)" \
  --body "Adds PreTxSim to the list. Built in Rust on revm for air-gapped local EVM simulation and AI agent execution safety."

echo "PR submitted safely for $TARGET_REPO."
