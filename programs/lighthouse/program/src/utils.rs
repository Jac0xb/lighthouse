use crate::{processor::assert::AssertionConfig, structs::Operator};
use solana_program::msg;

pub fn print_assertion_result(
    config: &Option<AssertionConfig>,
    assertion_info: String,
    assertion_result: bool,
    assertion_index: usize,
    operator: &Operator,
    value_str: String,
    expected_value_str: String,
) {
    if let Some(config) = config {
        if !config.verbose {
            return;
        }
    } else {
        return;
    }

    msg!(
        "{} {} {} -> {} {} {}",
        format!("[{:?}]", assertion_index),
        if assertion_result {
            "[✅] SUCCESS"
        } else {
            "[❌] FAIL   "
        },
        assertion_info,
        value_str,
        operator.format(),
        expected_value_str,
    );
}
