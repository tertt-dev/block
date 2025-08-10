#![deny(clippy::all)]
#![allow(clippy::too_many_arguments)]

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    log::sol_log_compute_units,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod state;
pub mod error;
pub mod ix;
pub mod dex;

use crate::{error::RouterError, ix::RouterInstruction};

entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &'a [u8],
) -> ProgramResult {
    // ==manual decode.
    let instruction = RouterInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        RouterInstruction::Swap { token_mint, quote_mint, amount_in, min_amount_out } => {
            let dex_ix_bytes = &input[ix::RouterInstruction::swap_header_len()..];
            processor::process_swap(program_id, accounts, token_mint, quote_mint, amount_in, min_amount_out, dex_ix_bytes)
        }
    }
}

mod processor {
    use super::*;
    use crate::dex::{detect::detect_market, raydium_v4, meteora_v1, DexChoice, SwapParams};

    pub fn process_swap<'a>(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        token_mint: Pubkey,
        quote_mint: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
        dex_ix_bytes: &'a [u8],
    ) -> ProgramResult {
        if amount_in == 0 { return Err(RouterError::InvalidAmount.into()); }

        let acc_iter = &mut accounts.iter();
        let payer_ai = next_account_info(acc_iter)?; // user authority
        let user_quote_ata = next_account_info(acc_iter)?;
        let user_token_ata = next_account_info(acc_iter)?;
        let token_program = next_account_info(acc_iter)?;
        let _system_program = next_account_info(acc_iter)?; // might not be used

        let choice = detect_market(token_mint, quote_mint, acc_iter)?;

        let params = SwapParams {
            user_signer: payer_ai,
            user_quote_ata,
            user_token_ata,
            token_program,
            amount_in,
            min_amount_out,
            dex_ix_bytes,
        };

        let result = match choice {
            DexChoice::RaydiumV4 { accounts: ray_accs } => {
                raydium_v4::cpi_swap(user_token_ata, ray_accs, &params)
            }
            DexChoice::MeteoraV1 { accounts: met_accs } => {
                meteora_v1::cpi_swap(user_token_ata, met_accs, &params)
            }
        }?;

        msg!("swap_ok: amount_out={} fee_paid={} dex={}", result.amount_out, result.fee_paid, result.dex_id);
        sol_log_compute_units();
        Ok(())
    }
}
