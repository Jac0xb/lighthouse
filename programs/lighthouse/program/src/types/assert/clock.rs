use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::clock::Clock;

use crate::types::{Assert, ComparableOperator, EvaluationResult, Operator};
use crate::utils::Result;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum SysvarClockFieldAssertion {
    Slot(u64, ComparableOperator),
    EpochStartTimestamp(i64, ComparableOperator),
    Epoch(u64, ComparableOperator),
    LeaderScheduleEpoch(u64, ComparableOperator),
    UnixTimestamp(i64, ComparableOperator),
}

impl Assert<Clock> for SysvarClockFieldAssertion {
    fn format(&self) -> String {
        format!("SysvarClockFieldAssertion[{:?}]", self)
    }

    fn evaluate(&self, clock: &Clock, include_output: bool) -> Result<Box<EvaluationResult>> {
        let result = match self {
            SysvarClockFieldAssertion::Slot(slot, operator) => {
                let actual_slot = clock.slot;

                operator.evaluate(&actual_slot, slot, include_output)
            }
            SysvarClockFieldAssertion::EpochStartTimestamp(epoch_start_timestamp, operator) => {
                let actual_epoch_start_timestamp = clock.epoch_start_timestamp;

                operator.evaluate(
                    &actual_epoch_start_timestamp,
                    epoch_start_timestamp,
                    include_output,
                )
            }
            SysvarClockFieldAssertion::Epoch(epoch, operator) => {
                let actual_epoch = clock.epoch;

                operator.evaluate(&actual_epoch, epoch, include_output)
            }
            SysvarClockFieldAssertion::LeaderScheduleEpoch(leader_schedule_epoch, operator) => {
                let actual_leader_schedule_epoch = clock.leader_schedule_epoch;

                operator.evaluate(
                    &actual_leader_schedule_epoch,
                    leader_schedule_epoch,
                    include_output,
                )
            }
            SysvarClockFieldAssertion::UnixTimestamp(unix_timestamp, operator) => {
                let actual_unix_timestamp = clock.unix_timestamp;

                operator.evaluate(&actual_unix_timestamp, unix_timestamp, include_output)
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use crate::types::{Assert, ComparableOperator, Operator, SysvarClockFieldAssertion};

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

            let result = SysvarClockFieldAssertion::Slot(69, ComparableOperator::Equal)
                .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockFieldAssertion::Slot(1600, ComparableOperator::Equal)
                .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch_start_timestamp

            let result =
                SysvarClockFieldAssertion::EpochStartTimestamp(420, ComparableOperator::Equal)
                    .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                SysvarClockFieldAssertion::EpochStartTimestamp(1600, ComparableOperator::Equal)
                    .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch

            let result = SysvarClockFieldAssertion::Epoch(1337, ComparableOperator::Equal)
                .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockFieldAssertion::Epoch(1600, ComparableOperator::Equal)
                .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate leader_schedule_epoch

            let result =
                SysvarClockFieldAssertion::LeaderScheduleEpoch(9001, ComparableOperator::Equal)
                    .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                SysvarClockFieldAssertion::LeaderScheduleEpoch(1600, ComparableOperator::Equal)
                    .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate unix_timestamp

            let result =
                SysvarClockFieldAssertion::UnixTimestamp(123456789, ComparableOperator::Equal)
                    .evaluate(&clock, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = SysvarClockFieldAssertion::UnixTimestamp(1600, ComparableOperator::Equal)
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
