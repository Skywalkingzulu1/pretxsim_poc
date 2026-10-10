# Submission — Agentmaxxing (Self-Custodial AI Agent Spending Firewall)

## Project
**PreTxSim** — the spending firewall between an AI agent's intent and
irreversible onchain execution. Repo: `pretxsim_poc` (this repository).

## What it does
Agents route their JSON-RPC calls through the local proxy
(`http://127.0.0.1:8545`). Every payment is rehearsed in revm, checked by
the deterministic RiskInspector, and explained by a **local** LLM brief
(Ollama — keys, intents, and traces never leave the box). Out-of-policy
payments are refused before broadcast.

## Quickstart: route your agent through the firewall

```bash
# 1. Start the proxy (Base Sepolia agent-wallet profile shown)
cargo run -- --chain base
```

```python
# 2. LangChain / AutoGen / Kilo Code: point the EVM provider at the proxy
from web3 import Web3

w3 = Web3(Web3.HTTPProvider("http://127.0.0.1:8545"))  # PreTxSim gate
# Agent builds the tx as usual...
tx = {"from": agent, "to": vendor, "value": w3.to_wei(0.01, "ether")}
# ...and the firewall rehearses + briefs BEFORE anything is signed.
pre = w3.provider.make_request("eth_sendTransaction", [tx])
```

```bash
# 3. Or batch-audit a runbook of planned agent payments (CI mode)
cargo run -- --chain base --sim-file agent_runbook.json --ai-brief
# exit 1 if any payment is blocked -> fail the agent run before funds move
```

The structured brief (`brief.verdict`: PASS/WARN/BLOCK + plain-language
summary) is advisory: deterministic checks gate, the model only explains.
Model/endpoint configurable via `OLLAMA_URL` / `OLLAMA_MODEL` env.

## Why it fits
- **Self-custodial:** the agent keeps its keys; the firewall only rehearses
  and refuses. No custodian, no cloud, no telemetry.
- **Auditable:** every rehearsal emits a JSON report; CI mode turns agent
  runbooks into gated pipelines.

## Verification
- `cargo check`: clean. `cargo test`: **25 passed, 0 failed**.
- Live-tested against Base Sepolia: structured output, exit 0 on clean.
