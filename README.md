# QuickPay Counter

A simple job payment escrow system built as a Soroban smart contract on Stellar.

---

## What Is QuickPay Counter?

QuickPay Counter is a **non-custodial, blockchain-based job payment system** that enables clients and freelancers to execute secure, transparent payments on the Stellar network. Using Soroban smart contracts, it eliminates intermediaries and ensures that payments are only released when both parties confirm the transaction on-chain.

---

## Problem

Freelancers and clients face several challenges with traditional payment systems:
- **Lack of transparency:** Payment status is opaque and centralized
- **High fees:** Third-party payment processors take significant cuts
- **Trust issues:** Clients and freelancers must trust intermediaries with their funds
- **Settlement delays:** Payments can take days to clear
- **No immutable record:** Payment history cannot be cryptographically verified

## Solution

QuickPay Counter utilizes **Stellar and Soroban smart contracts** to provide a transparent, non-custodial job payment system where:
- ✅ Clients create jobs with specific amounts for freelancers
- ✅ All transactions are recorded immutably on-chain
- ✅ The client approves and releases payment to the freelancer explicitly
- ✅ The contract enforces the agreement automatically with zero intermediaries
- ✅ All payment history is publicly verifiable on the blockchain

---

## How It Works

### 1. Job Creation
A client creates a job record on-chain, specifying:
- Freelancer's Stellar address
- Payment amount
- Job details (stored in contract state)

### 2. Work Completion
The freelancer completes the work. Both parties can verify the job details on-chain at any time.

### 3. Approval & Payment Release
Once satisfied, the client invokes the `approve_and_release` function, which:
- Marks the job as released in the contract
- Records the payment as complete on-chain
- Provides cryptographic proof of payment

### 4. Verification
Anyone can query the contract to verify:
- Job status (pending or released)
- Total amount released
- Complete job details (client, freelancer, amount)

All records are **immutable and permanent** on the Stellar blockchain.

---

## Demo Flow (CLI)

```
1. Create Job      → Client submits job with freelancer address and amount
2. Complete Work   → Freelancer completes the work
3. Approve & Release → Client calls approve_and_release to release payment
4. Track Payment   → View job details and payment status on-chain
```

---

## Architecture

```
Stellar Testnet
└── QuickPay Counter Soroban Contract (contracts/freelance_escrow/src/lib.rs)
    └── Job Management Logic (create, approve, status tracking)
```

**No frontend. No backend server. All job data, payments, and status live immutably on-chain via the Soroban contract.**

---

## Smart Contract

Deployed on **Stellar Testnet**:

```
CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X
```

**View on Stellar Lab:**
https://lab.stellar.org/r/testnet/contract/CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X

**View on Stellar Expert:**
https://stellar.expert/explorer/testnet/contract/CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X

### Contract Functions

| Function | Caller | Description |
|----------|--------|-------------|
| `create_job(...)` | Client | Creates a new job with client, freelancer, and amount |
| `approve_and_release(...)` | Client | Approves work and marks payment as released |
| `get_status()` | Anyone | Read-only view of current job payment status (0=pending, 1=released) |
| `get_total()` | Anyone | Read-only view of total amount released |
| `get_job()` | Anyone | Read-only view of job details (client, freelancer, amount) |

### Job Lifecycle

```
Pending (status = 0) → Released (status = 1)
```

---

## Prerequisites

- Rust (latest stable)
- Soroban CLI v25+
- A Stellar Testnet account funded via [Friendbot](https://developers.stellar.org/docs/build/guides/testnet#use-testnet-with-soroban)

---

## Build & Deployment

### 1. Build the Contract

```bash
# Navigate to the contract directory
cd contracts/freelance_escrow

# Build the WASM target
soroban contract build

# Run local Rust unit tests
cargo test
```

### 2. Deploy to Testnet

```bash
# Generate a persistent identity for deployment
soroban keys generate --global deployer --network testnet

# Deploy to Stellar Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/freelance_escrow.wasm \
  --source deployer \
  --network testnet
```

The output of this command will be your Contract ID. Use it in the examples below.

---

## CLI Invocation Examples

Interact with the deployed contract directly using the Soroban CLI:

```bash
# 1. Create a job (100 units from client to freelancer)
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- create_job \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --amount 100

# 2. Approve work (Release payment to freelancer)
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- approve_and_release \
  --caller <CLIENT_ADDRESS>

# 3. Check payment status
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- get_status

# 4. View total released amount
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- get_total

# 5. View job details
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- get_job
```

---

## Target Users

Freelancers and clients who need a simple, trustless way to manage milestone payments on-chain with transparent approval workflows and cryptographic proof of payment.

---

## Why Stellar

Stellar provides:
- ✅ **Sub-cent fees** (~0.00001 XLM per operation)
- ✅ **Fast finality** (3-5 second transaction confirmation)
- ✅ **Native smart contract support** via Soroban
- ✅ **Built-in compliance** (anchors, KYC-ready infrastructure)
- ✅ **USDC support** for stablecoin payments

This makes Stellar ideal for lightweight, cost-effective payment applications where transparency and immutability matter.

---

## Project Repository

https://github.com/reymarkjpanes/quickpay_counter

**Language Composition:** 92% Rust, 8% Makefile
