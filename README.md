# amm_turbine

Turbin3 builder project — simple Constant Product Market Maker (CPMM) on Solana using Anchor.

## Formula

```
x * y = k
```

Where `x` and `y` are the reserves of two tokens in the pool, and `k` is the invariant maintained across all swaps.

## Architecture

### Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Create a new pool for a token pair, deploy vaults and LP mint |
| `deposit` | Add liquidity proportionally, receive LP tokens |
| `withdraw` | Burn LP tokens, receive underlying tokens back |
| `swap` | Swap one token for the other using the `x * y = k` formula |

## Setup

```bash
# install deps
yarn

# build
anchor build

# run tests (LiteSVM)
cargo test
```

## Environment

- Solana toolchain: see `rust-toolchain.toml`
- Cluster: localnet (see `Anchor.toml`)
- Program ID: `C9VNgETz1HMM1G73aVT6b1KVFG2ag1yBQ4PKXuwiCFm5`
