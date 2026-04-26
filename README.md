# Freelance Escrow

A trustless, low-cost escrow system for freelancers and clients, built as a Soroban smart contract on Stellar.

---

## Problem

Freelancers and small agencies working with international clients face high financial risk and friction. Upfront payment disputes, delayed bank transfers, and expensive third-party escrow services mean freelancers often perform work without guaranteed payment, while clients lack trust that deliverables will be met. Traditional methods take days to settle, lock up working capital, and charge high fees—making them impractical for project-based, gig-economy work.

## Solution

Freelance Escrow utilizes Stellar and Soroban smart contracts to act as a transparent, non-custodial escrow. 

Clients lock funds (such as USDC) into an on-chain contract. The contract strictly enforces the agreement: funds are released to the freelancer upon client approval, or automatically refunded to the appropriate party if deadlines pass. Settlement happens in 3–5 seconds with transaction fees under $0.01, relying entirely on programmable on-chain logic rather than intermediaries.

---

## Demo Flow (CLI)

1. **Create Escrow:** The client locks tokens against the contract, specifying the freelancer's address and a UNIX deadline.
2. **Approve Work:** Once deliverables are met, the client invokes `approve_work` to release funds to the freelancer.
3. **Claim Expired:** If the client disappears or stalls, the freelancer invokes `claim_expired` after the deadline to reclaim the locked funds.

---

## Architecture
Stellar Testnet └── FreelanceEscrow Soroban Contract (contracts/src/lib.rs) └── Escrow Logic (lock, release, expiry, read-only state)

code

Copy

No frontend and no backend server. All escrow state, balances, and lifecycle rules live immutably on-chain via the Soroban contract.

---

## Stellar Features Used

| Feature            | Usage                                                     |
|--------------------|-----------------------------------------------------------|
| **Soroban Smart Contracts** | Core escrow logic, time-locks, and state management |
| **Stellar Assets / Tokens** | Settling payments (USDC, XLM, or custom tokens) |
| **Trustlines**     | Recipient must establish trust for the specific token used |
| **Low Fees & Speed** | Sub-cent transaction costs and 3-5 second finality |

---

## Smart Contract

**Contract ID (Testnet):**
CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X

code

Copy

**Stellar Lab Explorer:**  
https://lab.stellar.org/testnet/contract/CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X

### Contract Functions

| Function          | Caller   | Description                                                              |
|-------------------|----------|--------------------------------------------------------------------------|
| `create_escrow(...)` | Client   | Locks funds against a specific freelancer and a UNIX deadline            |
| `approve_work(...)`  | Client   | Releases funds to the freelancer immediately                             |
| `claim_expired(...)` | Freelancer | Claims locked funds after the deadline has passed                      |
| `get_escrow(...)`    | Anyone   | Read-only view of the current escrow state                              |

### Escrow Lifecycle
Created --> Approved (Funds successfully sent to Freelancer) --> Expired (Freelancer claims funds after deadline passes)

code

Copy

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
cd contracts

# Build the WASM target
soroban contract build

# Run local Rust unit tests
cargo test
2. Deploy to Testnet
bash

Copy
# Generate a persistent identity for deployment
soroban keys generate --global deployer --network testnet

# Deploy to Stellar Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/freelance_escrow.wasm \
  --source deployer \
  --network testnet
(Note: The output of this command will be your Contract ID. Ensure it matches the one documented above, or update the documentation with the new ID.)

CLI Invocation Examples
Interact with the deployed contract directly using the Soroban CLI:

bash

Copy
# 1. Create an escrow (Locking 100 USDC for 1 hour)
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- create_escrow \
  --client <CLIENT_ADDRESS> \
  --freelancer <FREELANCER_ADDRESS> \
  --token <USDC_CONTRACT_ADDRESS> \
  --amount 100000000 \
  --deadline $(date -d "+1 hour" +%s)

# 2. Approve work (Release funds to freelancer)
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source client \
  --network testnet \
  -- approve_work \
  --escrow_id <ESCROW_ID>

# 3. Claim expired funds (Freelancer claims after deadline)
soroban contract invoke \
  --id CBPXD4WLBHOQAX3YRI3Y55LE57ERDT57SLSBP32VFEQNL66PN7MAT26X \
  --source freelancer \
  --network testnet \
  -- claim_expired \
  --escrow_id <ESCROW_ID>
Target Users
Independent freelancers, remote contractors, and small agencies who require a fast, cheap, and trustless way to secure milestone payments without relying on expensive centralized escrow platforms.

Why Stellar
No other chain provides sub-cent fees, 3-5 second finality, and native stablecoin support out-of-the-box. By building on Soroban, Freelance Escrow enforces contractual agreements immutably at the ledger level, ensuring fairness for both clients and freelancers without intermediaries or gas fee volatility.

