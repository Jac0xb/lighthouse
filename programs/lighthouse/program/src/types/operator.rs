use std::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Exists,
    DoesNotExist,
}

pub struct EvaluationResult {
    pub passed: bool,
    pub output: String,
}

impl Operator {
    pub fn evaluate<T: PartialEq + Eq + PartialOrd + Ord + Debug>(
        &self,
        actual_value: &T,
        assertion_value: &T,
        output: bool,
    ) -> Box<EvaluationResult> {
        Box::new(EvaluationResult {
            passed: match self {
                Operator::Equal => T::eq(actual_value, assertion_value),
                Operator::NotEqual => T::ne(actual_value, assertion_value),
                Operator::GreaterThan => T::gt(actual_value, assertion_value),
                Operator::LessThan => T::lt(actual_value, assertion_value),
                Operator::GreaterThanOrEqual => T::ge(actual_value, assertion_value),
                Operator::LessThanOrEqual => T::le(actual_value, assertion_value),
                Operator::Exists => true,
                Operator::DoesNotExist => false,
            },
            output: if output {
                format!("{:?} {} {:?}", actual_value, self.format(), assertion_value)
            } else {
                "".to_string()
            },
        })
    }

    pub fn format(&self) -> String {
        match self {
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanOrEqual => ">=",
            Operator::LessThanOrEqual => "<=",
            Operator::Exists => "Exists",
            Operator::DoesNotExist => "DoesNotExist",
        }
        .to_string()
    }
}
