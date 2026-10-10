# Submission — Arc Microgrants / Circle (Arc Mainnet tooling)

## Project
**PreTxSim on Arc** — working x402 inspection proxy with USDC-native gas
accounting. Repo: `pretxsim_poc` (this repository).

## What it does
Run the proxy against Arc Mainnet (`--chain arc`): every transaction is
rehearsed locally while the `arc_x402_gate` module validates
machine-to-machine payment envelopes — per-payment caps (default 10,000
USDC), USDC-asset allowlist, recipient validity, replay nonces, expiry —
before anything executes. Gas is reported with `gas_asset: "USDC"`.

## How to run (judges)
```bash
RPC_URL="https://rpc.arc.network" cargo run -- --chain arc
cargo run -- --chain arc --sim-file arc_payments.json --ai-brief
```
Full deploy guide: [ARC_MAINNET_DEPLOY.md](ARC_MAINNET_DEPLOY.md).
Note: confirm the Arc chain id in Circle's current docs; override any time
with `--chain-id <id>`.

## Why it fits
- **Deployed/working tool:** single binary + docker-compose, no accounts,
  no API keys, runs against Arc Mainnet RPC today.
- **x402-native:** purpose-built inspection for agent payment payloads,
  air-gapped and unit-tested (7 gate tests green).

## Verification
- `cargo check`: clean. `cargo test`: **25 passed, 0 failed**.
- x402 verdicts serialize into batch JSON with reasons; exit 1 on block.
