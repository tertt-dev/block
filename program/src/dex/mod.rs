pub mod detect;
pub mod raydium_v4;
pub mod meteora_v1;

#[derive(Clone, Debug)]
pub struct SwapOutcome {
    pub amount_out: u64,
    pub fee_paid: u64,
    pub dex_id: u8,
}

pub struct SwapParams<'a> {
    pub user_signer: &'a solana_program::account_info::AccountInfo<'a>,
    pub user_quote_ata: &'a solana_program::account_info::AccountInfo<'a>,
    pub user_token_ata: &'a solana_program::account_info::AccountInfo<'a>,
    pub token_program: &'a solana_program::account_info::AccountInfo<'a>,
    pub token_mint: solana_program::pubkey::Pubkey,
    pub quote_mint: solana_program::pubkey::Pubkey,
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub dex_ix_bytes: &'a [u8],
}

#[derive(Clone, Debug)]
pub enum DexChoice<'a> {
    RaydiumV4 { accounts: raydium_v4::RaydiumAccounts<'a> },
    MeteoraV1 { accounts: meteora_v1::MeteoraAccounts<'a> },
}
