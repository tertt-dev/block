use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use super::{SwapOutcome, SwapParams};
use crate::util::read_spl_token_amount;

#[derive(Clone, Debug)]
pub struct MeteoraAccounts<'a> {
    pub program: AccountInfo<'a>,
    pub metas: Vec<AccountInfo<'a>>,
}

pub fn collect_accounts<'a>(acc_iter: &mut std::slice::Iter<'a, AccountInfo<'a>>) -> Result<MeteoraAccounts<'a>, ProgramError> {
    let program = acc_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?.clone();
    let metas: Vec<AccountInfo> = acc_iter.cloned().collect();
    if metas.is_empty() { return Err(ProgramError::NotEnoughAccountKeys); }
    Ok(MeteoraAccounts { program, metas })
}

pub fn cpi_swap(
    user_token_ata: &AccountInfo,
    met: MeteoraAccounts,
    params: &SwapParams,
) -> Result<SwapOutcome, ProgramError> {
    let pre_out = read_spl_token_amount(user_token_ata)?;
    let metas: Vec<AccountMeta> = met.metas.iter().map(|ai| AccountMeta {
        pubkey: *ai.key,
        is_signer: ai.is_signer,
        is_writable: ai.is_writable,
    }).collect();

    let ix = Instruction { program_id: *met.program.key, accounts: metas, data: params.dex_ix_bytes.to_vec() };

    let mut infos: Vec<AccountInfo> = Vec::with_capacity(1 + met.metas.len());
    infos.push(met.program);
    for a in met.metas { infos.push(a); }
    invoke(&ix, &infos)?;

    let post_out = read_spl_token_amount(user_token_ata)?;
    let amount_out = post_out.saturating_sub(pre_out);

    if amount_out == 0 { return Err(super::super::error::RouterError::InsufficientLiquidity.into()); }
    if amount_out < params.min_amount_out { return Err(super::super::error::RouterError::SlippageExceeded.into()); }

    Ok(SwapOutcome { amount_out, fee_paid: 0, dex_id: 1 })
}
