# 🚀 Turbin3 Accelerated Solana Program Assignments (Q2 2026)

Welcome to the central repository for the **Turbin3 Accelerated Solana Program** assignments for **Q2 2026**. This repository contains a collection of highly optimized, advanced Solana programs, submodules, and tools covering framework paradigms, low-level optimization techniques, state automation, and next-generation NFT standards.

---

## 📅 Curriculum & Project Map

| Week | Project Name | GitHub Repository | Local Workspace Path | Key Tech & Concepts |
|:---|:---|:---|:---|:---|
| **Week 1** | Anchor Escrow | [DivineUX23/anchor-escrow-q2-2026](https://github.com/DivineUX23/anchor-escrow-q2-2026) | [/anchor-escrow-q2-2026](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026) | Anchor 1.0.0, Solana Token Interface, 1-byte discriminators |
| | Whitelist Transfer Hook | [DivineUX23/whitelist-transfer-hook-q2](https://github.com/DivineUX23/whitelist-transfer-hook-q2) | [/whitelist-transfer-hook-q2](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2) | SPL Token 2022, Extra Account Metas, dynamic resizing |
| | Anchor Transfer Hook Vault | [DivineUX23/transfer-hook-vault](https://github.com/DivineUX23/transfer-hook-vault) | [/transfer-hook-vault](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/transfer-hook-vault) | Anchor 1.0.0, SPL Token 2022 Transfer Hook, Vault PDA |
| **Week 2** | Magic Block ER Coinflip | [DivineUX23/magicblock-er-example](https://github.com/DivineUX23/magicblock-er-example) | [/magicblock-er-example](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-er-example) | Ephemeral Rollups SDK, Anchor, Surfpool |
| | Magic Block Tuk-tuk Counter | [DivineUX23/magicblock-tuktuk](https://github.com/DivineUX23/magicblock-tuktuk) | [/magicblock-tuktuk](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-tuktuk) | Tuk-tuk scheduler, Ephemeral Rollups, cron delegation |
| | Tuk-tuk GPT Oracle | [DivineUX23/tuktuk-gpt-oracle](https://github.com/DivineUX23/tuktuk-gpt-oracle) | [/tuktuk-gpt-oracle](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/tuktuk-gpt-oracle) | Solana GPT Oracle, CPI integration, Tuk-tuk callback |
| **Week 3** | Rust Serializers | [DivineUX23/turbine-serialize](https://github.com/DivineUX23/turbine-serialize) | [/turbine-serialize](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize) | Borsh, Wincode (bincode v2), JSON, Rust traits |
| **Week 4** | Pinocchio Escrow | [DivineUX23/accel-p-escrow](https://github.com/DivineUX23/accel-p-escrow) | [/accel-p-escrow](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/accel-p-escrow) | Pinocchio framework, standard escrow logic |
| | Pinocchio Fundraiser | [DivineUX23/pinocchio-fundraiser](https://github.com/DivineUX23/pinocchio-fundraiser) | [/pinocchio-fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/pinocchio-fundraiser) | Baseline unoptimized Pinocchio token fundraiser (v1) |
| | Pinocchio Optimized Fundraiser | [DivineUX23/optimized-fundraiser](https://github.com/DivineUX23/optimized-fundraiser) | [/optimized-fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/optimized-fundraiser) | Safe framework-level optimizations (v2), syscall bypass |
| | Zero Framework Optimized Fundraiser | [DivineUX23/asm-optimized-fundraiser](https://github.com/DivineUX23/asm-optimized-fundraiser) | [/asm-optimized-fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/asm-optimized-fundraiser) | Handcrafted BPF input buffer parser (v3), branchless |
| **Week 5** | NFT Core Staking | [DivineUX23/anchor-core-staking](https://github.com/DivineUX23/anchor-core-staking) | [/anchor-core-staking](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking) | Metaplex Core standard, Anchor, freeze staking |

---

## 🛠️ Detailed Breakdown by Week

### 📦 Week 1: Advanced Anchor & Token Extensions (Token-2022)

#### 1. Anchor Escrow ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026) / [GitHub Repository](https://github.com/DivineUX23/anchor-escrow-q2-2026))
An Escrow contract implemented using **Anchor 1.0.0**. It allows a `maker` to initiate an escrow by depositing `amount_a` of `mint_a` into a program-controlled vault in exchange for a specified `amount_b` of `mint_b`. Any `taker` can subsequently execute the escrow atomically, trustlessly exchanging the assets.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026/programs/anchor-escrow-q2-2026/src/lib.rs): Contains instructions using custom 1-byte discriminators for gas-savings.
*   **Key Structures**:
    *   [Escrow](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026/programs/anchor-escrow-q2-2026/src/lib.rs#L15): State account containing escrow configuration.
    *   [Make](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026/programs/anchor-escrow-q2-2026/src/lib.rs#L48): Initialize context for setting up escrow and locking tokens.
    *   [Take](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026/programs/anchor-escrow-q2-2026/src/lib.rs#L152): Accepts the swap conditions and distributes funds.
    *   [Refund](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-q2-2026/programs/anchor-escrow-q2-2026/src/lib.rs#L293): Allows the maker to cancel the open escrow and regain deposited tokens.

#### 2. Whitelist Transfer Hook ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2) / [GitHub Repository](https://github.com/DivineUX23/whitelist-transfer-hook-q2))
Implements a transfer hook using the **SPL Token-2022 Transfer Hook** interface to enforce dynamic whitelist restrictions on token movements. Transfers fail automatically if the source token owner is not included in the whitelist PDA.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2/programs/whitelist-transfer-hook-q2/src/lib.rs)
*   **Key Structures**:
    *   [Whitelist](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2/programs/whitelist-transfer-hook-q2/src/lib.rs#L19): State account holding a dynamic vector of whitelisted public keys.
    *   [InitializeWhitelist](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2/programs/whitelist-transfer-hook-q2/src/lib.rs#L38): Instantiates the whitelist with dynamic reallocation handling.
    *   [InitializeExtraAccountMetaList](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/whitelist-transfer-hook-q2/programs/whitelist-transfer-hook-q2/src/lib.rs#L204): Configures the transfer hook's required metas to feed the whitelist PDA into the token program execution context transparently.

#### 3. Anchor Transfer Hook Vault ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/transfer-hook-vault) / [GitHub Repository](https://github.com/DivineUX23/transfer-hook-vault))
Extends token transfer hook operations to integrate with a custom program-controlled vault.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/transfer-hook-vault/programs/transfer-hook-vault/src/lib.rs)

---

### ⚡ Week 2: Ephemeral Rollups & Automation (Magic Block)

#### 1. Magic Block ER Coinflip ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-er-example) / [GitHub Repository](https://github.com/DivineUX23/magicblock-er-example))
Demonstrates high-performance, low-latency state mutations and randomness (VRF) utilizing **Magic Block Ephemeral Rollups**. State accounts can be delegated to rollups for sub-second confirmations and undelegated back to Solana Mainnet L1.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-er-example/programs/er-state-account/src/lib.rs)

#### 2. Magic Block Tuk-tuk Counter ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-tuktuk) / [GitHub Repository](https://github.com/DivineUX23/magicblock-tuktuk))
Integrates Magic Block's Ephemeral Rollups with the **Tuk-tuk Scheduler** to build automated, scheduled state adjustments (cranks) running natively on Solana.
*   **Key Files**:
    *   [schedule.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-tuktuk/programs/er-state-account/src/instructions/schedule.rs)
*   **Key Structures**:
    *   [Schedule](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/magicblock-tuktuk/programs/er-state-account/src/instructions/schedule.rs#L17): Compiles the target transactions and registers them to the Tuk-tuk scheduler queue.

#### 3. Tuk-tuk GPT Oracle ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/tuktuk-gpt-oracle) / [GitHub Repository](https://github.com/DivineUX23/tuktuk-gpt-oracle))
A decentralized GPT Oracle integration. Invokes LLM context interfaces via CPI to the `solana_gpt_oracle` and registers a state callback function `agent_response` to process GPT outputs (e.g., updating adoption scores).
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/tuktuk-gpt-oracle/programs/tuktuk-gpt-oracle/src/lib.rs)
    *   [agent.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/tuktuk-gpt-oracle/programs/tuktuk-gpt-oracle/src/instructions/agent.rs)
*   **Key Structures**:
    *   [InteractWithLlm](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/tuktuk-gpt-oracle/programs/tuktuk-gpt-oracle/src/instructions/agent.rs#L46): Context to register queries with the on-chain LLM.

---

### ⚙️ Week 3: Rust Serializers

#### 1. Rust Serializers Engine ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize) / [GitHub Repository](https://github.com/DivineUX23/turbine-serialize))
A benchmarking engine comparing serialization protocols in Rust. Defines a common interface and tests the performance/byte density of **Borsh**, **Wincode (bincode v2)**, and **JSON** on a common data structure.
*   **Key Files**:
    *   [main.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize/src/main.rs)
    *   [test.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize/src/test.rs)
*   **Key Structures**:
    *   [Serializer](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize/src/main.rs#L11): Trait defining `to_bytes` and `from_bytes` contracts.
    *   [Storage](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize/src/main.rs#L78): Orchestrates on-the-fly serialization migrations.
    *   [Person](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/turbine-serialize/src/test.rs#L14): Data structure serialized across all test runs.

---

### 🚀 Week 4: Extreme Solana CU (Compute Unit) Optimization

This week features a series of iterations optimizing a token fundraiser program using the lightweight, no-std [Pinocchio](https://github.com/anza-xyz/pinocchio) framework, pushing down execution costs to their theoretical floor.

#### 1. Pinocchio Escrow ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/accel-p-escrow) / [GitHub Repository](https://github.com/DivineUX23/accel-p-escrow))
An implementation of Escrow in Pinocchio. Bypasses Anchor's runtime overhead completely.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/accel-p-escrow/src/lib.rs)

#### 2. Fundraiser Optimization Series
*   **v1 Baseline ([Pinocchio Fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/pinocchio-fundraiser) / [GitHub](https://github.com/DivineUX23/pinocchio-fundraiser))**: Standard Pinocchio deserialization code using `Clock::get()`, `Rent::get()`, and `derive_address`. Used to establish a performance floor.
*   **v2 Optimized ([Pinocchio Optimized Fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/optimized-fundraiser) / [GitHub](https://github.com/DivineUX23/optimized-fundraiser))**: Removes wasted `derive_address` checks (saving ~4,500 CU total across transaction flows), hardcodes rent-exemption formulas, uses unchecked raw pointer reads for u64 fields (1 CU vs 25 CU), utilizes decimal lookup tables, and compiles with optimized release flags (`lto = "fat"`, `panic = "abort"`).
*   **v3 Near-Assembly ([Zero-Framework Optimized Fundraiser](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/asm-optimized-fundraiser) / [GitHub](https://github.com/DivineUX23/asm-optimized-fundraiser))**: Completely replaces the entrypoint and deserializer. Iterates directly over the raw BPF input buffer via pointer arithmetic. Replaces `AccountView` with raw `Account` pointers. Uses custom branchless keys comparisons (14 CU vs 310 CU for `sol_memcmp_`) and branchless ATA validation (28 CU).

#### 📊 Performance Comparison Matrix

| Metric | v1 Baseline | v2 Optimized | v3 Unsafe / Assembly | Optimization Technique (vs v1) |
| :--- | :--- | :--- | :--- | :--- |
| **Binary Size** | 52.44 KB (Fails iCache) | **14.20 KB** | 15.59 KB | Fat LTO, Strip symbols, panic=abort |
| **Initialize CU** | 16,628 | **16,174** | 16,179 | Remove duplicate PDA derivation |
| **Contribute CU** | 3,209 | 2,688 | **2,612** | Raw pointer reads, branchless ATA verification |
| **Refund CU** | 1,870 | 1,582 | **1,493** | Offset-based Token Account reads |
| **Checker CU** | 1,341 | **1,251** | 1,268 | Hardcoded Rent & Clock constants |

---

### 🎨 Week 5: Metaplex Core NFT Staking

#### 1. NFT Core Staking ([Local Workspace](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking) / [GitHub Repository](https://github.com/DivineUX23/anchor-core-staking))
An NFT Staking program utilizing the high-performance **Metaplex Core** standard. Supports collection creation, minting assets directly, staking assets into a secure freeze vault, claiming rewards based on custom BPS formulas, and unstaking.
*   **Key Files**:
    *   [lib.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking/programs/anchor-core-staking/src/lib.rs)
    *   [stake.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking/programs/anchor-core-staking/src/instructions/stake.rs)
    *   [unstake.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking/programs/anchor-core-staking/src/instructions/unstake.rs)
    *   [claim_rewards.rs](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-core-staking/programs/anchor-core-staking/src/instructions/claim_rewards.rs)

---

## ⚡ Testing & Compilation Guide

### Standard Anchor Projects (Weeks 1, 2, 5)
1. Install dependencies:
   ```bash
   yarn install
   ```
2. Build programs:
   ```bash
   anchor build
   ```
3. Run test suites:
   ```bash
   anchor test
   ```

### Pinocchio / Low-level Rust Projects (Weeks 3, 4)
1. Build BPF/SBF binary:
   ```bash
   cargo build-sbf
   ```
2. Execute Rust native unit tests:
   ```bash
   cargo test -- --nocapture
   ```

### 💡 Alternative LiteSVM Test Suite
For Week 1, the [/anchor-escrow-with-litesvm](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-with-litesvm) directory demonstrates how to use the fast, in-process **LiteSVM** emulator in TypeScript to completely bypass `solana-test-validator` local startup latency.
*   Check the test file [anchor-escrow.ts](file:///home/divine/turbine/acc-turbine/Q2_2026_Accel_DivineUX23/anchor-escrow-with-litesvm/tests/anchor-escrow.ts) for details.

---
*Created as part of the Turbin3 Q2 2026 cohort assignments.*
