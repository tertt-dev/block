use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Clone, Copy, Debug, Error)]
pub enum RouterError {
    #[error("Invalid amount")] 
    InvalidAmount,
    #[error("Market not found")] 
    MarketNotFound,
    #[error("Slippage too high")] 
    SlippageExceeded,
    #[error("Liquidity is insufficient")] 
    InsufficientLiquidity,
    #[error("Invalid accounts")] 
    InvalidAccounts,
}

impl From<RouterError> for ProgramError {
    fn from(e: RouterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
