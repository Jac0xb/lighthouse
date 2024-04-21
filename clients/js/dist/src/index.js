'use strict';

var umi = require('@metaplex-foundation/umi');
var serializers = require('@metaplex-foundation/umi/serializers');
var web3_js = require('@solana/web3.js');

// src/generated/errors/lighthouse.ts
var codeToErrorMap = /* @__PURE__ */ new Map();
var nameToErrorMap = /* @__PURE__ */ new Map();
var InvalidInstructionDataError = class extends umi.ProgramError {
  name = "InvalidInstructionData";
  code = 6e3;
  // 6000
  constructor(program, cause) {
    super("Invalid instruction", program, cause);
  }
};
codeToErrorMap.set(6e3, InvalidInstructionDataError);
nameToErrorMap.set("InvalidInstructionData", InvalidInstructionDataError);
var AssertionFailedError = class extends umi.ProgramError {
  name = "AssertionFailed";
  code = 6001;
  // 6001
  constructor(program, cause) {
    super("AssertionFailed", program, cause);
  }
};
codeToErrorMap.set(6001, AssertionFailedError);
nameToErrorMap.set("AssertionFailed", AssertionFailedError);
var NotEnoughAccountsError = class extends umi.ProgramError {
  name = "NotEnoughAccounts";
  code = 6002;
  // 6002
  constructor(program, cause) {
    super("NotEnoughAccounts", program, cause);
  }
};
codeToErrorMap.set(6002, NotEnoughAccountsError);
nameToErrorMap.set("NotEnoughAccounts", NotEnoughAccountsError);
var BumpNotFoundError = class extends umi.ProgramError {
  name = "BumpNotFound";
  code = 6003;
  // 6003
  constructor(program, cause) {
    super("BumpNotFound", program, cause);
  }
};
codeToErrorMap.set(6003, BumpNotFoundError);
nameToErrorMap.set("BumpNotFound", BumpNotFoundError);
var AccountBorrowFailedError = class extends umi.ProgramError {
  name = "AccountBorrowFailed";
  code = 6004;
  // 6004
  constructor(program, cause) {
    super("AccountBorrowFailed", program, cause);
  }
};
codeToErrorMap.set(6004, AccountBorrowFailedError);
nameToErrorMap.set("AccountBorrowFailed", AccountBorrowFailedError);
var RangeOutOfBoundsError = class extends umi.ProgramError {
  name = "RangeOutOfBounds";
  code = 6005;
  // 6005
  constructor(program, cause) {
    super("RangeOutOfBounds", program, cause);
  }
};
codeToErrorMap.set(6005, RangeOutOfBoundsError);
nameToErrorMap.set("RangeOutOfBounds", RangeOutOfBoundsError);
var IndexOutOfBoundsError = class extends umi.ProgramError {
  name = "IndexOutOfBounds";
  code = 6006;
  // 6006
  constructor(program, cause) {
    super("IndexOutOfBounds", program, cause);
  }
};
codeToErrorMap.set(6006, IndexOutOfBoundsError);
nameToErrorMap.set("IndexOutOfBounds", IndexOutOfBoundsError);
var FailedToDeserializeError = class extends umi.ProgramError {
  name = "FailedToDeserialize";
  code = 6007;
  // 6007
  constructor(program, cause) {
    super("FailedToDeserialize", program, cause);
  }
};
codeToErrorMap.set(6007, FailedToDeserializeError);
nameToErrorMap.set("FailedToDeserialize", FailedToDeserializeError);
var FailedToSerializeError = class extends umi.ProgramError {
  name = "FailedToSerialize";
  code = 6008;
  // 6008
  constructor(program, cause) {
    super("FailedToSerialize", program, cause);
  }
};
codeToErrorMap.set(6008, FailedToSerializeError);
nameToErrorMap.set("FailedToSerialize", FailedToSerializeError);
var AccountOwnerMismatchError = class extends umi.ProgramError {
  name = "AccountOwnerMismatch";
  code = 6009;
  // 6009
  constructor(program, cause) {
    super("AccountOwnerMismatch", program, cause);
  }
};
codeToErrorMap.set(6009, AccountOwnerMismatchError);
nameToErrorMap.set("AccountOwnerMismatch", AccountOwnerMismatchError);
var AccountKeyMismatchError = class extends umi.ProgramError {
  name = "AccountKeyMismatch";
  code = 6010;
  // 6010
  constructor(program, cause) {
    super("AccountKeyMismatch", program, cause);
  }
};
codeToErrorMap.set(6010, AccountKeyMismatchError);
nameToErrorMap.set("AccountKeyMismatch", AccountKeyMismatchError);
var AccountNotInitializedError = class extends umi.ProgramError {
  name = "AccountNotInitialized";
  code = 6011;
  // 6011
  constructor(program, cause) {
    super("AccountNotInitialized", program, cause);
  }
};
codeToErrorMap.set(6011, AccountNotInitializedError);
nameToErrorMap.set("AccountNotInitialized", AccountNotInitializedError);
var AccountOwnerValidationFailedError = class extends umi.ProgramError {
  name = "AccountOwnerValidationFailed";
  code = 6012;
  // 6012
  constructor(program, cause) {
    super("AccountOwnerValidationFailed", program, cause);
  }
};
codeToErrorMap.set(6012, AccountOwnerValidationFailedError);
nameToErrorMap.set(
  "AccountOwnerValidationFailed",
  AccountOwnerValidationFailedError
);
var AccountFundedValidationFailedError = class extends umi.ProgramError {
  name = "AccountFundedValidationFailed";
  code = 6013;
  // 6013
  constructor(program, cause) {
    super("AccountFundedValidationFailed", program, cause);
  }
};
codeToErrorMap.set(6013, AccountFundedValidationFailedError);
nameToErrorMap.set(
  "AccountFundedValidationFailed",
  AccountFundedValidationFailedError
);
var AccountDiscriminatorValidationFailedError = class extends umi.ProgramError {
  name = "AccountDiscriminatorValidationFailed";
  code = 6014;
  // 6014
  constructor(program, cause) {
    super("AccountDiscriminatorValidationFailed", program, cause);
  }
};
codeToErrorMap.set(6014, AccountDiscriminatorValidationFailedError);
nameToErrorMap.set(
  "AccountDiscriminatorValidationFailed",
  AccountDiscriminatorValidationFailedError
);
var AccountValidationFailedError = class extends umi.ProgramError {
  name = "AccountValidationFailed";
  code = 6015;
  // 6015
  constructor(program, cause) {
    super("AccountValidaitonFailed", program, cause);
  }
};
codeToErrorMap.set(6015, AccountValidationFailedError);
nameToErrorMap.set("AccountValidationFailed", AccountValidationFailedError);
var CrossProgramInvokeViolationError = class extends umi.ProgramError {
  name = "CrossProgramInvokeViolation";
  code = 6016;
  // 6016
  constructor(program, cause) {
    super("CrossProgramInvokeViolation", program, cause);
  }
};
codeToErrorMap.set(6016, CrossProgramInvokeViolationError);
nameToErrorMap.set(
  "CrossProgramInvokeViolation",
  CrossProgramInvokeViolationError
);
function getLighthouseErrorFromCode(code, program, cause) {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}
function getLighthouseErrorFromName(name, program, cause) {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
function expectSome(value) {
  if (value == null) {
    throw new Error("Expected a value but received null or undefined.");
  }
  return value;
}
function expectPublicKey(value) {
  if (!value) {
    throw new Error("Expected a PublicKey.");
  }
  return umi.publicKey(value, false);
}
function expectPda(value) {
  if (!value || !Array.isArray(value) || !umi.isPda(value)) {
    throw new Error("Expected a PDA.");
  }
  return value;
}
function getAccountMetasAndSigners(accounts, optionalAccountStrategy, programId) {
  const keys = [];
  const signers = [];
  accounts.forEach((account) => {
    if (!account.value) {
      if (optionalAccountStrategy === "omitted")
        return;
      keys.push({ pubkey: programId, isSigner: false, isWritable: false });
      return;
    }
    if (umi.isSigner(account.value)) {
      signers.push(account.value);
    }
    keys.push({
      pubkey: umi.publicKey(account.value, false),
      isSigner: umi.isSigner(account.value),
      isWritable: account.isWritable
    });
  });
  return [keys, signers];
}
function getAccountDeltaAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "AccountInfo",
        serializers.struct([
          ["aOffset", serializers.u16()],
          ["assertion", getAccountInfoDeltaAssertionSerializer()]
        ])
      ],
      [
        "Data",
        serializers.struct([
          ["aOffset", serializers.u16()],
          ["bOffset", serializers.u16()],
          ["assertion", getDataValueDeltaAssertionSerializer()]
        ])
      ]
    ],
    { description: "AccountDeltaAssertion" }
  );
}
function accountDeltaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAccountDeltaAssertion(kind, value) {
  return value.__kind === kind;
}
function getAccountInfoAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Lamports",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DataLength",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Owner",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "KnownOwner",
        serializers.struct([
          ["value", getKnownProgramSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "RentEpoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsSigner",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "IsWritable",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Executable",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "VerifyDatahash",
        serializers.struct([
          ["expectedHash", serializers.bytes({ size: 32 })],
          ["start", serializers.option(serializers.u16())],
          ["length", serializers.option(serializers.u16())]
        ])
      ]
    ],
    { description: "AccountInfoAssertion" }
  );
}
function accountInfoAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAccountInfoAssertion(kind, value) {
  return value.__kind === kind;
}
function getAccountInfoDeltaAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Lamports",
        serializers.struct([
          ["value", serializers.i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DataLength",
        serializers.struct(
          [
            ["value", serializers.i128()],
            ["operator", getIntegerOperatorSerializer()]
          ]
        )
      ],
      [
        "Owner",
        serializers.struct([
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "RentEpoch",
        serializers.struct([
          ["value", serializers.i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ]
    ],
    { description: "AccountInfoDeltaAssertion" }
  );
}
function accountInfoDeltaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAccountInfoDeltaAssertion(kind, value) {
  return value.__kind === kind;
}
var AccountInfoField = /* @__PURE__ */ ((AccountInfoField3) => {
  AccountInfoField3[AccountInfoField3["Key"] = 0] = "Key";
  AccountInfoField3[AccountInfoField3["Lamports"] = 1] = "Lamports";
  AccountInfoField3[AccountInfoField3["DataLength"] = 2] = "DataLength";
  AccountInfoField3[AccountInfoField3["Owner"] = 3] = "Owner";
  AccountInfoField3[AccountInfoField3["RentEpoch"] = 4] = "RentEpoch";
  AccountInfoField3[AccountInfoField3["Executable"] = 5] = "Executable";
  return AccountInfoField3;
})(AccountInfoField || {});
function getAccountInfoFieldSerializer() {
  return serializers.scalarEnum(AccountInfoField, {
    description: "AccountInfoField"
  });
}
function getAssertionResultSerializer() {
  return serializers.dataEnum(
    [
      [
        "U8",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.u8()), serializers.option(serializers.u8()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "U16",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.u16()), serializers.option(serializers.u16()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "U32",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.u32()), serializers.option(serializers.u32()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "U64",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.u64()), serializers.option(serializers.u64()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "U128",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.u128()), serializers.option(serializers.u128()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "I8",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.i8()), serializers.option(serializers.i8()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "I16",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.i16()), serializers.option(serializers.i16()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "I32",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.i32()), serializers.option(serializers.i32()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "I64",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.i64()), serializers.option(serializers.i64()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "I128",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.i128()), serializers.option(serializers.i128()), serializers.u8(), serializers.bool()])]
        ])
      ],
      [
        "Pubkey",
        serializers.struct([
          [
            "fields",
            serializers.tuple([
              serializers.option(serializers.publicKey()),
              serializers.option(serializers.publicKey()),
              serializers.u8(),
              serializers.bool()
            ])
          ]
        ])
      ],
      [
        "Bytes",
        serializers.struct([
          [
            "fields",
            serializers.tuple([
              serializers.bytes({ size: serializers.u32() }),
              serializers.bytes({ size: serializers.u32() }),
              serializers.u8(),
              serializers.bool()
            ])
          ]
        ])
      ],
      [
        "Bool",
        serializers.struct([
          ["fields", serializers.tuple([serializers.option(serializers.bool()), serializers.option(serializers.bool()), serializers.u8(), serializers.bool()])]
        ])
      ]
    ],
    { description: "AssertionResult" }
  );
}
function assertionResult(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAssertionResult(kind, value) {
  return value.__kind === kind;
}
function getBubblegumTreeConfigAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "TreeCreator",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "TreeDelegate",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "TotalMintCapacity",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "NumMinted",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsPublic",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "IsDecompressible",
        serializers.struct([
          ["value", serializers.u8()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "BubblegumTreeConfigAssertion" }
  );
}
function bubblegumTreeConfigAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isBubblegumTreeConfigAssertion(kind, value) {
  return value.__kind === kind;
}
var ClockField = /* @__PURE__ */ ((ClockField3) => {
  ClockField3[ClockField3["Slot"] = 0] = "Slot";
  ClockField3[ClockField3["EpochStartTimestamp"] = 1] = "EpochStartTimestamp";
  ClockField3[ClockField3["Epoch"] = 2] = "Epoch";
  ClockField3[ClockField3["LeaderScheduleEpoch"] = 3] = "LeaderScheduleEpoch";
  ClockField3[ClockField3["UnixTimestamp"] = 4] = "UnixTimestamp";
  return ClockField3;
})(ClockField || {});
function getClockFieldSerializer() {
  return serializers.scalarEnum(ClockField, {
    description: "ClockField"
  });
}
function getDataValueSerializer() {
  return serializers.dataEnum(
    [
      [
        "Bool",
        serializers.struct([
          ["fields", serializers.tuple([serializers.bool()])]
        ])
      ],
      [
        "U8",
        serializers.struct([
          ["fields", serializers.tuple([serializers.u8()])]
        ])
      ],
      [
        "I8",
        serializers.struct([
          ["fields", serializers.tuple([serializers.i8()])]
        ])
      ],
      [
        "U16",
        serializers.struct([
          ["fields", serializers.tuple([serializers.u16()])]
        ])
      ],
      [
        "I16",
        serializers.struct([
          ["fields", serializers.tuple([serializers.i16()])]
        ])
      ],
      [
        "U32",
        serializers.struct([
          ["fields", serializers.tuple([serializers.u32()])]
        ])
      ],
      [
        "I32",
        serializers.struct([
          ["fields", serializers.tuple([serializers.i32()])]
        ])
      ],
      [
        "U64",
        serializers.struct([
          ["fields", serializers.tuple([serializers.u64()])]
        ])
      ],
      [
        "I64",
        serializers.struct([
          ["fields", serializers.tuple([serializers.i64()])]
        ])
      ],
      [
        "U128",
        serializers.struct([
          ["fields", serializers.tuple([serializers.u128()])]
        ])
      ],
      [
        "I128",
        serializers.struct([
          ["fields", serializers.tuple([serializers.i128()])]
        ])
      ],
      [
        "Bytes",
        serializers.struct([
          ["fields", serializers.tuple([serializers.bytes({ size: serializers.u32() })])]
        ])
      ],
      [
        "Pubkey",
        serializers.struct([
          ["fields", serializers.tuple([serializers.publicKey()])]
        ])
      ]
    ],
    { description: "DataValue" }
  );
}
function dataValue(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValue(kind, value) {
  return value.__kind === kind;
}
function getDataValueAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Bool",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "U8",
        serializers.struct([
          ["value", serializers.u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I8",
        serializers.struct([
          ["value", serializers.i8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U16",
        serializers.struct([
          ["value", serializers.u16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I16",
        serializers.struct([
          ["value", serializers.i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U32",
        serializers.struct([
          ["value", serializers.u32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I32",
        serializers.struct([
          ["value", serializers.i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U64",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I64",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U128",
        serializers.struct([
          ["value", serializers.u128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I128",
        serializers.struct([
          ["value", serializers.i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Bytes",
        serializers.struct([
          ["value", serializers.bytes({ size: serializers.u32() })],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Pubkey",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "DataValueAssertion" }
  );
}
function dataValueAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValueAssertion(kind, value) {
  return value.__kind === kind;
}
function getDataValueDeltaAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "U8",
        serializers.struct([
          ["value", serializers.i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I8",
        serializers.struct([
          ["value", serializers.i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U16",
        serializers.struct([
          ["value", serializers.i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I16",
        serializers.struct([
          ["value", serializers.i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U32",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I32",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U64",
        serializers.struct([
          ["value", serializers.i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I64",
        serializers.struct([
          ["value", serializers.i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Bytes",
        serializers.struct([
          ["length", serializers.u16()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "DataValueDeltaAssertion" }
  );
}
function dataValueDeltaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValueDeltaAssertion(kind, value) {
  return value.__kind === kind;
}
var EquatableOperator6 = /* @__PURE__ */ ((EquatableOperator16) => {
  EquatableOperator16[EquatableOperator16["Equal"] = 0] = "Equal";
  EquatableOperator16[EquatableOperator16["NotEqual"] = 1] = "NotEqual";
  return EquatableOperator16;
})(EquatableOperator6 || {});
function getEquatableOperatorSerializer() {
  return serializers.scalarEnum(EquatableOperator6, {
    description: "EquatableOperator"
  });
}
var IntegerOperator6 = /* @__PURE__ */ ((IntegerOperator14) => {
  IntegerOperator14[IntegerOperator14["Equal"] = 0] = "Equal";
  IntegerOperator14[IntegerOperator14["NotEqual"] = 1] = "NotEqual";
  IntegerOperator14[IntegerOperator14["GreaterThan"] = 2] = "GreaterThan";
  IntegerOperator14[IntegerOperator14["LessThan"] = 3] = "LessThan";
  IntegerOperator14[IntegerOperator14["GreaterThanOrEqual"] = 4] = "GreaterThanOrEqual";
  IntegerOperator14[IntegerOperator14["LessThanOrEqual"] = 5] = "LessThanOrEqual";
  IntegerOperator14[IntegerOperator14["Contains"] = 6] = "Contains";
  IntegerOperator14[IntegerOperator14["DoesNotContain"] = 7] = "DoesNotContain";
  return IntegerOperator14;
})(IntegerOperator6 || {});
function getIntegerOperatorSerializer() {
  return serializers.scalarEnum(IntegerOperator6, {
    description: "IntegerOperator"
  });
}
var KnownProgram2 = /* @__PURE__ */ ((KnownProgram3) => {
  KnownProgram3[KnownProgram3["System"] = 0] = "System";
  KnownProgram3[KnownProgram3["Token"] = 1] = "Token";
  KnownProgram3[KnownProgram3["Token2022"] = 2] = "Token2022";
  KnownProgram3[KnownProgram3["Rent"] = 3] = "Rent";
  KnownProgram3[KnownProgram3["Stake"] = 4] = "Stake";
  KnownProgram3[KnownProgram3["Vote"] = 5] = "Vote";
  KnownProgram3[KnownProgram3["BpfLoader"] = 6] = "BpfLoader";
  KnownProgram3[KnownProgram3["UpgradeableLoader"] = 7] = "UpgradeableLoader";
  KnownProgram3[KnownProgram3["SysvarConfig"] = 8] = "SysvarConfig";
  return KnownProgram3;
})(KnownProgram2 || {});
function getKnownProgramSerializer() {
  return serializers.scalarEnum(KnownProgram2, {
    description: "KnownProgram"
  });
}
var LogLevel = /* @__PURE__ */ ((LogLevel2) => {
  LogLevel2[LogLevel2["Silent"] = 0] = "Silent";
  LogLevel2[LogLevel2["PlaintextMessage"] = 1] = "PlaintextMessage";
  LogLevel2[LogLevel2["EncodedMessage"] = 2] = "EncodedMessage";
  LogLevel2[LogLevel2["EncodedNoop"] = 3] = "EncodedNoop";
  return LogLevel2;
})(LogLevel || {});
function getLogLevelSerializer() {
  return serializers.scalarEnum(LogLevel, {
    description: "LogLevel"
  });
}
function getMerkleTreeAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "VerifyLeaf",
        serializers.struct([
          ["leafIndex", serializers.u32()],
          ["leafHash", serializers.bytes({ size: 32 })]
        ])
      ]
    ],
    { description: "MerkleTreeAssertion" }
  );
}
function merkleTreeAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMerkleTreeAssertion(kind, value) {
  return value.__kind === kind;
}
function getMetaAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "RentExemptReserve",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "AuthorizedStaker",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "AuthorizedWithdrawer",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "LockupUnixTimestamp",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LockupEpoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LockupCustodian",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "MetaAssertion" }
  );
}
function metaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMetaAssertion(kind, value) {
  return value.__kind === kind;
}
function getMintAccountAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "MintAuthority",
        serializers.struct([
          ["value", serializers.option(serializers.publicKey())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Supply",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Decimals",
        serializers.struct([
          ["value", serializers.u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsInitialized",
        serializers.struct([
          ["value", serializers.bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "FreezeAuthority",
        serializers.struct(
          [
            ["value", serializers.option(serializers.publicKey())],
            ["operator", getEquatableOperatorSerializer()]
          ]
        )
      ]
    ],
    { description: "MintAccountAssertion" }
  );
}
function mintAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMintAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getStakeAccountAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "State",
        serializers.struct([
          ["value", getStakeStateTypeSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "MetaAssertion",
        serializers.struct([
          ["fields", serializers.tuple([getMetaAssertionSerializer()])]
        ])
      ],
      [
        "StakeAssertion",
        serializers.struct(
          [["fields", serializers.tuple([getStakeAssertionSerializer()])]]
        )
      ],
      [
        "StakeFlags",
        serializers.struct([
          ["value", serializers.u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ]
    ],
    { description: "StakeAccountAssertion" }
  );
}
function stakeAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isStakeAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getStakeAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "DelegationVoterPubkey",
        serializers.struct(
          [
            ["value", serializers.publicKey()],
            ["operator", getEquatableOperatorSerializer()]
          ]
        )
      ],
      [
        "DelegationStake",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DelegationActivationEpoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DelegationDeactivationEpoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "CreditsObserved",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ]
    ],
    { description: "StakeAssertion" }
  );
}
function stakeAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isStakeAssertion(kind, value) {
  return value.__kind === kind;
}
var StakeStateType2 = /* @__PURE__ */ ((StakeStateType3) => {
  StakeStateType3[StakeStateType3["Uninitialized"] = 0] = "Uninitialized";
  StakeStateType3[StakeStateType3["Initialized"] = 1] = "Initialized";
  StakeStateType3[StakeStateType3["Stake"] = 2] = "Stake";
  StakeStateType3[StakeStateType3["RewardsPool"] = 3] = "RewardsPool";
  return StakeStateType3;
})(StakeStateType2 || {});
function getStakeStateTypeSerializer() {
  return serializers.scalarEnum(StakeStateType2, {
    description: "StakeStateType"
  });
}
function getSysvarClockAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Slot",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "EpochStartTimestamp",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Epoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LeaderScheduleEpoch",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "UnixTimestamp",
        serializers.struct([
          ["value", serializers.i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ]
    ],
    { description: "SysvarClockAssertion" }
  );
}
function sysvarClockAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isSysvarClockAssertion(kind, value) {
  return value.__kind === kind;
}
function getTokenAccountAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Mint",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Owner",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Amount",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Delegate",
        serializers.struct([
          ["value", serializers.option(serializers.publicKey())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "State",
        serializers.struct([
          ["value", serializers.u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsNative",
        serializers.struct([
          ["value", serializers.option(serializers.u64())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "DelegatedAmount",
        serializers.struct([
          ["value", serializers.u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "CloseAuthority",
        serializers.struct(
          [
            ["value", serializers.option(serializers.publicKey())],
            ["operator", getEquatableOperatorSerializer()]
          ]
        )
      ],
      ["TokenAccountOwnerIsDerived", serializers.unit()]
    ],
    { description: "TokenAccountAssertion" }
  );
}
function tokenAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isTokenAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradableBufferAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "Authority",
        serializers.struct([
          ["value", serializers.option(serializers.publicKey())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "UpgradableBufferAssertion" }
  );
}
function upgradableBufferAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradableBufferAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradeableLoaderStateAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "State",
        serializers.struct([
          ["value", getUpgradeableLoaderStateTypeSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Buffer",
        serializers.struct([["fields", serializers.tuple([getUpgradableBufferAssertionSerializer()])]])
      ],
      [
        "Program",
        serializers.struct([["fields", serializers.tuple([getUpgradeableProgramAssertionSerializer()])]])
      ],
      [
        "ProgramData",
        serializers.struct([
          ["fields", serializers.tuple([getUpgradeableProgramDataAssertionSerializer()])]
        ])
      ]
    ],
    { description: "UpgradeableLoaderStateAssertion" }
  );
}
function upgradeableLoaderStateAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradeableLoaderStateAssertion(kind, value) {
  return value.__kind === kind;
}
var UpgradeableLoaderStateType2 = /* @__PURE__ */ ((UpgradeableLoaderStateType3) => {
  UpgradeableLoaderStateType3[UpgradeableLoaderStateType3["Uninitialized"] = 0] = "Uninitialized";
  UpgradeableLoaderStateType3[UpgradeableLoaderStateType3["Buffer"] = 1] = "Buffer";
  UpgradeableLoaderStateType3[UpgradeableLoaderStateType3["Program"] = 2] = "Program";
  UpgradeableLoaderStateType3[UpgradeableLoaderStateType3["ProgramData"] = 3] = "ProgramData";
  return UpgradeableLoaderStateType3;
})(UpgradeableLoaderStateType2 || {});
function getUpgradeableLoaderStateTypeSerializer() {
  return serializers.scalarEnum(UpgradeableLoaderStateType2, {
    description: "UpgradeableLoaderStateType"
  });
}
function getUpgradeableProgramAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "ProgramDataAddress",
        serializers.struct([
          ["value", serializers.publicKey()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ]
    ],
    { description: "UpgradeableProgramAssertion" }
  );
}
function upgradeableProgramAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradeableProgramAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradeableProgramDataAssertionSerializer() {
  return serializers.dataEnum(
    [
      [
        "UpgradeAuthority",
        serializers.struct([
          ["value", serializers.option(serializers.publicKey())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Slot",
        serializers.struct(
          [
            ["value", serializers.u64()],
            ["operator", getIntegerOperatorSerializer()]
          ]
        )
      ]
    ],
    { description: "UpgradeableProgramDataAssertion" }
  );
}
function upgradeableProgramDataAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradeableProgramDataAssertion(kind, value) {
  return value.__kind === kind;
}
function getWriteTypeSerializer() {
  return serializers.dataEnum(
    [
      [
        "AccountData",
        serializers.struct([
          ["offset", serializers.u16()],
          ["dataLength", serializers.u16()]
        ])
      ],
      [
        "AccountInfoField",
        serializers.struct([
          ["fields", serializers.tuple([getAccountInfoFieldSerializer()])]
        ])
      ],
      [
        "DataValue",
        serializers.struct([
          ["fields", serializers.tuple([getDataValueSerializer()])]
        ])
      ],
      [
        "Clock",
        serializers.struct([
          ["fields", serializers.tuple([getClockFieldSerializer()])]
        ])
      ]
    ],
    { description: "WriteType" }
  );
}
function writeType(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isWriteType(kind, value) {
  return value.__kind === kind;
}

// src/generated/instructions/assertAccountData.ts
function getAssertAccountDataInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["offset", serializers.u16()],
        ["assertion", getDataValueAssertionSerializer()]
      ],
      { description: "AssertAccountDataInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 2,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertAccountData(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertAccountDataInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountDeltaInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getAccountDeltaAssertionSerializer()]
      ],
      { description: "AssertAccountDeltaInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 3,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertAccountDelta(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    accountA: {
      index: 0,
      isWritable: false,
      value: input.accountA ?? null
    },
    accountB: {
      index: 1,
      isWritable: false,
      value: input.accountB ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertAccountDeltaInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountInfoInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getAccountInfoAssertionSerializer()]
      ],
      { description: "AssertAccountInfoInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 4,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertAccountInfo(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertAccountInfoInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountInfoMultiInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", serializers.array(getAccountInfoAssertionSerializer())]
      ],
      { description: "AssertAccountInfoMultiInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 5,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertAccountInfoMulti(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertAccountInfoMultiInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertBubblegumTreeConfigAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getBubblegumTreeConfigAssertionSerializer()]
      ],
      { description: "AssertBubblegumTreeConfigAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 16,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertBubblegumTreeConfigAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = {
    ...input
  };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertBubblegumTreeConfigAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMerkleTreeAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getMerkleTreeAssertionSerializer()]
      ],
      { description: "AssertMerkleTreeAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 15,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertMerkleTreeAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetMerkleTree: {
      index: 0,
      isWritable: false,
      value: input.targetMerkleTree ?? null
    },
    root: { index: 1, isWritable: false, value: input.root ?? null },
    splAccountCompression: {
      index: 2,
      isWritable: false,
      value: input.splAccountCompression ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertMerkleTreeAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMintAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getMintAccountAssertionSerializer()]
      ],
      { description: "AssertMintAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 6,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertMintAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertMintAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMintAccountMultiInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", serializers.array(getMintAccountAssertionSerializer())]
      ],
      { description: "AssertMintAccountMultiInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 7,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertMintAccountMulti(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertMintAccountMultiInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertStakeAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getStakeAccountAssertionSerializer()]
      ],
      { description: "AssertStakeAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 10,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertStakeAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertStakeAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertStakeAccountMultiInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", serializers.array(getStakeAccountAssertionSerializer())]
      ],
      { description: "AssertStakeAccountMultiInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 11,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertStakeAccountMulti(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertStakeAccountMultiInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertSysvarClockInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getSysvarClockAssertionSerializer()]
      ],
      { description: "AssertSysvarClockInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 14,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertSysvarClock(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {};
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertSysvarClockInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertTokenAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getTokenAccountAssertionSerializer()]
      ],
      { description: "AssertTokenAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 8,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertTokenAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertTokenAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertTokenAccountMultiInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", serializers.array(getTokenAccountAssertionSerializer())]
      ],
      { description: "AssertTokenAccountMultiInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 9,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertTokenAccountMulti(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertTokenAccountMultiInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertUpgradeableLoaderAccountInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertion", getUpgradeableLoaderStateAssertionSerializer()]
      ],
      { description: "AssertUpgradeableLoaderAccountInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 12,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertUpgradeableLoaderAccount(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = {
    ...input
  };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertUpgradeableLoaderAccountInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", serializers.array(getUpgradeableLoaderStateAssertionSerializer())]
      ],
      { description: "AssertUpgradeableLoaderAccountMultiInstructionData" }
    ),
    (value) => ({
      ...value,
      discriminator: 13,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function assertUpgradeableLoaderAccountMulti(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false,
      value: input.targetAccount ?? null
    }
  };
  const resolvedArgs = {
    ...input
  };
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getMemoryCloseInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["memoryId", serializers.u8()],
        ["memoryBump", serializers.u8()]
      ],
      { description: "MemoryCloseInstructionData" }
    ),
    (value) => ({ ...value, discriminator: 1 })
  );
}
function memoryClose(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    programId: {
      index: 0,
      isWritable: false,
      value: input.programId ?? null
    },
    payer: {
      index: 1,
      isWritable: true,
      value: input.payer ?? null
    },
    memory: {
      index: 2,
      isWritable: true,
      value: input.memory ?? null
    }
  };
  const resolvedArgs = { ...input };
  if (!resolvedAccounts.programId.value) {
    resolvedAccounts.programId.value = programId;
    resolvedAccounts.programId.isWritable = false;
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getMemoryCloseInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getMemoryWriteInstructionDataSerializer() {
  return serializers.mapSerializer(
    serializers.struct(
      [
        ["discriminator", serializers.u8()],
        ["memoryId", serializers.u8()],
        ["memoryBump", serializers.u8()],
        ["writeOffset", serializers.u16()],
        ["writeType", getWriteTypeSerializer()]
      ],
      { description: "MemoryWriteInstructionData" }
    ),
    (value) => ({ ...value, discriminator: 0, memoryId: value.memoryId ?? 0 })
  );
}
function memoryWrite(context, input) {
  const programId = context.programs.getPublicKey(
    "lighthouse",
    "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  );
  const resolvedAccounts = {
    programId: {
      index: 0,
      isWritable: false,
      value: input.programId ?? null
    },
    systemProgram: {
      index: 1,
      isWritable: false,
      value: input.systemProgram ?? null
    },
    payer: {
      index: 2,
      isWritable: true,
      value: input.payer ?? null
    },
    memory: {
      index: 3,
      isWritable: true,
      value: input.memory ?? null
    },
    sourceAccount: {
      index: 4,
      isWritable: false,
      value: input.sourceAccount ?? null
    }
  };
  const resolvedArgs = { ...input };
  if (!resolvedAccounts.programId.value) {
    resolvedAccounts.programId.value = programId;
    resolvedAccounts.programId.isWritable = false;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      "splSystem",
      "11111111111111111111111111111111"
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  const orderedAccounts = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    "programId",
    programId
  );
  const data = getMemoryWriteInstructionDataSerializer().serialize(
    resolvedArgs
  );
  const bytesCreatedOnChain = 0;
  return umi.transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}

// src/generated/programs/lighthouse.ts
var LIGHTHOUSE_PROGRAM_ID = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
function createLighthouseProgram() {
  return {
    name: "lighthouse",
    publicKey: LIGHTHOUSE_PROGRAM_ID,
    getErrorFromCode(code, cause) {
      return getLighthouseErrorFromCode(code, this, cause);
    },
    getErrorFromName(name, cause) {
      return getLighthouseErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    }
  };
}
function getLighthouseProgram(context, clusterFilter) {
  return context.programs.get("lighthouse", clusterFilter);
}
function getLighthouseProgramId(context, clusterFilter) {
  return context.programs.getPublicKey(
    "lighthouse",
    LIGHTHOUSE_PROGRAM_ID,
    clusterFilter
  );
}
function findMemoryPda(seeds, config = {}) {
  return web3_js.PublicKey.findProgramAddressSync(
    [
      Buffer.from("memory"),
      seeds.payer.toBuffer(),
      Buffer.from([seeds.memoryId])
    ],
    config.programAddress ?? new web3_js.PublicKey(LIGHTHOUSE_PROGRAM_ID)
  );
}

exports.AccountBorrowFailedError = AccountBorrowFailedError;
exports.AccountDiscriminatorValidationFailedError = AccountDiscriminatorValidationFailedError;
exports.AccountFundedValidationFailedError = AccountFundedValidationFailedError;
exports.AccountInfoField = AccountInfoField;
exports.AccountKeyMismatchError = AccountKeyMismatchError;
exports.AccountNotInitializedError = AccountNotInitializedError;
exports.AccountOwnerMismatchError = AccountOwnerMismatchError;
exports.AccountOwnerValidationFailedError = AccountOwnerValidationFailedError;
exports.AccountValidationFailedError = AccountValidationFailedError;
exports.AssertionFailedError = AssertionFailedError;
exports.BumpNotFoundError = BumpNotFoundError;
exports.ClockField = ClockField;
exports.CrossProgramInvokeViolationError = CrossProgramInvokeViolationError;
exports.EquatableOperator = EquatableOperator6;
exports.FailedToDeserializeError = FailedToDeserializeError;
exports.FailedToSerializeError = FailedToSerializeError;
exports.IndexOutOfBoundsError = IndexOutOfBoundsError;
exports.IntegerOperator = IntegerOperator6;
exports.InvalidInstructionDataError = InvalidInstructionDataError;
exports.KnownProgram = KnownProgram2;
exports.LIGHTHOUSE_PROGRAM_ID = LIGHTHOUSE_PROGRAM_ID;
exports.LogLevel = LogLevel;
exports.NotEnoughAccountsError = NotEnoughAccountsError;
exports.RangeOutOfBoundsError = RangeOutOfBoundsError;
exports.StakeStateType = StakeStateType2;
exports.UpgradeableLoaderStateType = UpgradeableLoaderStateType2;
exports.accountDeltaAssertion = accountDeltaAssertion;
exports.accountInfoAssertion = accountInfoAssertion;
exports.accountInfoDeltaAssertion = accountInfoDeltaAssertion;
exports.assertAccountData = assertAccountData;
exports.assertAccountDelta = assertAccountDelta;
exports.assertAccountInfo = assertAccountInfo;
exports.assertAccountInfoMulti = assertAccountInfoMulti;
exports.assertBubblegumTreeConfigAccount = assertBubblegumTreeConfigAccount;
exports.assertMerkleTreeAccount = assertMerkleTreeAccount;
exports.assertMintAccount = assertMintAccount;
exports.assertMintAccountMulti = assertMintAccountMulti;
exports.assertStakeAccount = assertStakeAccount;
exports.assertStakeAccountMulti = assertStakeAccountMulti;
exports.assertSysvarClock = assertSysvarClock;
exports.assertTokenAccount = assertTokenAccount;
exports.assertTokenAccountMulti = assertTokenAccountMulti;
exports.assertUpgradeableLoaderAccount = assertUpgradeableLoaderAccount;
exports.assertUpgradeableLoaderAccountMulti = assertUpgradeableLoaderAccountMulti;
exports.assertionResult = assertionResult;
exports.bubblegumTreeConfigAssertion = bubblegumTreeConfigAssertion;
exports.createLighthouseProgram = createLighthouseProgram;
exports.dataValue = dataValue;
exports.dataValueAssertion = dataValueAssertion;
exports.dataValueDeltaAssertion = dataValueDeltaAssertion;
exports.expectPda = expectPda;
exports.expectPublicKey = expectPublicKey;
exports.expectSome = expectSome;
exports.findMemoryPda = findMemoryPda;
exports.getAccountDeltaAssertionSerializer = getAccountDeltaAssertionSerializer;
exports.getAccountInfoAssertionSerializer = getAccountInfoAssertionSerializer;
exports.getAccountInfoDeltaAssertionSerializer = getAccountInfoDeltaAssertionSerializer;
exports.getAccountInfoFieldSerializer = getAccountInfoFieldSerializer;
exports.getAccountMetasAndSigners = getAccountMetasAndSigners;
exports.getAssertAccountDataInstructionDataSerializer = getAssertAccountDataInstructionDataSerializer;
exports.getAssertAccountDeltaInstructionDataSerializer = getAssertAccountDeltaInstructionDataSerializer;
exports.getAssertAccountInfoInstructionDataSerializer = getAssertAccountInfoInstructionDataSerializer;
exports.getAssertAccountInfoMultiInstructionDataSerializer = getAssertAccountInfoMultiInstructionDataSerializer;
exports.getAssertBubblegumTreeConfigAccountInstructionDataSerializer = getAssertBubblegumTreeConfigAccountInstructionDataSerializer;
exports.getAssertMerkleTreeAccountInstructionDataSerializer = getAssertMerkleTreeAccountInstructionDataSerializer;
exports.getAssertMintAccountInstructionDataSerializer = getAssertMintAccountInstructionDataSerializer;
exports.getAssertMintAccountMultiInstructionDataSerializer = getAssertMintAccountMultiInstructionDataSerializer;
exports.getAssertStakeAccountInstructionDataSerializer = getAssertStakeAccountInstructionDataSerializer;
exports.getAssertStakeAccountMultiInstructionDataSerializer = getAssertStakeAccountMultiInstructionDataSerializer;
exports.getAssertSysvarClockInstructionDataSerializer = getAssertSysvarClockInstructionDataSerializer;
exports.getAssertTokenAccountInstructionDataSerializer = getAssertTokenAccountInstructionDataSerializer;
exports.getAssertTokenAccountMultiInstructionDataSerializer = getAssertTokenAccountMultiInstructionDataSerializer;
exports.getAssertUpgradeableLoaderAccountInstructionDataSerializer = getAssertUpgradeableLoaderAccountInstructionDataSerializer;
exports.getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer = getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer;
exports.getAssertionResultSerializer = getAssertionResultSerializer;
exports.getBubblegumTreeConfigAssertionSerializer = getBubblegumTreeConfigAssertionSerializer;
exports.getClockFieldSerializer = getClockFieldSerializer;
exports.getDataValueAssertionSerializer = getDataValueAssertionSerializer;
exports.getDataValueDeltaAssertionSerializer = getDataValueDeltaAssertionSerializer;
exports.getDataValueSerializer = getDataValueSerializer;
exports.getEquatableOperatorSerializer = getEquatableOperatorSerializer;
exports.getIntegerOperatorSerializer = getIntegerOperatorSerializer;
exports.getKnownProgramSerializer = getKnownProgramSerializer;
exports.getLighthouseErrorFromCode = getLighthouseErrorFromCode;
exports.getLighthouseErrorFromName = getLighthouseErrorFromName;
exports.getLighthouseProgram = getLighthouseProgram;
exports.getLighthouseProgramId = getLighthouseProgramId;
exports.getLogLevelSerializer = getLogLevelSerializer;
exports.getMemoryCloseInstructionDataSerializer = getMemoryCloseInstructionDataSerializer;
exports.getMemoryWriteInstructionDataSerializer = getMemoryWriteInstructionDataSerializer;
exports.getMerkleTreeAssertionSerializer = getMerkleTreeAssertionSerializer;
exports.getMetaAssertionSerializer = getMetaAssertionSerializer;
exports.getMintAccountAssertionSerializer = getMintAccountAssertionSerializer;
exports.getStakeAccountAssertionSerializer = getStakeAccountAssertionSerializer;
exports.getStakeAssertionSerializer = getStakeAssertionSerializer;
exports.getStakeStateTypeSerializer = getStakeStateTypeSerializer;
exports.getSysvarClockAssertionSerializer = getSysvarClockAssertionSerializer;
exports.getTokenAccountAssertionSerializer = getTokenAccountAssertionSerializer;
exports.getUpgradableBufferAssertionSerializer = getUpgradableBufferAssertionSerializer;
exports.getUpgradeableLoaderStateAssertionSerializer = getUpgradeableLoaderStateAssertionSerializer;
exports.getUpgradeableLoaderStateTypeSerializer = getUpgradeableLoaderStateTypeSerializer;
exports.getUpgradeableProgramAssertionSerializer = getUpgradeableProgramAssertionSerializer;
exports.getUpgradeableProgramDataAssertionSerializer = getUpgradeableProgramDataAssertionSerializer;
exports.getWriteTypeSerializer = getWriteTypeSerializer;
exports.isAccountDeltaAssertion = isAccountDeltaAssertion;
exports.isAccountInfoAssertion = isAccountInfoAssertion;
exports.isAccountInfoDeltaAssertion = isAccountInfoDeltaAssertion;
exports.isAssertionResult = isAssertionResult;
exports.isBubblegumTreeConfigAssertion = isBubblegumTreeConfigAssertion;
exports.isDataValue = isDataValue;
exports.isDataValueAssertion = isDataValueAssertion;
exports.isDataValueDeltaAssertion = isDataValueDeltaAssertion;
exports.isMerkleTreeAssertion = isMerkleTreeAssertion;
exports.isMetaAssertion = isMetaAssertion;
exports.isMintAccountAssertion = isMintAccountAssertion;
exports.isStakeAccountAssertion = isStakeAccountAssertion;
exports.isStakeAssertion = isStakeAssertion;
exports.isSysvarClockAssertion = isSysvarClockAssertion;
exports.isTokenAccountAssertion = isTokenAccountAssertion;
exports.isUpgradableBufferAssertion = isUpgradableBufferAssertion;
exports.isUpgradeableLoaderStateAssertion = isUpgradeableLoaderStateAssertion;
exports.isUpgradeableProgramAssertion = isUpgradeableProgramAssertion;
exports.isUpgradeableProgramDataAssertion = isUpgradeableProgramDataAssertion;
exports.isWriteType = isWriteType;
exports.memoryClose = memoryClose;
exports.memoryWrite = memoryWrite;
exports.merkleTreeAssertion = merkleTreeAssertion;
exports.metaAssertion = metaAssertion;
exports.mintAccountAssertion = mintAccountAssertion;
exports.stakeAccountAssertion = stakeAccountAssertion;
exports.stakeAssertion = stakeAssertion;
exports.sysvarClockAssertion = sysvarClockAssertion;
exports.tokenAccountAssertion = tokenAccountAssertion;
exports.upgradableBufferAssertion = upgradableBufferAssertion;
exports.upgradeableLoaderStateAssertion = upgradeableLoaderStateAssertion;
exports.upgradeableProgramAssertion = upgradeableProgramAssertion;
exports.upgradeableProgramDataAssertion = upgradeableProgramDataAssertion;
exports.writeType = writeType;
//# sourceMappingURL=out.js.map
//# sourceMappingURL=index.js.map