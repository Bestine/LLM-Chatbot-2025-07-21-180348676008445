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

# Guide from DFX Set up - Automated README
## `crowdfund-rustchain`

Welcome to your new `crowdfund-rustchain` project and to the Internet Computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `crowdfund-rustchain`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd crowdfund-rustchain/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor

