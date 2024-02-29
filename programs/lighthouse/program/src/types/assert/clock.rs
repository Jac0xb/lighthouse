use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::clock::Clock;

use crate::types::{Assert, ComparableOperator, EvaluationResult, Operator};
use crate::utils::Result;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum SysvarClockAssertion {
    Slot {
        value: u64,
        operator: ComparableOperator,
    },
    EpochStartTimestamp {
        value: i64,
        operator: ComparableOperator,
    },
    Epoch {
        value: u64,
        operator: ComparableOperator,
    },
    LeaderScheduleEpoch {
        value: u64,
        operator: ComparableOperator,
    },
    UnixTimestamp {
        value: i64,
        operator: ComparableOperator,
    },
}

impl Assert<Clock> for SysvarClockAssertion {
    fn format(&self) -> String {
        format!("SysvarClockAssertion[{:?}]", self)
    }

    fn evaluate(&self, clock: &Clock, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            SysvarClockAssertion::Slot {
                value: assertion_value,
                operator,
            } => {
                let actual_slot = clock.slot;

                operator.evaluate(&actual_slot, assertion_value, include_output)
            }
            SysvarClockAssertion::EpochStartTimestamp {
                value: assertion_value,
                operator,
            } => {
                let actual_epoch_start_timestamp = clock.epoch_start_timestamp;

                operator.evaluate(
                    &actual_epoch_start_timestamp,
                    assertion_value,
                    include_output,
                )
            }
            SysvarClockAssertion::Epoch {
                value: assertion_value,
                operator,
            } => {
                let actual_epoch = clock.epoch;

                operator.evaluate(&actual_epoch, assertion_value, include_output)
            }
            SysvarClockAssertion::LeaderScheduleEpoch {
                value: assertion_value,
                operator,
            } => {
                let actual_leader_schedule_epoch = clock.leader_schedule_epoch;

                operator.evaluate(
                    &actual_leader_schedule_epoch,
                    assertion_value,
                    include_output,
                )
            }
            SysvarClockAssertion::UnixTimestamp {
                value: assertion_value,
                operator,
            } => {
                let actual_unix_timestamp = clock.unix_timestamp;

                operator.evaluate(&actual_unix_timestamp, assertion_value, include_output)
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use crate::types::{Assert, ComparableOperator, SysvarClockAssertion};

        #[test]
        fn evaluate_clock() {
            let clock = solana_program::clock::Clock {
                slot: 69,
                epoch_start_timestamp: 420,
                epoch: 1337,
                leader_schedule_epoch: 9001,
                unix_timestamp: 123456789,
            };

            // Evaluate slot

            let result = SysvarClockAssertion::Slot {
                value: 69,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockAssertion::Slot {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch_start_timestamp

            let result = SysvarClockAssertion::EpochStartTimestamp {
                value: 420,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockAssertion::EpochStartTimestamp {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch

            let result = SysvarClockAssertion::Epoch {
                value: 1337,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockAssertion::Epoch {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate leader_schedule_epoch

            let result = SysvarClockAssertion::LeaderScheduleEpoch {
                value: 9001,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockAssertion::LeaderScheduleEpoch {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate unix_timestamp

            let result = SysvarClockAssertion::UnixTimestamp {
                value: 123456789,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockAssertion::UnixTimestamp {
                value: 1600,
                operator: ComparableOperator::Equal,
            }
            .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
