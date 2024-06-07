import { LanguageOutput } from '.';

export type Operator = 'EquatableOperator' | 'IntegerOperator';
export type ValueType = 'number' | 'bigint' | 'PublicKey';
export type Kind =
  | 'U8'
  | 'I8'
  | 'U16'
  | 'I16'
  | 'U32'
  | 'I32'
  | 'U64'
  | 'I64'
  | 'U128'
  | 'I128'
  | 'Pubkey';

export const KIND_MAP: {
  [key in Kind]: {
    kind: Kind;
    operatorType: Operator;
  };
} = {
  U8: {
    kind: 'U8',
    operatorType: 'IntegerOperator',
  },
  I8: {
    kind: 'I8',
    operatorType: 'IntegerOperator',
  },
  U16: {
    kind: 'U16',
    operatorType: 'IntegerOperator',
  },
  I16: {
    kind: 'I16',
    operatorType: 'IntegerOperator',
  },
  U32: {
    kind: 'U32',
    operatorType: 'IntegerOperator',
  },
  I32: {
    kind: 'I32',
    operatorType: 'IntegerOperator',
  },
  U64: {
    kind: 'U64',
    operatorType: 'IntegerOperator',
  },
  I64: {
    kind: 'I64',
    operatorType: 'IntegerOperator',
  },
  U128: {
    kind: 'U128',
    operatorType: 'IntegerOperator',
  },
  I128: {
    kind: 'I128',
    operatorType: 'IntegerOperator',
  },
  Pubkey: {
    kind: 'Pubkey',
    operatorType: 'EquatableOperator',
  },
};

export const KindToRust = {
  U8: 'u8',
  I8: 'i8',
  U16: 'u16',
  I16: 'i16',
  U32: 'u32',
  I32: 'i32',
  U64: 'u64',
  I64: 'i64',
  U128: 'u128',
  I128: 'i128',
  Pubkey: 'solana_program::pubkey::Pubkey',
};

export const KindToTs = {
  U8: 'number',
  I8: 'number',
  U16: 'number',
  I16: 'number',
  U32: 'number',
  I32: 'number',
  U64: 'number | bigint',
  I64: 'number | bigint',
  U128: 'number | bigint',
  I128: 'number | bigint',
  Pubkey: 'PublicKey',
};

export const renderKind = (
  kind: Kind,
  language: LanguageOutput
): {
  kind: Kind;
  valueType: string;
  operatorType: Operator;
} => {
  const kindValue = KIND_MAP[kind];

  switch (language) {
    case 'typescript-preview':
    case 'typescript':
      return {
        kind: kindValue.kind,
        valueType: KindToTs[kind],
        operatorType: kindValue.operatorType,
      };
    case 'rust':
      return {
        kind: kindValue.kind,
        valueType: KindToRust[kind],
        operatorType: kindValue.operatorType,
      };
  }
};
