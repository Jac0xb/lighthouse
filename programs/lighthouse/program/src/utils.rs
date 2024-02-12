use crate::{
    error::LighthouseError,
    types::{operator::EvaluationResult, Assertion},
};
use anchor_lang::prelude::Result;
use solana_program::{msg, program_option::COption, pubkey::Pubkey};

pub fn print_assertion_result(
    assertion: &Assertion,
    assertion_index: usize,
    evaluation_result: &EvaluationResult,
) {
    msg!(
        // repeating zeros infront of assettion index
        "{} {} {} {}",
        format!("[{:0>2}]", assertion_index),
        if evaluation_result.passed {
            "[✓] PASSED"
        } else {
            "[✕] FAILED"
        },
        assertion.format(),
        evaluation_result.output
    );
}

pub fn unpack_coption_key(src: &[u8]) -> Result<COption<Pubkey>> {
    let tag = &src[0..4];
    let body = &src[4..36];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(
            body.try_into().unwrap(),
        ))),
        _ => Err(LighthouseError::AccountNotInitialized.into()),
    }
}

pub fn unpack_coption_u64(src: &[u8]) -> Result<COption<u64>> {
    let tag = &src[0..4];
    let body = &src[4..12];

    match *tag {
        [0, 0, 0, 0] => Ok(COption::None),
        [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(body.try_into().unwrap()))),
        _ => Err(LighthouseError::AccountNotInitialized.into()),
    }
}
