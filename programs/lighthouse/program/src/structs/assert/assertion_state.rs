use std::collections::BTreeSet;

use crate::{
    error::LighthouseError,
    structs::{Assertion, Expression},
};
pub use anchor_lang::prelude::Result;
use solana_program::msg;

#[derive(Debug)]
pub struct AssertionState {
    pub assertion_results: Vec<bool>,
    pub expressions: Vec<Expression>,
}

impl AssertionState {
    pub fn new(assertions: Vec<Assertion>, expressions: Vec<Expression>) -> Result<Self> {
        let assertion_results: Vec<bool> = vec![true; assertions.len()];

        let expressions = &mut expressions.clone();

        let btree = expressions
            .iter()
            .flat_map(|expression| match expression {
                Expression::And(assertion_indexes) => assertion_indexes.clone(),
                Expression::Or(assertion_indexes) => assertion_indexes.clone(),
            })
            .collect::<BTreeSet<u8>>();

        // find set of indexes not in btree and create an AND expression
        let mut missing_indexes: Vec<u8> = Vec::new();
        for i in 0..assertions.len() {
            if !btree.contains(&(i as u8)) {
                missing_indexes.push(i as u8);
            }
        }
        if !missing_indexes.is_empty() {
            expressions.push(Expression::And(missing_indexes));
        }

        // iterate through btree and make sure that all indexes are in the assertion_results
        for index in btree {
            if index as usize >= assertion_results.len() {
                msg!("expression contained index out of bounds {:?}", index);
                return Err(LighthouseError::OutOfRange.into());
            }
        }

        Ok(Self {
            assertion_results,
            expressions: expressions.clone(),
        })
    }

    pub fn record_result(&mut self, index: usize, result: bool) -> Result<()> {
        self.assertion_results[index] = result;
        Ok(())
    }

    pub fn evaluate(&self) -> Result<()> {
        for (_, expression) in self.expressions.iter().enumerate() {
            match expression {
                Expression::And(assertion_indexes) => {
                    let mut result = true;

                    for assertion_index in assertion_indexes {
                        result = result && self.assertion_results[*assertion_index as usize];
                    }

                    if !result {
                        msg!("expression failed {:?}", expression);
                        return Err(LighthouseError::AssertionFailed.into());
                    }
                }
                Expression::Or(assertion_indexes) => {
                    let mut result = false;

                    for assertion_index in assertion_indexes {
                        result = result || self.assertion_results[*assertion_index as usize];
                    }

                    if !result {
                        msg!("expression failed {:?}", expression);
                        return Err(LighthouseError::AssertionFailed.into());
                    }
                }
            }
        }

        Ok(())
    }
}
