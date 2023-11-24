## Temporal Smart Contracts

This repository contains contracts for Temporal's **Automated Market Maker**. This contract is only responsible for maintaining holding tokens, maintaining escrows, and different states for the users throughout. The AMM and its logic are hosted on an off-chain Backend.

## Prerequisites
- Rust (1.72.1 or higher)
- Docker (4.24.0 or higher)

## Installation
- You can install **Rust** from https://www.rust-lang.org/tools/install
- You can install **Dcoker** from https://docs.docker.com/get-docker/

## System Design 

![AMMMech](https://github.com/T-emporal/ammse/assets/41021590/58e67b70-dcf0-4e40-9651-93986ad4ffae)

## High Level Design of contracts 

![HighLevelDesign](https://github.com/T-emporal/ammse/assets/41021590/e6a7a979-92a5-4290-961a-7fee13a0931f)

-	The Lender sends the Token in the Lending Pool
-	A borrower Requests the token from the Pool
-	A borrower repays the loan to the pool, Includes the Interest tokens
-	The Pool returns the Token to the Lender after aggreged upon lending period

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
