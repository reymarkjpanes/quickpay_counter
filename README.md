# FreelanceEscrow

A trustless freelance payment escrow built on Stellar using Soroban smart contracts. Clients lock payment on-chain and release it only after confirming work is delivered — no middlemen, no wire delays, no disputes over "did you pay yet?"

---

## Problem

Filipino freelancers — designers, developers, virtual assistants — routinely work with overseas clients who pay late, underpay, or ghost entirely. Traditional escrow services charge 5–10% and take days to process. Bank wires and PayPal hold funds arbitrarily. There is no lightweight, trustless option that works at the scale of a ₱5,000 logo project or a ₱50,000 web build.

## Solution

FreelanceEscrow lets a client lock payment in a Soroban smart contract the moment they hire a freelancer. The funds are verifiably on-chain — the freelancer can see them before lifting a finger. When the client confirms delivery, the contract releases payment instantly. Neither party can manipulate the outcome: the contract enforces the rules.

Settlement on Stellar takes under 5 seconds with fees under $0.01.

---

## How It Works

```
Client creates a job  →  Amount locked on-chain
Freelancer delivers work
Client calls approve_and_release  →  Payment released to freelancer
```

**Status lifecycle:**

```
0 (Pending)  →  1 (Released)
```

Only the client who created the job can call `approve_and_release`. Double-release is blocked at the contract level.

---

## Smart Contract

**Deployed on Stellar Testnet:**

```
CONTRACT_ID_HERE
```

> Replace `CONTRACT_ID_HERE` with your deployed contract address.

Explorer: `https://stellar.expert/explorer/testnet/contract/CONTRACT_ID_HERE`

> Add a screenshot of the deployed contract on Stellar Expert or Stellar Lab here.

---

## Contract Functions

| Function | Caller | Description |
|---|---|---|
| `create_job(client, freelancer, amount)` | Client | Registers a job and locks the agreed amount |
| `approve_and_release(caller)` | Client only | Confirms delivery and releases payment to freelancer |
| `get_status()` | Anyone | Returns current status: `0` = Pending, `1` = Released |
| `get_total()` | Anyone | Returns cumulative total of all released payments |
| `get_job()` | Anyone | Returns `(client_address, freelancer_address, amount)` |

---

## Project Structure

```
freelance_escrow/
├── src/
│   ├── lib.rs       # Soroban escrow contract logic
│   └── test.rs      # Unit tests (create, release, status)
└── Cargo.toml
```

---

## Tests

Three unit tests cover the core flows:

| Test | What it checks |
|---|---|
| `test_create_job` | Job is created and returns the correct amount |
| `test_release_payment` | `approve_and_release` returns the correct amount |
| `test_status_update` | Status flips to `1` after release |

Run the tests:

```bash
cargo test
```

---

## Prerequisites

- Rust (latest stable)
- Soroban CLI v25+
- A funded Stellar testnet account ([Friendbot](https://friendbot.stellar.org))

---

## Setup & Deployment

### Build

```bash
stellar contract build
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
# Generate a deployer key
soroban keys generate --global deployer --network testnet

# Deploy the contract
soroban contract deploy \
  --wasm target/wasm32v1-none/release/freelance_escrow.wasm \
  --source deployer \
  --network testnet
```

---

## Sample CLI Invocations

```bash
# Create a job: client locks 100 units for a freelancer
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- create_job \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --amount 100

# Approve and release payment after delivery
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- approve_and_release \
  --caller <CLIENT_ADDRESS>

# Check escrow status
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_status
```

---

## Why Stellar

- **Sub-cent fees** — a ₱5,000 freelance job doesn't lose ₱500 to transaction costs
- **5-second finality** — payment lands before the client closes their laptop
- **Soroban** — smart contracts with predictable execution and on-chain verifiability
- **No backend needed** — all escrow state lives on-chain; no server to hack or go down

---

## Target Users

Filipino freelancers (developers, designers, VAs, writers) working with overseas clients on projects ranging from ₱5,000 to ₱200,000. They need proof of payment before starting work and their clients need confidence before sending money to someone they've never met in person.