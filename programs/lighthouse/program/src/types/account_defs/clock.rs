use crate::{Assert, EvaluationResult, Operator};
use anchor_lang::{
    prelude::borsh::{self, BorshDeserialize, BorshSerialize},
    Result,
};
use solana_program::clock::Clock;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum ClockField {
    Slot(u64),
    EpochStartTimestamp(i64),
    Epoch(u64),
    LeaderScheduleEpoch(u64),
    UnixTimestamp(i64),
}

impl Assert<Clock> for ClockField {
    fn evaluate(
        &self,
        clock: &Clock,
        operator: &Operator,
        include_output: bool,
    ) -> Result<Box<EvaluationResult>> {
        let result = match self {
            ClockField::Slot(slot) => {
                let actual_slot = clock.slot;

                operator.evaluate(&actual_slot, slot, include_output)
            }
            ClockField::EpochStartTimestamp(epoch_start_timestamp) => {
                let actual_epoch_start_timestamp = clock.epoch_start_timestamp;

                operator.evaluate(
                    &actual_epoch_start_timestamp,
                    epoch_start_timestamp,
                    include_output,
                )
            }
            ClockField::Epoch(epoch) => {
                let actual_epoch = clock.epoch;

                operator.evaluate(&actual_epoch, epoch, include_output)
            }
            ClockField::LeaderScheduleEpoch(leader_schedule_epoch) => {
                let actual_leader_schedule_epoch = clock.leader_schedule_epoch;

                operator.evaluate(
                    &actual_leader_schedule_epoch,
                    leader_schedule_epoch,
                    include_output,
                )
            }
            ClockField::UnixTimestamp(unix_timestamp) => {
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
        use crate::{Assert, ClockField, Operator};

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

            let result = ClockField::Slot(69).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = ClockField::Slot(1600).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch_start_timestamp

            let result =
                ClockField::EpochStartTimestamp(420).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                ClockField::EpochStartTimestamp(1600).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate epoch

            let result = ClockField::Epoch(1337).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = ClockField::Epoch(1600).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate leader_schedule_epoch

            let result =
                ClockField::LeaderScheduleEpoch(9001).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result =
                ClockField::LeaderScheduleEpoch(1600).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            // Evaluate unix_timestamp

            let result =
                ClockField::UnixTimestamp(123456789).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }

            let result = ClockField::UnixTimestamp(1600).evaluate(&clock, &Operator::Equal, true);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
