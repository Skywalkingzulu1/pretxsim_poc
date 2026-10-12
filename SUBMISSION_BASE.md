# Submission — Base Ecosystem (Reusable Block)

## Project
**PreTxSim** — local, air-gapped transaction simulation firewall with a
first-class Base profile. Repo: `pretxsim_poc` (this repository).

## What it does
Rehearses every transaction against live Base state before signing: net
balance deltas, gas bounds, and structural risk flags (delegatecalls,
selfdestructs, storage blowouts, allowance changes, EIP-7702 delegations).
Runs as a drop-in `:8545` proxy, a `--ci` pipeline gate, or a batch
auditor — with zero outbound telemetry.

## How to run (judges)
```bash
# Proxy mode on Base Sepolia (verified live)
cargo run -- --chain base
# Batch audit with local-LLM briefs
cargo run -- --chain base --sim-file tx_payload.json --ai-brief
# Mainnet state: override the endpoint
cargo run -- --chain base --rpc-url "https://mainnet.base.org"
```
`--chain base` selects chain id 84532 with Base Sepolia as the default
endpoint. `RPC_TIMEOUT_SECS` bounds every upstream call (default 25s).

## Why Base
Base is where consumer crypto actually transacts — mobile-first users,
agent payments, and high-scam surfaces. A local simulation layer is the
safety primitive that lets invisible UX stay safe: every approval rehearsed,
every drainer pattern refused, no data leaving the machine.
