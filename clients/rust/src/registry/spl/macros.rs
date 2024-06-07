#[macro_export]
macro_rules! generate_builder {
    ($name:ident, $builder_name:ident,
        [$(
            $variant_name:ident {
                value_type: $value_ty:ty,
                operator_type: $operator_ty:ty,
                assertions: [$(($offset:expr, $assert_type:ident, $func:expr, $op:expr)),*]
            }
        ),*]
    ) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $(
                $variant_name {
                    value: $value_ty,
                    operator: $operator_ty,
                },
            )*
        }

        pub struct $builder_name ($crate::generated::instructions::AssertAccountDataMultiBuilder);

        impl $builder_name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self($crate::generated::instructions::AssertAccountDataMultiBuilder::new())
            }

            #[allow(clippy::redundant_closure_call)]
            pub fn assertion(&mut self, assertion: $name) -> &mut Self {
                match assertion {
                    $(
                        $name::$variant_name { value, operator } => {
                            self.0.assertions(vec![
                                $(
                                    $crate::hooked::AccountDataAssertion {
                                        offset: $crate::CompactU64($offset),
                                        assertion: $crate::generated::types::DataValueAssertion::$assert_type {
                                            value: $func(value, &operator),
                                            operator: $op,
                                        },
                                    },
                                )*
                            ].into());
                        },
                    )*
                }
                self
            }

            pub fn log_level(&mut self, log_level: $crate::generated::types::LogLevel) -> &mut Self {
                self.0.log_level(log_level);
                self
            }

            pub fn target_account(&mut self, target_account: $crate::Pubkey) -> &mut Self {
                self.0.target_account(target_account);
                self
            }

            pub fn instruction(&self) -> solana_program::instruction::Instruction {
                self.0.instruction()
            }
        }
    };
}
