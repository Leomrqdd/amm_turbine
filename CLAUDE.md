# AMM Turbine — Project Notes

## Anchor PDA signer seeds

Always use this form when building `signer_seeds` for `CpiContext::new_with_signer`:

```rust
let signer_seeds: &[&[&[u8]]] = &[&[
    b"config",
    &self.config.seed.to_le_bytes(),
    &[self.config.config_bump],
]];
```

- The explicit type annotation `&[&[&[u8]]]` resolves element-type mismatch between `&[u8; N]` slices of different sizes.
- Do not introduce an intermediate `seed_bytes` binding.
