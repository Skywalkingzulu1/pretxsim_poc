# PreTxSim submissions

PreTxSim is a pre-execution firewall for agent commerce: the agent's intended transaction is inspected
before anything is signed. There are two shapes of it:

- **[pretxsim-mcp](https://github.com/Skywalkingzulu1/pretxsim-mcp)** - the current implementation, an MCP
  server (Rust, `rmcp`, streamable HTTP) that validates checkout calls on four layers: intent match, price
  anomaly, session cap, tax surface. Submitted to the **Amazon Developer Hackathon 2026, Alexa+ track**.
- **this repo** - the original air-gapped EVM pre-execution simulator that the hackathon submissions below
  were built around.

## Hackathon submissions (kept for the record, archived)

| when | hackathon | repo | notes |
|---|---|---|---|
| 2026-10 | Amazon Developer Hackathon, Alexa+ track | [pretxsim-mcp](https://github.com/Skywalkingzulu1/pretxsim-mcp) | current, maintained |
| 2026-10 | Metropolis (Monad) | [monad-metropolis-pretctxsim](https://github.com/Skywalkingzulu1/monad-metropolis-pretctxsim) | live testnet contract |
| 2026-10 | Colosseum | colosseum-pretctxsim | archived |
| 2026-10 | HackCanton S3 | hackcanton-pretctxsim | archived |
| 2026-10 | BNB Hack | bnb-hack-pretctxsim | archived |
| 2026-10 | ETH Lagos | eth-lagos-pretctxsim | archived |
| 2026-10 | Zecathon | zecathon-pretctxsim | archived |
| 2026-10 | TUM Blockchain + AI | tum-blockchain-ai-pretctxsim | archived |
| 2026-10 | BLI Legaltech | bli-legaltech-pretctxsim | archived |
| 2026-10 | Event Contracts | event-contracts-pretctxsim | archived |
| 2026-10 | From Vision to Impact | from-vision-to-impact-pretctxsim | archived |
| 2026-10 | GitLab Life After Code | gitlab-lifeaftercode-pretctxsim | archived |
| 2026-10 | European AI Hackathon | european-ai-hackathon-pretctxsim | archived |

Each submission repo is a read-only snapshot. All of them share the same engine in this repository; only the
demo contract and the pitch differ.
