use solana_program::pubkey::Pubkey;

pub(crate) trait DerivedAddress<T> {
    fn get_seeds(parameters: T) -> Vec<Vec<u8>>;
}

pub(crate) trait Id {
    fn id() -> Pubkey;
}
