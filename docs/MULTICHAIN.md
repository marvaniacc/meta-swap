# Multi-chain core contract

## Scope

The MVP executes intra-chain swaps on TON only. The core must allow future chain adapters for EVM, Solana, and Bitcoin without turning TON data structures into universal domain types. Cross-chain swaps and bridges are explicitly out of scope.

## Identity rules

```text
Asset identity = (ChainId, canonical native marker or canonical contract/program/master address)
Symbol is display metadata, never identity.
Amount is valid only with its AssetId and atomic-unit scale.
```

## Core concepts

- `ChainId`, `ChainFamily`, `NetworkId`, and `FinalityLevel` identify execution context.
- `AssetId`, `AssetKind`, `CanonicalAddress`, and `AssetAmount` model assets safely.
- `Quote`, `RoutePlan`, `TransactionIntent`, `ChainSubmissionReference`, and `VerifiedExecution` are chain-neutral records.
- All amounts are non-negative atomic integers. Rounding is explicit and recorded with policy version.

## Adapter responsibilities

Each chain adapter must implement or declare support for address normalization, asset validation, wallet handoff, intent compilation, chain observation, finality evaluation, and independent execution verification. A new chain cannot be activated by adding an RPC URL; it requires a dedicated adapter, wallet matrix, verifier, testnet E2E suite, threat-model supplement, and launch approval.

## Isolation rule

Only `TonAdapter` may know about BOCs, cells, Jettons, and TON router message bodies. EVM calldata, Solana instructions, PSBTs, and UTXO selection must likewise remain inside their future adapters.
