use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum ClockField {
    Slot,
    EpochStartTimestamp,
    Epoch,
    LeaderScheduleEpoch,
    UnixTimestamp,
}
