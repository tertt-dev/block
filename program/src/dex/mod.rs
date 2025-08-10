pub mod detect;
pub mod raydium_v4;
pub mod meteora_v1;

#[derive(Clone, Debug)]
pub struct SwapOutcome {
    pub amount_out: u64,
    pub fee_paid: u64,
    pub dex_id: u8,
}

#[derive(Clone, Debug)]
pub enum DexChoice<'a> {
    RaydiumV4 { accounts: raydium_v4::RaydiumAccounts<'a> },
    MeteoraV1 { accounts: meteora_v1::MeteoraAccounts<'a> },
}
