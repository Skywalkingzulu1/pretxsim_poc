# PreTxSim on Arc Mainnet — x402 Local Proxy Deploy

**Target:** Arc Microgrants (Circle) — working tool on Arc Mainnet.

Arc prices gas in **native USDC** and serves machine-to-machine payments
carrying **x402 payloads**. PreTxSim runs a local proxy that rehearses
every Arc transaction, inspects the x402 envelope against agent spending
limits, and reports gas in USDC terms — all air-gapped.

## Prerequisites

- Rust stable, an Arc Mainnet RPC endpoint, optional local Ollama.
- Verify the Arc chain id against current Circle docs; the default is set
  in `src/config.rs` (`ChainProfile::arc`) and overridable at runtime.

## Run the proxy on Arc

```bash
RPC_URL="https://rpc.arc.network" cargo run -- --chain arc
# or: cargo run -- --chain arc --rpc-url "https://<your-arc-rpc>"
```

The banner confirms `x402 gate: ON (USDC-native gas)`. Point any wallet,
bot, or agent at `http://127.0.0.1:8545`.

## Batch-check an x402 payment file

Attach the payment envelope as an `x402` object on the tx:

```json
[
  {
    "from": "0x...",
    "to": "0x...",
    "value": "0x0",
    "gas": "0x186a0",
    "data": "0x...",
    "x402": {
      "amount": "2500000",
      "asset": "USDC",
      "recipient": "0x...",
      "nonce": "agent-run-042",
      "expires_at": 1790000000
    }
  }
]
```

```bash
cargo run -- --chain arc --sim-file arc_payments.json --ai-brief
```

Amounts are integer **micro-USDC** (`2500000` = 2.5 USDC). The gate blocks
on: over-limit amounts (default cap 10,000 USDC), non-USDC assets, invalid
recipients, missing nonces, missing/expired `expires_at`. Each output entry
carries an `x402` inspection (`verdict`, `reasons`) and `gas_asset: "USDC"`.
Exit code is 1 if any entry is blocked — structurally or by x402 policy.

## What "USDC-native gas" means here

Gas is estimated in gas units by revm exactly as on any EVM chain; because
Arc settles gas in USDC, the output labels `gas_asset: "USDC"` so downstream
tooling prices it directly. No conversion guessing inside the firewall.

## Docker

`docker-compose.yml` starts the proxy alongside Ollama. Set
`RPC_URL` to your Arc endpoint before `docker compose up -d`.
