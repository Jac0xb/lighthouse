use solana_program::pubkey::Pubkey;

use crate::{
    generate_builder,
    types::{EquatableOperator, IntegerOperator},
};

generate_builder! {
    VoteAccountAssertion,
    AssertVoteAccountBuilder,
    [
        AuthorizedWithdrawer {
            value_type: Pubkey,
            operator_type: EquatableOperator,
            assertions: [
                (0, U8, |_value, _operator| 1, IntegerOperator::Equal),
                (36, Pubkey, |value, _operator| value, EquatableOperator::Equal)
            ]
        }
    ]
}
