use num_enum::TryFromPrimitive;
use shank::ShankInstruction;

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Copy, Clone, ShankInstruction, PartialEq, Eq)]
#[rustfmt::skip]
pub enum LighthouseInstruction {
    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "target_account", desc = "Target account")]
    Assert = 0,

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    MultiAssert = 1,

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account")]
    #[account(2, name = "memory_account", desc = "Memory account")]
    #[account(3, name = "system_program", desc = "System program")]
    CreateMemoryAccount = 2,

    #[account(0, name = "lighthouse_program", desc = "Lighthouse program")]
    #[account(1, name = "payer", desc = "Payer account")]
    #[account(2, name = "memory_account", desc = "Memory account")]
    #[account(3, name = "source_account", desc = "System program")]
    // #[account(4, name = "system_program", desc = "System program")]
    Write = 3,
}

impl LighthouseInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}
