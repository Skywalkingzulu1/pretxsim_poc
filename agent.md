# preTxsim Multi-Hackathon Master Board (`agent.md`)

## Repository Audit Baseline
- **Repository**: `Skywalkingzulu1/pretxsim_poc` (cloned locally to `C:\Users\molel\preTxsim`)
- **Audited By**: Kilo
- **Partner**: Opencode (integration test suites + terminal demo recording)
- **Status**: Phase 0 complete. Build + test baseline green.

## Toolchain (verified local)
- git 2.52.0, rustc 1.94.1, cargo 1.94.1, Python 3.13.14, node v24.12.0
- Rust edition 2021; crate `pretxsim_poc` v0.1.0

## Codebase Inventory (audited)
**Source** (`src/`, 7 core modules + `adapters/`, 9 modules):
- `main.rs` — JSON-RPC proxy (`eth_sendTransaction`, `eth_blockNumber`, `eth_getBalance`), `--ci` batch/stdin mode, `--sim-file` batch mode, `--monad-parallel-check`, structural BLOCK heuristic, `run_single_tx`, `--target` adapter routing
- `inspector.rs` — `RiskInspector` (delegatecall 0xF4, SSTORE 0x55, selfdestruct 0xFF, EIP-7702 designator parse, balance/storage/log diffs), `EVMStructuralAnalysis`
- `agent.rs` — `query_local_agent` to Ollama `smollm2:135m` (advisory PASS/WARN only)
- `rpc_db.rs` — `RpcCacheDB` (standard anvil methods: eth_getBalance/nonce/code/storage/block, batch + caching)
- `config.rs` — `ChainProfile` registry: ethereum (1), monad (143), arc (5042002), base (84532); `--chain`, `--rpc-url`, `--chain-id`, `--ci`, `--sim-file`, `--ai-brief`, `--monad-parallel-check`, `--target`; `RPC_TIMEOUT_SECS` default 25s
- `ollama_risk_brief.rs` — `RiskBrief` (PASS/WARN/BLOCK/Unknown), `parse_verdict`, `build_prompt`
- `arc_x402_gate.rs` — `X402Payment`/`GateLimits`/`X402Inspection`; micro-USDC caps, asset allowlist, recipient/nonce/expiry checks; `extract` from tx JSON
- `adapters/mod.rs` — `Adapter` trait, `AdapterResult`, `AdapterVerdict`, `default_registry()`
- `adapters/registry.rs` — `AdapterRegistry` (register/get/inspect/names)
- `adapters/evm.rs` — EVM targets via shared revm engine (ethereum, monad, arc, base)
- `adapters/sui.rs` — Sui agent wrapper (TransferObject/Publish/Call hazard inspection)
- `adapters/solana.rs` — Solana privacy proxy (program-id allowlist, encrypted memo)
- `adapters/stellar.rs` — Stellar/Poseidon ZK adapter (memo hash, sponsor, ops)
- `adapters/aleo.rs` — Aleo private app sim (encrypted records, viewing key)
- `adapters/mcp.rs` — Amazon MCP / Agent Skill proxy (destructive + spend cap)
- `adapters/agent_firewall.rs` — Open Agent autonomous firewall (drain patterns, consent)
- `adapters/zk.rs` — ZK simulation wrapper (curve, proof, vk, public inputs)

**Tests**: 40 passed, 0 failed (`cargo test` green). 25 original + 15 new adapter unit tests.

**Fixtures & demos**:
- `examples/demo_txs.json`, `monad_tx_batch.json`, `onetx.json`
- `demo/tx_risky.json`, `demo/tx_7702.json`, `demo/setup_chain.sh` (anvil fixture), `demo/run_demo_with_pipe.sh`, `demo/render.sh` (agg+ffmpeg GIF/MP4)
- `demo/contracts/src` (Foundry: Risky + Delegatee fixtures)
- `docs/assets/pretxsim_demo.gif`, `pretxsim_demo.mp4`, `demo.tape` (existing demo GIF)
- `docker-compose.yml` (proxy + Ollama), `Dockerfile`

**Submission docs already present**: SUBMISSION_COLOSSEUM.md, SUBMISSION_MONAD.md, SUBMISSION_ARC.md, SUBMISSION_BASE.md, SUBMISSION_AGENTMAXXING.md, README_MONAD.md, README-hackcanton.md, ARC_MAINNET_DEPLOY.md
**Funding**: funding/w3f_application_pretxsim.md, batch_grant_proposals.md (Arbitrum/Optimism/Autonomys), esp_application_pretxsim.md, sprint_summary.md
**Taxonomy**: HACKATHON_TAXONOMY.md (12 qualifying events)

## Build Baseline
- `cargo build`: compiles clean
- `cargo test`: **40 passed, 0 failed**

## Smoke-Tested CLI Paths
- `echo '<tx>' | ./target/debug/pretxsim_poc.exe --ci --target mcp` → `{"verdict":"ALLOW",...}` exit 0
- `echo '<zk-proof-empty>' | ./target/debug/pretxsim_poc.exe --ci --target zk` → `{"verdict":"BLOCK",...}` exit 1
- `--sim-file <file> --target <name>` batch routing through `AdapterRegistry`

## Universal Hackathon Delivery Matrix
(Kilo = adapter/middleware; Opencode = integration suite + terminal demo GIF)

- [x] **Kilo adapters built**: sui, solana, stellar, aleo, mcp, agent_firewall, zk, evm (ethereum/monad/arc/base)
- [ ] **Sui Overflow 2026** (Kilo: Sui agent wrapper | Opencode: State delta demo GIF)
- [ ] **Meta Global AI Developer Hackathon** (Kilo: Verifiable model checkpoint middleware | Opencode: Recording)
- [ ] **Colosseum: Crypto World's Fair** (Kilo: Multi-chain proxy (evm base) | Opencode: Recording)
- [ ] **Monad Metropolis Hackathon** (Kilo: Trust/Identity AI infra adapter (evm monad) | Opencode: Recording)
- [ ] **HackerNoon Proof of Usefulness** (Kilo: Utility benchmark hook | Opencode: Recording)
- [ ] **Solana Privacy Hack** (Kilo: Solana privacy proxy | Opencode: Recording)
- [ ] **HackerNoon Decentralize AI** (Kilo: Verifiable AI inference hook | Opencode: Recording)
- [ ] **Privacy Virtual Hackathon** (Kilo: ZK simulation wrapper | Opencode: Recording)
- [ ] **HackMeridian / Stellar Real-World ZK** (Kilo: Stellar/Poseidon adapter | Opencode: Recording)
- [ ] **Open Agent Hackathon 2026** (Kilo: Autonomous agent firewall | Opencode: Recording)
- [ ] **SEED Hackathon 2026** (Kilo: Autonomy accountability engine | Opencode: Recording)
- [ ] **Aleo Privacy Virtual Hackathon** (Kilo: Aleo private app sim | Opencode: Recording)
- [ ] **Amazon Developer Hackathon** (Kilo: MCP/Agent Skill proxy | Opencode: Recording)
- [ ] **Event Contracts Hackathon** (Kilo: Event verification hook | Opencode: Recording)
- [ ] **Mezo / Arc / Midnight / Circle / Trading Events (WEEX/Bitget/Nebius/GitLab/Ryo-chan)** (Kilo: Modular RPC/micro-grant bindings (evm arc + x402) | Opencode: Demo assets)
- [ ] **zkVerify Foundation Hackathon** (Kilo: ZK verification infra case study (zk adapter) | Opencode: Recording)

## Activity Log
- **[Audit]**: Cloned `Skywalkingzulu1/pretxsim_poc` -> `C:\Users\molel\preTxsim`. Layout, 7 core modules, 25 tests, demo assets, submission/funding docs logged. Build + `cargo test` green (25/25).
- **[Adapters]**: Scaffolded `src/adapters/` framework (`Adapter` trait, `AdapterResult`, `AdapterVerdict`, `AdapterRegistry`, `default_registry()`). Implemented sui, solana, stellar, aleo, mcp, agent_firewall, zk adapters + evm wrapper for ethereum/monad/arc/base profiles. Added `--target` CLI flag with `--ci`/`--sim-file` routing. `cargo test`: **40/40 green**. `--target` smoke-tested via stdin (mcp ALLOW exit 0, zk BLOCK exit 1).
- **[Next]**: Kilo — per-event pitch/glue files + remaining adapters as needed. Opencode — integration test suites + terminal demo GIFs per matrix. Keep `agent.md` checkboxes/log in sync.
