# Freelance Escrow

> A decentralized freelance payment escrow system built on Stellar using Soroban smart contracts — protecting freelancers in Southeast Asia from payment fraud.

---

## The Problem

Freelancers in Southeast Asia — especially in the Philippines, Indonesia, and Vietnam — routinely face:

- Clients who disappear after receiving completed work
- Delayed or withheld payments with no recourse
- No trusted infrastructure to enforce payment agreements

This creates unstable income and erodes trust in remote work for millions of freelancers.

---

## The Solution

**Freelance Escrow** uses Soroban smart contracts on Stellar to record job agreements and track payment state on-chain. No middleman. No trust required. Just code.

**How it works:**

1. Client creates a job — the agreement and payment amount are recorded on-chain
2. Freelancer completes the work
3. Client approves the output
4. Contract status updates to completed and total is incremented on-chain
5. The entire flow is transparent and immutable on Stellar

> **MVP note:** This version simulates payment tracking. Real XLM transfers are planned for a future release (see Roadmap).

---

## Demo Flow (under 2 minutes)

```
Client creates job → agreement recorded on-chain
→ Freelancer completes work
→ Client approves payment
→ Status marked complete, total updated on-chain
```

**Real-world example:**
A startup founder in Manila hires a freelance designer. The client creates a job for 100 XLM. The freelancer delivers the logo. The client approves — status is updated instantly, transaction recorded on-chain.

---

## Smart Contract Functions

### `create_job`
Records a freelance agreement between a client and freelancer on-chain, storing:
- Client address
- Freelancer address
- Payment amount (as `u32`)

Status is initialized to `0` (pending). Note: no XLM is transferred at this stage in the current MVP.

### `approve_and_release`
Allows **only the client** to approve completed work and mark the job as released. Prevents:
- Unauthorized approvals (non-client callers are rejected)
- Duplicate releases (status `1` blocks re-execution)

On success: sets status to `1` and increments the running total by the job amount.

### `get_status`
Returns the current payment status as a `u32`:
- `0` — pending
- `1` — completed

### `get_total`
Returns the cumulative total of all released payment amounts (as `u32`).

### `get_job`
Returns the stored job details as a tuple: `(client: Address, freelancer: Address, amount: u32)`.

---

## Project Structure

```
.
├── contracts/
│   └── freelance_escrow/
│       ├── src/
│       │   ├── lib.rs       # Core contract logic
│       │   └── test.rs      # Test suite
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

---

## Test Coverage

| Test | What it verifies |
|------|-----------------|
| `test_create_job` | Client can create a job; `create_job` returns the correct amount |
| `test_release_payment` | Client can approve and release; `approve_and_release` returns the correct amount |
| `test_status_update` | Status changes to `1` after `approve_and_release` is called |

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Smart contracts | Rust + Soroban SDK |
| Blockchain | Stellar (Testnet) |
| Contract runtime | Soroban |

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- Stellar CLI tools

### Build

```bash
soroban contract build
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/freelance_escrow.wasm \
  --source YOUR_ACCOUNT \
  --network testnet
```

---

## Current Limitations (MVP)

This MVP is scoped to demonstrate core escrow logic:

- Supports one active job at a time
- Payment tracking is simulated — no real XLM transfers occur
- `approve_and_release` does not yet transfer tokens to the freelancer address

---

## Roadmap

- [ ] Real XLM transfers on Stellar Mainnet
- [ ] Multi-job support
- [ ] Milestone-based payments
- [ ] Dispute resolution system
- [ ] Frontend dApp interface

---

## Vision

Build trusted freelance payment infrastructure for global remote workers — starting with Southeast Asia — using the speed and low cost of Stellar blockchain.

---

## License

MIT License