#!/bin/bash
set -euo pipefail

# Usage: ./submit_remaining.sh

WORK_DIR="/c/Users/molel/AppData/Local/Temp/pretxsim_remaining"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

submit_pr() {
  local TARGET_REPO="$1"
  local ENTRY_TEXT="$2"
  local REPO_DIR
  REPO_DIR="$(basename "$TARGET_REPO")"
  local BRANCH_NAME="add-pretxsim-$(date +%s)"
  local TARGET_FILE="README.md"

  echo "=== Processing $TARGET_REPO ==="

  if [ ! -d "$REPO_DIR" ]; then
    gh repo fork "$TARGET_REPO" --clone=true
  fi

  cd "$REPO_DIR"
  git checkout main || git checkout master
  git pull --ff-only
  git checkout -b "$BRANCH_NAME"

  if [ ! -f "$TARGET_FILE" ]; then
    echo "Checking for alternative README files..."
    TARGET_FILE=$(ls *.md 2>/dev/null | head -1)
    if [ -z "$TARGET_FILE" ]; then
      echo "Error: No README found in $TARGET_REPO"
      cd "$WORK_DIR"
      return 1
    fi
  fi

  if grep -qi "PreTxSim" "$TARGET_FILE"; then
    echo "Notice: PreTxSim already referenced in $TARGET_FILE. Skipping."
    cd "$WORK_DIR"
    return 0
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
  cd "$WORK_DIR"
}

submit_pr "HyperDevCommunity/AwesomeHyperEVM" "* [PreTxSim](https://github.com/Skywalkingzulu1/pretxsim_poc) - Air-gapped local EVM simulation proxy built on revm."
submit_pr "Consensys/ethereum-developer-tools-list" "* [PreTxSim](https://github.com/Skywalkingzulu1/pretxsim_poc) - Air-gapped local EVM simulation proxy built in Rust on revm."
