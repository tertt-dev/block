## Block Router (Solana)

Rust Solana program that routes swaps for a desired SPL token using a quote token (e.g., WSOL). It auto-picks a supported DEX (Raydium v4 or Meteora Dynamic Pools v1) via a cheap flag account and performs a CPI pass-through swap.

### Build

```bash
cargo build -p block-router
```

### Instruction format

Variant 0: Swap

```
u8   : 0
bytes: token_mint (32)
bytes: quote_mint (32)
u64  : amount_in (LE)
u64  : min_amount_out (LE)
# trailing payload (ignored by program)
```

### Accounts (prefix)

```
0. signer: user authority (payer)
1. user quote ATA (source)
2. user token ATA (dest)
3. token program
4. system program (unused)
5. flag account (derives DEX: even first byte => Raydium, odd => Meteora)
# then DEX-specific accounts:
#   program id
#   <metas...>
#   ix_data account (holds raw ix bytes prepared off-chain)
```

### Notes

- Program computes amount_out by reading user destination ATA balance delta.
- Fees are logged as 0 (DEX fee not parsed on-chain here).
- Errors: InvalidAmount, InsufficientLiquidity, SlippageExceeded, MarketNotFound, InvalidAccounts.

### Why pass-through CPI?

Avoids brittle on-chain encoding of third-party instructions. Client prepares the exact Raydium/Meteora ix bytes; the program forwards them and enforces only min_amount_out and basic sanity. This keeps compute small and makes adding DEXes trivial.


