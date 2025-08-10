use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use super::SwapOutcome;

#[derive(Clone, Debug)]
pub struct MeteoraAccounts<'a> {
    pub program: AccountInfo<'a>,
    pub metas: Vec<AccountInfo<'a>>, // excludes ix_data
    pub ix_data: AccountInfo<'a>,
}

pub fn collect_accounts<'a>(acc_iter: &mut std::slice::Iter<'a, AccountInfo<'a>>) -> Result<MeteoraAccounts<'a>, ProgramError> {
    let program = acc_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?.clone();
    let rest: Vec<AccountInfo> = acc_iter.cloned().collect();
    if rest.len() < 2 { return Err(ProgramError::NotEnoughAccountKeys); }
    let (metas, last) = rest.split_at(rest.len() - 1);
    Ok(MeteoraAccounts { program, metas: metas.to_vec(), ix_data: last[0].clone() })
}

pub fn cpi_swap(
    _payer: &AccountInfo,
    _user_quote_ata: &AccountInfo,
    user_token_ata: &AccountInfo,
    _token_program: &AccountInfo,
    met: MeteoraAccounts,
    _amount_in: u64,
    min_amount_out: u64,
) -> Result<SwapOutcome, ProgramError> {
    let pre_out = read_token_amount(user_token_ata)?;

    let data_vec = {
        let data = met.ix_data.try_borrow_data()?;
        data.to_vec()
    };
    let metas: Vec<AccountMeta> = met.metas.iter().map(|ai| AccountMeta {
        pubkey: *ai.key,
        is_signer: ai.is_signer,
        is_writable: ai.is_writable,
    }).collect();

    let ix = Instruction { program_id: *met.program.key, accounts: metas, data: data_vec };

    let mut infos: Vec<AccountInfo> = Vec::with_capacity(1 + met.metas.len() + 1);
    infos.push(met.program);
    for a in met.metas { infos.push(a); }
    infos.push(met.ix_data);
    invoke(&ix, &infos)?;

    let post_out = read_token_amount(user_token_ata)?;
    let amount_out = post_out.saturating_sub(pre_out);

    if amount_out < min_amount_out { return Err(super::super::error::RouterError::SlippageExceeded.into()); }

    Ok(SwapOutcome { amount_out, fee_paid: 0, dex_id: 1 })
}

fn read_token_amount(ai: &AccountInfo) -> Result<u64, ProgramError> {
    let data = ai.try_borrow_data()?;
    if data.len() < 72 { return Err(ProgramError::InvalidAccountData); }
    let amount_bytes: [u8; 8] = data[64..72].try_into().unwrap();
    Ok(u64::from_le_bytes(amount_bytes))
}
