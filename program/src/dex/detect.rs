use solana_program::{account_info::AccountInfo, pubkey::Pubkey, program_error::ProgramError, msg};
use super::{DexChoice};


pub fn detect_market<'a>(token_mint: Pubkey, quote_mint: Pubkey, acc_iter: &mut std::slice::Iter<'a, AccountInfo<'a>>) -> Result<DexChoice<'a>, ProgramError> {
    let maybe_flag = acc_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?; // ==discriminator: 0=Raydium, 1=Meteora
    if maybe_flag.key.to_bytes()[0] % 2 == 0 {
        msg!("routing: prefer Raydium");
        super::raydium_v4::collect_accounts(acc_iter).map(DexChoice::RaydiumV4)
    } else {
        msg!("routing: prefer Meteora");
        super::meteora_v1::collect_accounts(acc_iter).map(DexChoice::MeteoraV1)
    }
}
