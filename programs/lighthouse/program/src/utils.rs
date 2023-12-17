use crate::structs::Operator;
use solana_program::msg;

pub fn print_result(
    assertion_result: bool,
    assertion_index: usize,
    operator: Operator,
    value_str: String,
    expected_value_str: String,
) {
    msg!(
        "{} {} Assertion::Memory ({}) -> {} {} {}",
        format!("[{:?}]", assertion_index),
        if assertion_result {
            "[✅] SUCCESS"
        } else {
            "[❌] FAIL   "
        },
        "Cache...".to_string(),
        value_str,
        operator.format(),
        expected_value_str,
    );
}
