use crate::{
    processor::assert::AssertionConfig,
    structs::{operator::EvaluationResult, Assertion, Operator},
};
use solana_program::msg;

pub fn print_assertion_result(
    config: &Option<AssertionConfig>,
    assertion: &Assertion,
    assertion_index: usize,
    operator: &Operator,
    evaluation_result: &Box<EvaluationResult>,
) {
    if let Some(config) = config {
        if !config.verbose {
            return;
        }
    } else {
        return;
    }

    msg!(
        // repeating zeros infront of assettion index
        "{} {} {} {} {} {}",
        format!("[{:0>2}]", assertion_index),
        if evaluation_result.passed {
            "[✅] PASSED"
        } else {
            "[❌] FAILED"
        },
        assertion.format(),
        evaluation_result.actual,
        operator.format(),
        evaluation_result.expected,
    );
}
