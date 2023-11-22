use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct CacheAccount {
    pub data: [u8; 1024],
}
