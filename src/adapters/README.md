# PreTxSim — Hackathon Adapters Framework

Modular per-hackathon target adapters for PreTxSim, the air-gapped local EVM simulation proxy (Rust, `revm`). Each adapter implements a shared `Adapter` trait and runs a pure structural inspection that never broadcasts — keys, intents, and state stay on the local machine.

## Repository layout

```
src/adapters/
  mod.rs          # Adapter trait, AdapterResult, AdapterVerdict, default_registry()
  registry.rs     # AdapterRegistry (register / get / inspect / names)
  evm.rs          # EVM targets via shared revm engine: ethereum, monad, arc, base
  sui.rs          # Sui agent wrapper                   (Sui Overflow 2026)
  solana.rs       # Solana privacy proxy                (Solana Privacy Hack)
  stellar.rs      # Stellar / Poseidon ZK adapter       (HackMeridian / Stellar ZK)
  aleo.rs         # Aleo private app simulation         (Aleo Privacy Virtual)
  mcp.rs          # Amazon MCP / Agent Skill proxy      (Amazon Developer Hackathon)
  agent_firewall.rs # Autonomous agent firewall         (Open Agent Hackathon 2026)
  zk.rs           # ZK simulation wrapper               (Privacy Virtual / zkVerify)
```

## The `Adapter` trait

```rust
pub trait Adapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn chain_profile(&self) -> Option<ChainProfile>;
    fn inspect(&self, tx: &Value) -> AdapterResult;
}
```

`inspect` is synchronous and air-gapped — it reads the transaction JSON, applies deterministic rules, and returns an `AdapterResult` with a `verdict` (`ALLOW` / `WARN` / `BLOCK`), `reasons`, and structured `details`. EVM-native targets delegate to the existing revm `run_single_tx` engine; non-EVM targets (Sui, Solana, Stellar, Aleo, MCP, agent firewall, ZK) inspect their native transaction shape.

## Delivery matrix (hackathon → adapter)

| Hackathon | Track / Focus | Adapter | Kilo deliverable | Opencode deliverable |
|---|---|---|---|---|
| Sui Overflow 2026 | Autonomous AI agents | `sui` | Sui agent wrapper | State delta demo GIF |
| Meta Global AI Developer Hackathon | Verifiable model checkpoints | (planned) | Verifiable model checkpoint middleware | Recording |
| Colosseum: Crypto World's Fair | Multi-chain (Ethereum + Base) | `evm` (base/ethereum) | Multi-chain proxy | Recording |
| Monad Metropolis Hackathon | Trust / Identity & AI infra | `evm` (monad) | Trust/Identity AI infra adapter | Recording |
| HackerNoon Proof of Usefulness | Utility benchmark | (planned) | Utility benchmark hook | Recording |
| Solana Privacy Hack | Private Payments / Privacy Tooling | `solana` | Privacy tooling RPC proxy | Recording |
| HackerNoon Decentralize AI | Verifiable AI | (planned) | Verifiable AI inference hook | Recording |
| Privacy Virtual Hackathon | ZK / privacy dApps | `zk` | ZK simulation wrapper | Recording |
| HackMeridian / Stellar Real-World ZK | Stellar / Poseidon | `stellar` | Stellar/Poseidon hash adapter | Recording |
| Open Agent Hackathon 2026 | Autonomous agents | `agent_firewall` | Autonomous agent firewall | Recording |
| SEED Hackathon 2026 | Autonomy & Accountability | (planned) | Autonomy accountability engine | Recording |
| Aleo Privacy Virtual Hackathon | Private dApps | `aleo` | Full-stack private app simulation | Recording |
| Amazon Developer Hackathon | Alexa+ MCP / Agent Skill | `mcp` | MCP server / Agent Skill proxy | Recording |
| Event Contracts Hackathon | Web3 event-contract tooling | (planned) | Event verification hook | Recording |
| Mezo / Arc / Midnight / Circle / WEEX / Bitget / Nebius / GitLab / Ryo-chan | RPC/micro-grant bindings | `evm` (arc + x402) | Modular RPC bindings | Demo assets |
| zkVerify Foundation Hackathon | ZK verification infra | `zk` | ZK verification case study | Recording |

## CLI usage

Route any transaction through a named target adapter:

```bash
# Single tx via stdin (CI mode — exits 1 on BLOCK)
echo '{"curve":"bn254","proof":"0x...","public_inputs":["0x1"],"verification_key":"0xvk"}' \
  | cargo run -- --ci --target zk

# Batch audit from a JSON file (array or single object)
cargo run -- --sim-file tx_payload.json --target mcp

# EVM-native target with a chain profile
cargo run -- --sim-file tx_payload.json --target monad --ai-brief
```

List registered targets:

```rust
let reg = adapters::default_registry();
reg.names(); // ["aleo", "arc", "agent_firewall", "base", "evm", "mcp", "monad", "sui", "solana", "stellar", "zk"]
```

## Test status

```
cargo test  -> 40 passed, 0 failed
```
25 original unit tests (x402 gate, config, inspector, ollama brief) + 15 adapter unit tests (sui, solana, stellar, aleo, mcp, agent_firewall, zk).

## Per-hackathon repos

Each adapter is self-contained in its own module under `src/adapters/` with its own unit tests, so any target can be extracted into a dedicated per-hackathon repository by copying its module + the shared `Adapter`/`registry` core and adding an event-specific pitch README.

See `agent.md` for the Kilo/Opencode multi-agent coordination board (audit baseline, delivery matrix checkboxes, activity log).
