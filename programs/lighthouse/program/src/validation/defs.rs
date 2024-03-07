use solana_program::pubkey::Pubkey;

pub trait DerivedAddress<T> {
    fn get_seeds(parameters: T) -> Vec<Vec<u8>>;
}

pub trait Id {
    fn id() -> Pubkey;
}
