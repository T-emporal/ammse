## Temporal Smart Contracts

This repository contains contracts for Temporal's **Automated Market Maker**. This contract is only responsible for maintaining holding tokens, maintaining escrows, and different states for the users throughout. The AMM and its logic are hosted on an off-chain Backend.

## Prerequisites
- Rust (1.72.1 or higher)
- Docker (4.24.0 or higher)

## Installation
- You can install **Rust** from https://www.rust-lang.org/tools/install
- You can install **Dcoker** from https://docs.docker.com/get-docker/

## Setup
clone this repository
```
git clone git@github.com:T-emporal/ammse.git
cd ammse
```
### Run test cases -

```
cargo test
```

### Buidling the contracts

for Mac system (M1, M2, ..)
```
 docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.14.0

```

for non Mac system -
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```
Temporal Litepaper - https://temporal.exchange/litepaper
