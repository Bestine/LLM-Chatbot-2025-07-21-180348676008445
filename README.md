# 🦀 crowdfund-rustchain

A decentralized crowdfunding dApp built with Rust. This project is designed to help you learn Rust and Web3 by building something meaningful — a trustless, censorship-resistant fundraising platform.

---

## 🚀 Overview

This project walks through the creation of a crowdfunding smart contract using Rust, deployed either on **Solana (Anchor)** or **Internet Computer (ICP)** via `icp.ninja`.

---

## 📦 Suggested Stack

- **Rust** (Core language)
- **Solana + Anchor** *(or ICP with Rust canisters)*
- **Phantom Wallet** for frontend interaction
- **Optional**: React / Svelte frontend

---

## 🧹 Project Breakdown

### 🔹 Phase 1: Setup & Rust Basics (Day 1–2)

- Install Rust and Cargo
- Learn:
  - Ownership & Borrowing
  - Structs and Enums
  - Pattern Matching
  - `Result` and `Option`
- Build basic CLI tools with `cargo`

---

### 🔹 Phase 2: Blockchain Setup (Day 3)

- Choose a blockchain:
  - ✅ **Solana with Anchor**
  - ⛓️ OR: **Internet Computer (ICP)**
- Install `solana-cli` and `anchor`
- Initialize project:
  ```bash
  anchor init crowdfund-rustchain
  ```

---

### 🔹 Phase 3: Smart Contract Development (Days 4–7)

#### Core Contract Features:

- ✅ Create a campaign
- 💰 Donate to campaign
- 💸 Withdraw by creator
- ❌ Refund if goal not met

#### Rust Concepts:

- Structs and Traits
- Pattern matching
- PDA (Program Derived Addresses)
- Error handling (`Result`, custom errors)

---

### 🔹 Phase 4: Testing & Simulation (Day 8–9)

- Write unit tests with `#[test]`
- Use `anchor test` on local validator
- Test edge cases:
  - Refund logic
  - Deadline expiry
  - Access control

---

### 🔹 Phase 5: Frontend Integration (Days 10–13)

- Build frontend with React or Svelte
- Connect Phantom wallet
- Call contract methods via Solana JS SDK
- Display campaign status and history

---

### 🔹 Phase 6: Deployment (Day 14)

#### Option 1: Solana

```bash
anchor build
anchor deploy --provider.cluster devnet
```

#### Option 2: Deploy to ICP with Rust Canisters

1. Install [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/build/install/)
2. Initialize project:
   ```bash
   dfx new crowdfund-rustchain --type rust
   cd crowdfund-rustchain
   ```
3. Write crowdfunding logic in `src/lib.rs`
4. Deploy locally:
   ```bash
   dfx start
   dfx deploy
   ```
5. Deploy to Internet Computer:
   ```bash
   dfx deploy --network ic
   ```

---

## 🗓️ Timeline Summary

| Phase                | Duration | Key Outcome                        |
| -------------------- | -------- | ---------------------------------- |
| Rust Basics          | 2 days   | Rust fundamentals & project setup  |
| Blockchain Setup     | 1 day    | Anchor or ICP installed & ready    |
| Smart Contract Dev   | 4 days   | Contract for campaign logic        |
| Testing              | 2 days   | Working unit tests and simulations |
| Frontend Integration | 3 days   | UI integration with wallet support |
| Deployment           | 1 day    | Hosted on devnet or ICP            |

**Total**: \~14 days

---

## 🧠 What You'll Learn

- Rust syntax, ownership, and memory safety
- Web3 smart contract development
- Solana's account-based programming model (or ICP’s canister model)
- How to write, test, and deploy dApps
- How to build real-world decentralized applications

---

## 💼 Project Name Ideas

- `crowdfund-rustchain`
- `ruststarter`
- `decentrafund`
- `anchorfund`

---

## ✨ Next Steps

If you'd like help scaffolding the project (especially for Anchor), feel free to open an issue or reach out!

---

Happy building! 🚀🦀

