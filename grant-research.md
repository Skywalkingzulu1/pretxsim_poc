# Grant Application Research: PreTxSim

**Project:** PreTxSim — Air-gapped local EVM simulation proxy built in Rust on revm  
**Date:** Wed Sep 23 2026  
**Researcher:** Kilo

---

## 1. Web3 Foundation (W3F) Grants-Program

**Repository:** `https://github.com/w3f/Grants-Program`  
**Template:** `applications/application-template.md`  
**Status: DISCONTINUED** — The W3F Grants Program is no longer accepting applications.

### Important Status Update
> The Web3 Foundation has decided to discontinue the general Grants Program. We are not accepting new applications at this point. Please do not submit new pull requests.

**Alternative funding sources mentioned by W3F:**
- **Polkadot Treasury** — Community-governed funding via Treasury proposals
- **Hackathons** — Periodic hackathon events
- **Other Grant Programs** — Ecosystem-specific or community-run grant initiatives

### Historical Template / Format (for future reference)

If the program reopens or for similar applications, the required structure was:

**Submission Mechanism:** GitHub Pull Request to `w3f/Grants-Program`
1. Fork the repository
2. Create a copy of `applications/application-template.md`
3. Name it `project_name.md`
4. Fill out all mandatory fields
5. Create a PR containing only the new file

**Required Fields/Sections:**
- **Team Name** (legal entity)
- **Payment Details** — DOT address + USDC on Polkadot AssetHub address
- **Level** — 1, 2, or 3 (funding tier)
- **Project Overview** — tagline, description, Substrate/Polkadot/Kusama integration, motivation
- **Project Details** — mockups/designs, data models/API specs, tech stack, architecture, PoC/MVP, limitations
- **Ecosystem Fit** — placement in ecosystem, target audience, problem solved, evidence, comparable projects
- **Team** — members, contact, legal structure, experience, GitHub profiles, LinkedIn
- **Development Status** — prior work, research, forum discussions
- **Development Roadmap** — milestones with deliverables, estimated duration, FTE, costs (USD), DOT percentage
  - Each milestone must include: License (Apache 2.0/GPLv3/MIT/Unlicense), Documentation, Testing guide, Dockerfile, optional article
- **Future Plans** — maintenance/sustainability plan, short-term enhancement, long-term intentions
- **Referral Program** (optional) — referrer name, payment address
- **Additional Information** — how heard about program, prior work, other funding

**Content notes:**
- Applications must NOT mention tokenomics, deployment/hosting costs, business activities, events
- At least 50% of payment in DOT (vested over 2 years), remainder in USDC on AssetHub
- PR should only contain one new file

**Review Process (historical):**
- Committee reviews PR, may request changes
- Needs 1/3 committee approvals
- Milestone delivery via separate repo with invoice
- Evaluators approve each milestone

---

## 2. Arbitrum Foundation / Arbitrum DAO Grants

**Portal:** `https://arbitrum.questbook.app/`  
**Platform:** Questbook (DDA — Domain Allocator Offerings)  
**Program:** Arbitrum DDA 3.0 (current/main program)

### Program Overview
- **Total Budget:** $6.75M (one-year program)
- **Managed by:** Questbook, with domain allocators elected by the DAO
- **Payment:** Milestone-based, in ARB tokens
- **KYC Required:** Yes (acknowledged in application)

### Five Grant Domains (DDA 3.0)
| Domain | Budget | Focus Areas |
|---|---|---|
| **New Protocols and Ideas** | $1.5M | General bucket: protocols, platforms, governance tooling |
| **Dev Tooling on Arbitrum One and Stylus** | $1.5M | Developer tools, Arbitrum One/Stylus adoption |
| **Gaming** | $1.5M | Web3 gaming infrastructure, gaming KOL activities |
| **Education, Community Growth & Events** | $1.5M | Physical events, educational materials |
| **Orbit Chains** | $750K | Orbit chain expansion, UX solutions |

**Domain Allocators (Season 3):**
- Program Manager: JoJo
- New Protocol Ideas: CastleCapital
- Gaming: Flook
- Dev Tooling: Juandi
- Education/Community/Events: SEEDGOV
- Orbit: MaxLomu

### Application Form Required Fields (Questbook)
- **Applicant Information:** Name, Email, RSS metrics (Telegram, Twitter, Discord, Website, LinkedIn, Instagram, Others)
- **KYC Acknowledgment:** Yes/No
- **Report Acknowledgment:** Yes/No (completion report + 3-month survey)
- **Wallet Address:** ARB1 chain address for grant receipt
- **Project Title** (max 80 chars)
- **Project Details:** Description, innovation/value, target audience, team experience, comparable projects, current stage
- **Past Grants:** Any Arbitrum ecosystem grants received? Any non-Arbitrum grants received?
- **Grant Request Details:** Project idea, deliverables outline, alignment with Arbitrum ecosystem goals
- **Requested Grant Amount**
- **Budget Breakdown:** Detailed cost utilization template
- **Milestones:** List with USD amounts, deliverables, estimated completion time
- **Milestone KPIs:** Reference KPIs for each milestone
- **Project Duration:** Estimated max time for completion
- **Success Metrics:** How community measures success
- **Economic Plan:** Post-grant sustainability/maintenance
- **Category:** DeSci, AI, DeFi, NFT, Educational Material, Analytics, Privacy, Consumer App, Developer Tooling, Others
- **Protocol Performance/Audit** (select if applicable)

**Submission:** Online via Questbook portal — `https://arbitrum.questbook.app/proposal_form?chainId=10&grantId=...`  
Select the appropriate domain/grant program when applying.

**Review Process:**
- Domain allocators evaluate proposals using rubrics
- Community can view decisions and reasoning
- Milestone-based disbursement
- Regular check-ins required

---

## 3. Optimism Grant Council

**Portal:** `https://app.opgrants.io/` (Karma-powered)  
**Governance Forum:** `https://gov.optimism.io/c/grants/87`  
**Charmverse Space:** `https://app.charmverse.io/op-grants/optimism-grants-council-8323028890716944`  
**Application Platform:** Charmverse + OP Atlas (`https://atlas.optimism.io/`)

### Current Status (Season 9)
- **Applications Open:** Feb 11, 2026
- **Submissions Close:** May 20, 2026
- **Review Timeline:** ~15 business days (5-day intake + 1-day buffer + 1-day ops sync)
- **Payment:** OP tokens (lock-up rules apply — grants locked for 1 year)

### Tracks / Funding Types
1. **Grants Council Mission Requests** — Builder grants for specific ecosystem needs
   - **Builder Grants** (small): Up to 2,500 OP for experimental projects (continuous microgrant round)
   - **Growth Grants**: Up to 3.89M OP in Season 9 (targets DEX TVL, fees in priority pairs)
   - **Mission Requests**: Various tiers (Ember, Bronze, Silver, Gold) with baseline OP amounts
2. **Foundation Mission Requests** — Created by Optimism Foundation on rolling basis
3. **Retro Funding** (RetroPGF) — Retroactive public goods funding (application via `retrofunding.optimism.io`)
4. **Audit Grants** — Subsidized smart contract audits

### Season 9 Priority Metrics
For Grants Council applications:
- DEX TVL in Priority Pairs (Liquidity)
- Fees from swaps in priority pairs (`ΔPairFees = PairFees_end − PairFees_start`)

**Eligible Superchain chains:** All green/orange chains in the Superchain Registry (OP Mainnet, Base, Frax, Ink, Lisk, Metal, Mode, Polynomial, Shape, soneium, Superseed, Unichain, Zora, etc.)

### Application Form Fields (Charmverse-based)
- **Project Name**
- **Grant Size Request** (max 250k OP; above 150k requires full Council vote)
- **Justification for grant size**
- **Roadmap and Distribution Plan** — discrete steps
- **OP Distribution Plan** — % allocation to different initiatives, distribution timeline
- **No-sale rule acknowledgment** (absolute requirement)
- **Post-grant rationale** — if tokens for direct user distribution, why incentivized users remain after incentives dry up
- **Metrics/KPIs** — how success is measured
- **Team information** and relevant experience
- **Past grants/funding received**

### Application Process Flow
1. **Submission (Week 0):** Submit application on Charmverse
2. **Initial Review (Week 1):** GrantNerds filter submissions (spam filter)
3. **Final Review (Weeks 2-3):** Council reviewers assess alignment, feasibility, completeness
4. **Decision:** Approved projects receive OP rewards; milestones and metrics validated before disbursement
5. **Reporting:** Ongoing milestone tracking required

**Key links:**
- Mission Requests: `https://github.com/ethereum-optimism/OPerating-manual`
- Application portal: `https://app.charmverse.io/op-grants/` (invite link required)
- Calendar: Public governance calendar on Optimism forum
- Office Hours: Bi-weekly on Tuesdays at 2:00 PM UTC

---

## 4. Autonomys Network (Subspace Foundation)

**Portal:** `https://subspace.foundation/grants`  
**Application Form:** `https://forms.gle/vqPnqm1vQdAWtW3T7` (Google Forms)  
**Developer Hub:** `https://develop.autonomys.xyz/`

### Program Overview
- **Funding:** Up to 10% of total $AI3 token supply allocated to ecosystem development
- **Payment Methods:** Gas credits, $AI3 tokens, stablecoins, or USD
- **Focus:** Decentralized AI (deAI) / AI3.0 on the Autonomys Network
- **Managed by:** Subspace Foundation (Zug, Switzerland)

### Five Grant Categories
| # | Category | Focus |
|---|---|---|
| 1 | **Infrastructure Grants** | Distributed storage (DSN), modular blockchain architecture, PoAS consensus, AI-optimized compute |
| 2 | **AI-Powered dApp Grants** | Super dApps, on-chain agents, AI-driven tools (finance, healthcare, logistics) |
| 3 | **Research Grants** | Novel consensus, scalability, eco-friendly alternatives, privacy-preserving AI, governance |
| 4 | **Community Grants** | Education, developer bootcamps, documentation, content, onboarding |
| 5 | **Integration Grants** | Bridges, relayers, oracles, SDKs, cross-chain connectivity |

### Application Form Required Fields
- **Contact Information:** Name, email, organization, preferred communication channels
- **Project Overview:** Goals, target audience, alignment with Subspace mission
- **Category:** Select from the 5 above
- **Key Milestones:** Expected deliverables and estimated timelines
- **Funding Breakdown:** Estimated funding required per milestone

### Selection Criteria
- **Relevance** — Alignment with Subspace Foundation mission and grant objectives
- **Feasibility** — Technical and operational viability
- **Impact** — Potential benefits to the AI3.0 ecosystem
- **Team Expertise** — Qualifications and past experience

### Application Process (5 Steps)
1. **Prepare** — Review program docs, sample Grant Agreement, Terms & Conditions
2. **Apply** — Submit application via Google Form (link above)
3. **Discovery** — Initial screening (up to 8 weeks); selected applicants invited to virtual deep-dive meeting; may complete detailed technical form
4. **Funding** — Decision within 2-4 weeks after Discovery; milestone-based funding; sign Grant Agreement
5. **Create & Share** — Deliver milestones, submit reports, showcase results

**Documents to review before applying:**
- Sample Grant Agreement: `https://drive.google.com/file/d/1eNAvDF_nIJ4Ijtk1KT-ulyW1hsfJmqH9/view`
- Terms & Conditions: `https://drive.google.com/file/d/1XCaa65rbzDRj7ArzjsOkVz9i2Vu3ttW0/view`
- Example Projects: `https://subspace.foundation/grants/example-projects`

### Related Program: Autonomys x WeatherXM Builders Program
- **Up to $10,000** in AI3 storage credits
- Focus: Applications using WeatherXM data + Autonomys DSN
- **Not a direct grants application** — this is a co-development program with specific requirements (must use both WeatherXM data and Autonomys storage)

**Apply at:** `https://www.autonomys.xyz/builders` (WeatherXM joint program) or `https://forms.gle/vqPnqm1vQdAWtW3T7` (general grants)

---

## 5. Ethereum ESP (Ethereum Support Program)

**Portal:** `https://esp.ethereum.foundation/`  
**Application Form:** `https://esp.ethereum.foundation/form-direct/apply`  
**Small Grants:** `https://esp.ethereum.foundation/applicants/small-grants`

### Program Overview
- **Payment:** ETH by default (paid on-chain)
- **Focus:** Free, open-source, non-commercial projects that benefit Ethereum
- **Target:** Builders (not end-users) — infrastructure, tools, research, community resources, public goods
- **Managed by:** Ethereum Foundation Ecosystem Support Program

### Application Types
1. **Direct Grant Application** — For general projects
2. **RFP-based Application** — For specific Wishlist/RFP items
3. **Small Grants** (up to $30,000) — Seed grants for smaller scope projects
4. **Academic Grants Round** — Research grants (separate process, deadlines)

### Direct Grant Application Required Fields

**Contact Information:**
- First name, Last name, Email, Company/Organization (or "N/A")
- Profile Type (Individual/Team)
- Alternative contact: Website, City, Country, Time Zone
- Applicant Profile (biography including relevant experience)

**Budget:**
- Budget Request amount + Currency

**Project Overview:**
- Project Name (concise title)
- Project Summary (brief description, what's being built and why, links to existing work)
- Project Repo Link (GitHub/GitLab/HackMD)
- Domain (select from dropdown)
- Output Type (select from dropdown)

**Project Details:**
- Project Structure (detailed breakdown of scope, timeline, milestones, deliverables)
- Sustainability Plan (post-grant sustainability, both financial and non-financial)
- Funding (other parties involved? prior funding discussions?)
- Problem Being Solved (problem, who's affected, concrete examples)
- Measured Impact (current ecosystem metrics — users, page visits, code contributors)
- Success Metrics (quantifiable measurements post-completion)
- Ecosystem Fit (compare to 2-3 similar projects; how is yours unique?)
- Community Feedback (domain expert or community feedback received)
- Open Source License (select from dropdown)

**Additional Details:**
- Previous EF grant applications? (Yes/No)
- Internal EF Contact (name of team member who directed application)
- Additional questions/comments (optional)
- PDF Proposal (optional attachment for extra details)
- Grant Payment Acknowledgement (required checkbox)

### Small Grants (up to $30,000)
- **Process:** Online form → ESP team review → Interview → Decision
- **Evaluation:** Every submission read by ESP team; may include interview, budget negotiation, rescoping
- **Onboarding:** All recipients complete KYC and sign legal grant agreement
- **Payment:** Milestone-based, in ETH

### General Application Process
1. **Browse** — Find Wishlist or RFP items matching your interests
2. **Apply** — Submit application detailing methodology, timeline, deliverables
3. **Review** — GM team reviews with relevant EF team; may include interview, rescoping, budget negotiation
4. **Decision** — Notification by email; if selected, work with ESP to establish grant structure
5. **Execute** — Begin work with Grant Evaluator for check-ins and milestone reviews
6. **Complete** — Share results publicly in a report

### Selection Criteria
- **Technical approach** — Soundness, feasibility, clarity
- **Ecosystem impact** — Potential benefit to Ethereum community
- **Open source** — All outputs must be open-source/freely available
- **Alignment** — Understanding of Ethereum's values and ecosystem needs

**Key Links:**
- Office Hours (for guidance before applying)
- ESP Grant Explorer (1,000+ funded projects)
- Wishlist and RFPs: `https://esp.ethereum.foundation/applicants/rfp`

---

## Summary Table

| Program | Submission URL | Mechanism | KYC Required | Payment | Status |
|---|---|---|---|---|---|
| **W3F Grants-Program** | `github.com/w3f/Grants-Program` | GitHub PR | Yes | DOT (50%+) + USDC | **Discontinued** |
| **Arbitrum DAO** | `arbitrum.questbook.app` | Questbook web form | Yes | ARB (milestone-based) | Active (DDA 3.0 domains) |
| **Optimism Grants** | `app.charmverse.io/op-grants` | Charmverse form | Required for >$150k OP | OP tokens (1yr lock) | Season 9 (closes May 20, 2026) |
| **Autonomys** | `forms.gle/vqPnqm1vQdAWtW3T7` | Google Form | Implied (Discovery phase) | $AI3, stablecoins, USD, gas credits | Active (rolling review) |
| **Ethereum ESP** | `esp.ethereum.foundation/form-direct/apply` | Web form + optional PDF | Yes (onboarding) | ETH by default | Active (continuous) |

## Notes for PreTxSim Applications

**PreTxSim** is an air-gapped local EVM simulation proxy built in Rust on revm. Key considerations:

1. **W3F Grants-Program** is currently **discontinued** — do not submit. Monitor for reopening or apply to the Polkadot Treasury directly.

2. **Arbitrum** — PreTxSim could fit under "Dev Tooling on Arbitrum One and Stylus" since it's a developer tool for transaction simulation. The project is EVM-compatible and could integrate with Arbitrum's ecosystem. Apply via Questbook when a domain is Open.

3. **Optimism** — PreTxSim aligns with "Developer Tooling" and could target the "DEX TVL" or "fees" metrics if it helps optimize transaction flow. Season 9 applications close May 20, 2026. Apply via Charmverse.

4. **Autonomys** — The project could fit under "Infrastructure Grants" (developer tools) or "Integration Grants" if it connects to Autonomys' Auto EVM. The application is straightforward via Google Form.

5. **Ethereum ESP** — PreTxSim is a strong fit — it's an open-source developer tool built on EVM-compatible tech (revm). The Direct Grant Application or Small Grants (up to $30k) would be appropriate. Payment in ETH by default.
