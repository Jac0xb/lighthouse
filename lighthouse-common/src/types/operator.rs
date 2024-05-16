pub trait Operator {
    fn format(&self) -> &str;
}

pub const EQUAL_SYMBOL: &str = "==";
pub const NOT_EQUAL_SYMBOL: &str = "!=";
pub const GREATER_THAN_SYMBOL: &str = ">";
pub const LESS_THAN_SYMBOL: &str = "<";
pub const GREATER_THAN_OR_EQUAL_SYMBOL: &str = ">=";
pub const LESS_THAN_OR_EQUAL_SYMBOL: &str = "<=";
pub const CONTAINS_SYMBOL: &str = "&";
pub const DOES_NOT_CONTAIN_SYMBOL: &str = "!&";
