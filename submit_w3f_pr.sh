#!/bin/bash
set -euo pipefail

WORK_DIR="/c/Users/molel/AppData/Local/Temp/w3f_pr"
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

# Fork first (just creates the fork on GitHub, no clone yet)
gh repo fork w3f/Grants-Program --fork-name "Grants-Program"

# Shallow clone the fork (not the upstream which is huge)
git clone --depth 1 "https://github.com/$(gh api user --jq '.login')/Grants-Program.git"
cd Grants-Program

# Create branch from master
BRANCH="pretxsim-application-$(date +%s)"
git checkout -b "$BRANCH"

# Create applications directory if needed
mkdir -p applications

# Copy application file
cp "/c/Users/molel/grant/funding/w3f_application_pretxsim.md" "applications/pretxsim.md"

# Commit
git add "applications/pretxsim.md"
git commit -m "feat: add PreTxSim air-gapped EVM simulation proxy application"

# Push
git push -u origin "$BRANCH"

# Create PR
MY_USER=$(gh api user --jq '.login')
PR_URL=$(gh pr create --repo w3f/Grants-Program \
  --base master \
  --head "${MY_USER}:${BRANCH}" \
  --title "Application: PreTxSim - Air-gapped EVM simulation proxy for Substrate parachains" \
  --body "This PR submits the PreTxSim grant application for a Level 2 open-source grant. PreTxSim is an air-gapped local EVM simulation proxy built in Rust on revm, designed to enhance security for users of Substrate-based EVM parachains (Moonbeam, Astar) by providing pre-execution transaction analysis.")

echo "PR created: $PR_URL"
