#!/usr/bin/env bash
set -euo pipefail

# Usage: ./add_awesome_entry.sh <target_repo> <entry_markdown_line>
# Example: ./add_awesome_entry.sh alexromanov/awesome-blockchain-testing "* [PreTxSim](https://github.com/Skywalkingzulu1/pretxsim_poc) - Air-gapped local EVM simulation proxy."

TARGET_REPO="${1:?Missing target repo, e.g., org/repo}"
ENTRY_TEXT="${2:?Missing markdown entry text}"
BRANCH_NAME="add-pretxsim-$(date +%s)"
REPO_DIR="$(basename "$TARGET_REPO")"

# Safety check: ensure clean state or work in temp/isolated workspace
WORK_DIR="/tmp/pretxsim_gtm_pr"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

if [ ! -d "$REPO_DIR" ]; then
  gh repo fork "$TARGET_REPO" --clone=true
fi

cd "$REPO_DIR"
git checkout main || git checkout master
git pull --ff-only

# Isolate branch
git checkout -b "$BRANCH_NAME"

TARGET_FILE="README.md"
if [ ! -f "$TARGET_FILE" ]; then
  echo "Error: README.md not found in $TARGET_REPO"
  exit 1
fi

# Idempotency guard: avoid duplicate PR/entries
if grep -q "PreTxSim" "$TARGET_FILE"; then
  echo "Notice: PreTxSim already referenced in $TARGET_FILE. Skipping."
  exit 0
fi

# Append safely under relevant section or end of file (customizable via awk/sed if specific section needed)
printf "\n%s\n" "$ENTRY_TEXT" >> "$TARGET_FILE"

# Verify diff is clean and minimal
git diff "$TARGET_FILE"

# Commit and push to fork
git add "$TARGET_FILE"
git commit -m "docs: add PreTxSim air-gapped EVM simulation proxy"
git push -u origin "$BRANCH_NAME"

# Create PR via GitHub CLI
gh pr create \
  --repo "$TARGET_REPO" \
  --base main \
  --head "$(gh api user --jq '.login'):$BRANCH_NAME" \
  --title "docs: add PreTxSim (local air-gapped EVM simulation)" \
  --body "Adds PreTxSim to the list. Built in Rust on revm for air-gapped local EVM simulation and AI agent execution safety."

echo "PR submitted safely for $TARGET_REPO."
