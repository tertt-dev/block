use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum RouterInstruction {
    Swap {
        token_mint: Pubkey,
        quote_mint: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
        // carry DEX-specific ix data (ignored in program)
    }
}

impl RouterInstruction {
    pub fn try_from_slice(data: &[u8]) -> Result<Self, ()> {
        // v==ery small bespoke decoder
        if data.len() < 1 { return Err(()); }
        match data[0] {
            0 => {
                if data.len() < 1 + 32 + 32 + 8 + 8 { return Err(()); }
                let token_mint = Pubkey::new_from_array(data[1..33].try_into().unwrap());
                let quote_mint = Pubkey::new_from_array(data[33..65].try_into().unwrap());
                let amount_in = u64::from_le_bytes(data[65..73].try_into().unwrap());
                let min_amount_out = u64::from_le_bytes(data[73..81].try_into().unwrap());
                Ok(Self::Swap{ token_mint, quote_mint, amount_in, min_amount_out })
            }
            _ => Err(())
        }
    }
}
