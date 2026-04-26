# QuickPay Counter

A simple job payment escrow system built as a Soroban smart contract on Stellar.

---

## Problem

Freelancers and clients need a straightforward way to manage job payments on-chain with clear approval workflows, without the complexity of traditional payment systems.

## Solution

QuickPay Counter utilizes Stellar and Soroban smart contracts to provide a transparent, non-custodial job payment system.

Clients create jobs with specific amounts for freelancers. The client can then approve and release payment to the freelancer. The contract tracks all payments and job statuses immutably on-chain.

---

## Demo Flow (CLI)

1. **Create Job:** The client creates a job for a freelancer with a specified amount.
2. **Approve & Release:** Once the work is complete, the client invokes `approve_and_release` to confirm and release the payment.
3. **Track Payment:** View job details and payment totals at any time using read-only contract functions.

---

## Architecture

```
Stellar Testnet
└── QuickPay Counter Soroban Contract (contracts/freelance_escrow/src/lib.rs)
    └── Job Management Logic (create, approve, status tracking)
```

No frontend and no backend server. All job data, payments, and status live immutably on-chain via the Soroban contract.

---

## Smart Contract

The main contract is located in `contracts/freelance_escrow/src/lib.rs`.

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
- A Stellar Testnet account funded via Friendbot

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

## CLI Invocation Examples

Interact with the deployed contract directly using the Soroban CLI:

```bash
# 1. Create a job (100 units from client to freelancer)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- create_job \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --amount 100

# 2. Approve work (Release payment to freelancer)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- approve_and_release \
  --caller <CLIENT_ADDRESS>

# 3. Check payment status
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- get_status

# 4. View total released amount
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- get_total

# 5. View job details
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source client \
  --network testnet \
  -- get_job
```

## Target Users

Freelancers and clients who need a simple, trustless way to manage milestone payments on-chain with transparent approval workflows.

## Why Stellar

Stellar provides sub-cent fees, fast finality, and native smart contract support through Soroban, making it ideal for lightweight payment applications.
