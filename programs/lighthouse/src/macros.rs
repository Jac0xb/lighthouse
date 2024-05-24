#[macro_export]
macro_rules! generate_asserts_borsh {
    ($assertion:ident, $enum:ident, $data:ident, $log_level:ident,
        standard_cases: [ $( ($variant:ident, $type:tt, $offset:expr) ),* ],
        custom_cases: [ $( ($variant_simple:ident, $func:expr) ),* ]) => {
        match $assertion {
            $(
                $enum::$variant { value, operator } => {
                    let actual_value = generate_asserts_borsh!(@unpack $type, $data, $offset, value);
                    generate_asserts_borsh!(@evaluate $type, actual_value, value, operator, $log_level)
                }
            )*
            $(
                $enum::$variant_simple { value, operator } => {
                    $func(value, operator)
                }
            )*
        }
    };

    (@unpack (Pubkey), $data:ident, $offset:expr, $value:ident) => {
        bytemuck::from_bytes::<Pubkey>($crate::utils::checked_get_slice(&$data, $offset, 32)?)
    };
    (@unpack ([u8]), $data:ident, $offset:expr, $value:ident) => {
        $crate::utils::checked_get_slice(&$data, $offset, $value.len())?
    };
    (@unpack $type:tt, $data:ident, $offset:expr, $value:ident) => {
        $crate::utils::try_from_slice::<$type>(&$data, $offset)?
    };


    (@evaluate (Option<Pubkey>), $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        <Option<&Pubkey>>::evaluate(&$actual_value, &$value.as_ref(), $operator, $log_level)
    };

    (@evaluate (Option<u64>), $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        <Option<&u64>>::evaluate(&$actual_value.as_ref(), &$value.as_ref(), $operator, $log_level)
    };

    (@evaluate $type:tt, $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        #[allow(unused_parens)]
        <$type>::evaluate(&$actual_value, $value, $operator, $log_level)
    };
}

#[macro_export]
macro_rules! generate_asserts_c {
    ($self:ident, $enum:ident, $data:ident, $log_level:ident, $( ($variant:ident, $type:tt, $offset:expr) ),* $( (custom, $variant_simple:ident, $impl:block) ),*) => {
        match $self {
            $(
                $enum::$variant { value, operator } => {
                    let actual_value = generate_asserts_c!(@unpack $type, $data, $offset);
                    generate_asserts_c!(@evaluate $type, actual_value, value, operator, $log_level)
                }
            )*
            $(
                $enum::$variant_simple => {
                    $impl
                }
            )*
        }
    };


    (@unpack (Option<Pubkey>), $data:ident, $offset:expr) => {
        $crate::utils::unpack_coption::<Pubkey>(&$data, $offset)?
    };
    (@unpack (Option<u64>), $data:ident, $offset:expr) => {
        $crate::utils::unpack_coption_u64(&$data, $offset)?
    };
    (@unpack (Pubkey), $data:ident, $offset:expr) => {
        bytemuck::from_bytes::<Pubkey>($crate::utils::checked_get_slice(&$data, $offset, 32)?)
    };
    (@unpack $type:tt, $data:ident, $offset:expr) => {
        $crate::utils::try_from_slice::<$type>(&$data, $offset)?
    };


    (@evaluate (Option<Pubkey>), $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        <Option<&Pubkey>>::evaluate(&$actual_value, &$value.as_ref(), $operator, $log_level)
    };

    (@evaluate (Option<u64>), $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        <Option<&u64>>::evaluate(&$actual_value.as_ref(), &$value.as_ref(), $operator, $log_level)
    };

    (@evaluate $type:tt, $actual_value:ident, $value:ident, $operator:ident, $log_level:ident) => {
        #[allow(unused_parens)]
        <$type>::evaluate(&$actual_value, $value, $operator, $log_level)
    };
}
