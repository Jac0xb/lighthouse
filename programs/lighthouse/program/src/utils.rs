use crate::types::{operator::EvaluationResult, Assertion};
use solana_program::msg;

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
