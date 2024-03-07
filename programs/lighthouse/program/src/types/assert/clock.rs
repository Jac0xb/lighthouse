use super::{Assert, LogLevel};
use crate::{
    types::assert::operator::{ComparableOperator, EvaluationResult, Operator},
    utils::Result,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::Clock, sysvar::Sysvar};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
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

impl Assert<()> for SysvarClockAssertion {
    fn evaluate(&self, _: (), log_level: LogLevel) -> Result<Box<EvaluationResult>> {
        let clock = Clock::get()?;

        let result = match self {
            SysvarClockAssertion::Slot {
                value: assertion_value,
                operator,
            } => {
                let actual_slot = clock.slot;

                operator.evaluate(&actual_slot, assertion_value, log_level)
            }
            SysvarClockAssertion::EpochStartTimestamp {
                value: assertion_value,
                operator,
            } => {
                let actual_epoch_start_timestamp = clock.epoch_start_timestamp;

                operator.evaluate(&actual_epoch_start_timestamp, assertion_value, log_level)
            }
            SysvarClockAssertion::Epoch {
                value: assertion_value,
                operator,
            } => {
                let actual_epoch = clock.epoch;

                operator.evaluate(&actual_epoch, assertion_value, log_level)
            }
            SysvarClockAssertion::LeaderScheduleEpoch {
                value: assertion_value,
                operator,
            } => {
                let actual_leader_schedule_epoch = clock.leader_schedule_epoch;

                operator.evaluate(&actual_leader_schedule_epoch, assertion_value, log_level)
            }
            SysvarClockAssertion::UnixTimestamp {
                value: assertion_value,
                operator,
            } => {
                let actual_unix_timestamp = clock.unix_timestamp;

                operator.evaluate(&actual_unix_timestamp, assertion_value, log_level)
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    mod evaluate {
        use crate::types::{
            assert::operator::ComparableOperator,
            assert::{Assert, LogLevel, SysvarClockAssertion},
        };
        use solana_sdk::{
            clock::Clock,
            program_stubs::{set_syscall_stubs, SyscallStubs},
        };

        pub struct MockSyscallStubs;

        impl SyscallStubs for MockSyscallStubs {
            fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
                unsafe {
                    *(var_addr as *mut _ as *mut Clock) = Clock {
                        slot: 69,
                        epoch_start_timestamp: 420,
                        epoch: 1337,
                        leader_schedule_epoch: 9001,
                        unix_timestamp: 123456789,
                    };
                }
                solana_program::entrypoint::SUCCESS
            }
        }

        #[test]
        fn evaluate_clock() {
            set_syscall_stubs(Box::new(MockSyscallStubs {}));

            // Evaluate slot
            let result = SysvarClockAssertion::Slot {
                value: 69,
                operator: ComparableOperator::Equal,
            }
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

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
            .evaluate((), LogLevel::PlaintextMsgLog);

            if let Ok(result) = result {
                assert!(!result.passed, "{:?}", result.output);
            } else {
                let error = result.err().unwrap();
                panic!("{:?}", error);
            }
        }
    }
}
