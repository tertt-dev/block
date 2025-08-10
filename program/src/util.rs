use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

#[inline]
pub fn read_spl_token_amount(account: &AccountInfo) -> Result<u64, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.len() < 72 { return Err(ProgramError::InvalidAccountData); }
    let amount_bytes: [u8; 8] = data[64..72].try_into().unwrap();
    Ok(u64::from_le_bytes(amount_bytes))
}

#[inline]
pub fn read_spl_token_mint(account: &AccountInfo) -> Result<Pubkey, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.len() < 64 { return Err(ProgramError::InvalidAccountData); }
    let mint_bytes: [u8; 32] = data[0..32].try_into().unwrap();
    Ok(Pubkey::new_from_array(mint_bytes))
}

#[inline]
pub fn read_spl_token_owner(account: &AccountInfo) -> Result<Pubkey, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.len() < 64 { return Err(ProgramError::InvalidAccountData); }
    let owner_bytes: [u8; 32] = data[32..64].try_into().unwrap();
    Ok(Pubkey::new_from_array(owner_bytes))
}


