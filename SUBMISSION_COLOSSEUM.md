# Submission — Colosseum Crypto World's Fair (Base Track + General Pool)

## Project
**PreTxSim** — the local security layer for every onchain transaction,
entered on the **Base track** and eligible for the general pool. Repo:
`pretxsim_poc` (this repository).

## What it does
An open-source Rust/revm proxy that sits between any wallet, bot, or AI
agent and the network: it rehearses each transaction against live state,
flags structural risks deterministically, briefs via a local LLM, and gates
broadcast. Verified live on Base Sepolia (`--chain base`); the same engine
serves Ethereum L1 and Monad via `--chain` profiles.

## How to run (judges)
```bash
cargo run -- --chain base --sim-file tx_payload.json --ai-brief
```
Single command, no accounts, no API keys. Full evaluator guide in
`demo_runbook.md`; pitch deck in `docs/pitch/`.

## Why it wins here
- **Base-native story:** consumer safety infrastructure for the ecosystem
  where real users transact — invisible UX needs a local safety layer.
- **Venture-scalable:** open core with a paid perimeter (compliance policy
  packs, team fleet management, SLAs) — zero marginal serving cost because
  it runs on the customer's box.
- **Proven, not promised:** `cargo test` 25/25 green, live RPC verification,
  every claim reproducible from this repo.

## Links
- Code: this repository (`pretxsim_poc`)
- EVM track record: 12 prior hackathon-targeted builds (Sept–Oct 2026)
- Sister project (Canton): `canton-preview-gate` — same pre-execution
  thesis, Daml-native, 6/6 proofs green
