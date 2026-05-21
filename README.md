# amm_turbine

Turbin3 builder project — Constant Product Market Maker (CPMM) on Solana using Anchor.

## Formula

```
x * y = k
```

`x` and `y` are the reserves of the two tokens in the pool. `k` is the invariant maintained across all swaps. Fees are applied to the effective input amount before the invariant calculation, so they accumulate in the pool over time.

## Architecture

### State

**`Config` PDA** — seeds `["config", seed]`

| Field | Type | Description |
|-------|------|-------------|
| `seed` | `u64` | Differentiates multiple pools for the same pair |
| `authority` | `Option<Pubkey>` | Admin allowed to lock/unlock the pool |
| `mint_x` | `Pubkey` | Token X mint |
| `mint_y` | `Pubkey` | Token Y mint |
| `fee` | `u16` | Fee in basis points (e.g. `30` = 0.30%) |
| `locked` | `bool` | Pauses deposits and swaps when `true` |
| `config_bump` | `u8` | PDA bump for `config` |
| `lp_bump` | `u8` | PDA bump for `mint_lp` |

**`mint_lp` PDA** — seeds `["lp", config]` — LP token mint, authority is the config PDA.

**Vaults** — ATAs (PDAs derived by the associated token program from `[config, token_program, mint]`), one per token. Authority is the config PDA.

### Instructions

| Instruction | Parameters | Description |
|-------------|------------|-------------|
| `initialize` | `seed, fee, authority` | Create a new pool, deploy vaults and LP mint |
| `deposit` | `amount, max_x, max_y` | Add liquidity, receive `amount` LP tokens. On first deposit sets the initial price. Subsequent deposits are proportional to current reserves. |
| `withdraw` | `amount, min_x, min_y` | Burn `amount` LP tokens, receive underlying tokens back proportionally |
| `swap` | `is_x, amount_in, min_amount_out` | Swap one token for the other. `is_x=true` sends X, receives Y. Fee applied before invariant calculation. |
| `lock` | `lock` | Set `config.locked`. Only callable by `authority`. Locked pool rejects deposits and swaps. |

### Error codes

| Error | Description |
|-------|-------------|
| `PoolLocked` | Deposit or swap attempted on a locked pool |
| `InvalidAmount` | Amount is zero |
| `SlippageExceeded` | Output below `min` or input above `max` |
| `Unauthorized` | Signer is not the pool authority |
| `NoAuthority` | Lock attempted on a pool with no authority set |

## Precision notes

Fee and swap calculations use integer arithmetic (u64). At small token amounts, truncation is significant:
- `fee_adjusted_in = amount * (10_000 - fee) / 10_000`
- On tiny amounts (e.g. `amount=2`, `fee=30`): effective input drops from `1.994` to `1`
- Use amounts with 6 decimals in production (i.e. `1_000_000` = 1 token) for realistic precision

## Testing

Tests use [LiteSVM](https://github.com/LiteSVM/litesvm) — an in-memory SVM, no validator required. Each test is fully isolated: a fresh `LiteSVM` instance is created per test via `setup()`.

```bash
# build
anchor build

# run tests
cargo test
```

## Setup

- Solana toolchain: see `rust-toolchain.toml`
- Cluster: localnet (see `Anchor.toml`)
- Program ID: `C9VNgETz1HMM1G73aVT6b1KVFG2ag1yBQ4PKXuwiCFm5`
