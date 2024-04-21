import { ProgramError, publicKey, isPda, isSigner, transactionBuilder } from '@metaplex-foundation/umi';
import { dataEnum, struct, u16, u64, publicKey as publicKey$1, bool, bytes, option, i128, scalarEnum, tuple, u8, u32, u128, i8, i16, i32, i64, unit, mapSerializer, array } from '@metaplex-foundation/umi/serializers';
import { PublicKey } from '@solana/web3.js';

// src/generated/errors/lighthouse.ts
var codeToErrorMap = /* @__PURE__ */ new Map();
var nameToErrorMap = /* @__PURE__ */ new Map();
var InvalidInstructionDataError = class extends ProgramError {
  name = "InvalidInstructionData";
  code = 6e3;
  // 6000
  constructor(program, cause) {
    super("Invalid instruction", program, cause);
  }
};
codeToErrorMap.set(6e3, InvalidInstructionDataError);
nameToErrorMap.set("InvalidInstructionData", InvalidInstructionDataError);
var AssertionFailedError = class extends ProgramError {
  name = "AssertionFailed";
  code = 6001;
  // 6001
  constructor(program, cause) {
    super("AssertionFailed", program, cause);
  }
};
codeToErrorMap.set(6001, AssertionFailedError);
nameToErrorMap.set("AssertionFailed", AssertionFailedError);
var NotEnoughAccountsError = class extends ProgramError {
  name = "NotEnoughAccounts";
  code = 6002;
  // 6002
  constructor(program, cause) {
    super("NotEnoughAccounts", program, cause);
  }
};
codeToErrorMap.set(6002, NotEnoughAccountsError);
nameToErrorMap.set("NotEnoughAccounts", NotEnoughAccountsError);
var BumpNotFoundError = class extends ProgramError {
  name = "BumpNotFound";
  code = 6003;
  // 6003
  constructor(program, cause) {
    super("BumpNotFound", program, cause);
  }
};
codeToErrorMap.set(6003, BumpNotFoundError);
nameToErrorMap.set("BumpNotFound", BumpNotFoundError);
var AccountBorrowFailedError = class extends ProgramError {
  name = "AccountBorrowFailed";
  code = 6004;
  // 6004
  constructor(program, cause) {
    super("AccountBorrowFailed", program, cause);
  }
};
codeToErrorMap.set(6004, AccountBorrowFailedError);
nameToErrorMap.set("AccountBorrowFailed", AccountBorrowFailedError);
var RangeOutOfBoundsError = class extends ProgramError {
  name = "RangeOutOfBounds";
  code = 6005;
  // 6005
  constructor(program, cause) {
    super("RangeOutOfBounds", program, cause);
  }
};
codeToErrorMap.set(6005, RangeOutOfBoundsError);
nameToErrorMap.set("RangeOutOfBounds", RangeOutOfBoundsError);
var IndexOutOfBoundsError = class extends ProgramError {
  name = "IndexOutOfBounds";
  code = 6006;
  // 6006
  constructor(program, cause) {
    super("IndexOutOfBounds", program, cause);
  }
};
codeToErrorMap.set(6006, IndexOutOfBoundsError);
nameToErrorMap.set("IndexOutOfBounds", IndexOutOfBoundsError);
var FailedToDeserializeError = class extends ProgramError {
  name = "FailedToDeserialize";
  code = 6007;
  // 6007
  constructor(program, cause) {
    super("FailedToDeserialize", program, cause);
  }
};
codeToErrorMap.set(6007, FailedToDeserializeError);
nameToErrorMap.set("FailedToDeserialize", FailedToDeserializeError);
var FailedToSerializeError = class extends ProgramError {
  name = "FailedToSerialize";
  code = 6008;
  // 6008
  constructor(program, cause) {
    super("FailedToSerialize", program, cause);
  }
};
codeToErrorMap.set(6008, FailedToSerializeError);
nameToErrorMap.set("FailedToSerialize", FailedToSerializeError);
var AccountOwnerMismatchError = class extends ProgramError {
  name = "AccountOwnerMismatch";
  code = 6009;
  // 6009
  constructor(program, cause) {
    super("AccountOwnerMismatch", program, cause);
  }
};
codeToErrorMap.set(6009, AccountOwnerMismatchError);
nameToErrorMap.set("AccountOwnerMismatch", AccountOwnerMismatchError);
var AccountKeyMismatchError = class extends ProgramError {
  name = "AccountKeyMismatch";
  code = 6010;
  // 6010
  constructor(program, cause) {
    super("AccountKeyMismatch", program, cause);
  }
};
codeToErrorMap.set(6010, AccountKeyMismatchError);
nameToErrorMap.set("AccountKeyMismatch", AccountKeyMismatchError);
var AccountNotInitializedError = class extends ProgramError {
  name = "AccountNotInitialized";
  code = 6011;
  // 6011
  constructor(program, cause) {
    super("AccountNotInitialized", program, cause);
  }
};
codeToErrorMap.set(6011, AccountNotInitializedError);
nameToErrorMap.set("AccountNotInitialized", AccountNotInitializedError);
var AccountOwnerValidationFailedError = class extends ProgramError {
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
var AccountFundedValidationFailedError = class extends ProgramError {
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
var AccountDiscriminatorValidationFailedError = class extends ProgramError {
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
var AccountValidationFailedError = class extends ProgramError {
  name = "AccountValidationFailed";
  code = 6015;
  // 6015
  constructor(program, cause) {
    super("AccountValidaitonFailed", program, cause);
  }
};
codeToErrorMap.set(6015, AccountValidationFailedError);
nameToErrorMap.set("AccountValidationFailed", AccountValidationFailedError);
var CrossProgramInvokeViolationError = class extends ProgramError {
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
  return publicKey(value, false);
}
function expectPda(value) {
  if (!value || !Array.isArray(value) || !isPda(value)) {
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
    if (isSigner(account.value)) {
      signers.push(account.value);
    }
    keys.push({
      pubkey: publicKey(account.value, false),
      isSigner: isSigner(account.value),
      isWritable: account.isWritable
    });
  });
  return [keys, signers];
}
function getAccountDeltaAssertionSerializer() {
  return dataEnum(
    [
      [
        "AccountInfo",
        struct([
          ["aOffset", u16()],
          ["assertion", getAccountInfoDeltaAssertionSerializer()]
        ])
      ],
      [
        "Data",
        struct([
          ["aOffset", u16()],
          ["bOffset", u16()],
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
  return dataEnum(
    [
      [
        "Lamports",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DataLength",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Owner",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "KnownOwner",
        struct([
          ["value", getKnownProgramSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "RentEpoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsSigner",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "IsWritable",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Executable",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "VerifyDatahash",
        struct([
          ["expectedHash", bytes({ size: 32 })],
          ["start", option(u16())],
          ["length", option(u16())]
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
  return dataEnum(
    [
      [
        "Lamports",
        struct([
          ["value", i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DataLength",
        struct(
          [
            ["value", i128()],
            ["operator", getIntegerOperatorSerializer()]
          ]
        )
      ],
      [
        "Owner",
        struct([
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "RentEpoch",
        struct([
          ["value", i128()],
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
  return scalarEnum(AccountInfoField, {
    description: "AccountInfoField"
  });
}
function getAssertionResultSerializer() {
  return dataEnum(
    [
      [
        "U8",
        struct([
          ["fields", tuple([option(u8()), option(u8()), u8(), bool()])]
        ])
      ],
      [
        "U16",
        struct([
          ["fields", tuple([option(u16()), option(u16()), u8(), bool()])]
        ])
      ],
      [
        "U32",
        struct([
          ["fields", tuple([option(u32()), option(u32()), u8(), bool()])]
        ])
      ],
      [
        "U64",
        struct([
          ["fields", tuple([option(u64()), option(u64()), u8(), bool()])]
        ])
      ],
      [
        "U128",
        struct([
          ["fields", tuple([option(u128()), option(u128()), u8(), bool()])]
        ])
      ],
      [
        "I8",
        struct([
          ["fields", tuple([option(i8()), option(i8()), u8(), bool()])]
        ])
      ],
      [
        "I16",
        struct([
          ["fields", tuple([option(i16()), option(i16()), u8(), bool()])]
        ])
      ],
      [
        "I32",
        struct([
          ["fields", tuple([option(i32()), option(i32()), u8(), bool()])]
        ])
      ],
      [
        "I64",
        struct([
          ["fields", tuple([option(i64()), option(i64()), u8(), bool()])]
        ])
      ],
      [
        "I128",
        struct([
          ["fields", tuple([option(i128()), option(i128()), u8(), bool()])]
        ])
      ],
      [
        "Pubkey",
        struct([
          [
            "fields",
            tuple([
              option(publicKey$1()),
              option(publicKey$1()),
              u8(),
              bool()
            ])
          ]
        ])
      ],
      [
        "Bytes",
        struct([
          [
            "fields",
            tuple([
              bytes({ size: u32() }),
              bytes({ size: u32() }),
              u8(),
              bool()
            ])
          ]
        ])
      ],
      [
        "Bool",
        struct([
          ["fields", tuple([option(bool()), option(bool()), u8(), bool()])]
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
  return dataEnum(
    [
      [
        "TreeCreator",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "TreeDelegate",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "TotalMintCapacity",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "NumMinted",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsPublic",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "IsDecompressible",
        struct([
          ["value", u8()],
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
  return scalarEnum(ClockField, {
    description: "ClockField"
  });
}
function getDataValueSerializer() {
  return dataEnum(
    [
      [
        "Bool",
        struct([
          ["fields", tuple([bool()])]
        ])
      ],
      [
        "U8",
        struct([
          ["fields", tuple([u8()])]
        ])
      ],
      [
        "I8",
        struct([
          ["fields", tuple([i8()])]
        ])
      ],
      [
        "U16",
        struct([
          ["fields", tuple([u16()])]
        ])
      ],
      [
        "I16",
        struct([
          ["fields", tuple([i16()])]
        ])
      ],
      [
        "U32",
        struct([
          ["fields", tuple([u32()])]
        ])
      ],
      [
        "I32",
        struct([
          ["fields", tuple([i32()])]
        ])
      ],
      [
        "U64",
        struct([
          ["fields", tuple([u64()])]
        ])
      ],
      [
        "I64",
        struct([
          ["fields", tuple([i64()])]
        ])
      ],
      [
        "U128",
        struct([
          ["fields", tuple([u128()])]
        ])
      ],
      [
        "I128",
        struct([
          ["fields", tuple([i128()])]
        ])
      ],
      [
        "Bytes",
        struct([
          ["fields", tuple([bytes({ size: u32() })])]
        ])
      ],
      [
        "Pubkey",
        struct([
          ["fields", tuple([publicKey$1()])]
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
  return dataEnum(
    [
      [
        "Bool",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "U8",
        struct([
          ["value", u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I8",
        struct([
          ["value", i8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U16",
        struct([
          ["value", u16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I16",
        struct([
          ["value", i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U32",
        struct([
          ["value", u32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I32",
        struct([
          ["value", i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U64",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I64",
        struct([
          ["value", i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U128",
        struct([
          ["value", u128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I128",
        struct([
          ["value", i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Bytes",
        struct([
          ["value", bytes({ size: u32() })],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Pubkey",
        struct([
          ["value", publicKey$1()],
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
  return dataEnum(
    [
      [
        "U8",
        struct([
          ["value", i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I8",
        struct([
          ["value", i16()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U16",
        struct([
          ["value", i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I16",
        struct([
          ["value", i32()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U32",
        struct([
          ["value", i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I32",
        struct([
          ["value", i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "U64",
        struct([
          ["value", i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "I64",
        struct([
          ["value", i128()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Bytes",
        struct([
          ["length", u16()],
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
  return scalarEnum(EquatableOperator6, {
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
  return scalarEnum(IntegerOperator6, {
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
  return scalarEnum(KnownProgram2, {
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
  return scalarEnum(LogLevel, {
    description: "LogLevel"
  });
}
function getMerkleTreeAssertionSerializer() {
  return dataEnum(
    [
      [
        "VerifyLeaf",
        struct([
          ["leafIndex", u32()],
          ["leafHash", bytes({ size: 32 })]
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
  return dataEnum(
    [
      [
        "RentExemptReserve",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "AuthorizedStaker",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "AuthorizedWithdrawer",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "LockupUnixTimestamp",
        struct([
          ["value", i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LockupEpoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LockupCustodian",
        struct([
          ["value", publicKey$1()],
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
  return dataEnum(
    [
      [
        "MintAuthority",
        struct([
          ["value", option(publicKey$1())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Supply",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Decimals",
        struct([
          ["value", u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsInitialized",
        struct([
          ["value", bool()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "FreezeAuthority",
        struct(
          [
            ["value", option(publicKey$1())],
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
  return dataEnum(
    [
      [
        "State",
        struct([
          ["value", getStakeStateTypeSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "MetaAssertion",
        struct([
          ["fields", tuple([getMetaAssertionSerializer()])]
        ])
      ],
      [
        "StakeAssertion",
        struct(
          [["fields", tuple([getStakeAssertionSerializer()])]]
        )
      ],
      [
        "StakeFlags",
        struct([
          ["value", u8()],
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
  return dataEnum(
    [
      [
        "DelegationVoterPubkey",
        struct(
          [
            ["value", publicKey$1()],
            ["operator", getEquatableOperatorSerializer()]
          ]
        )
      ],
      [
        "DelegationStake",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DelegationActivationEpoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "DelegationDeactivationEpoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "CreditsObserved",
        struct([
          ["value", u64()],
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
  return scalarEnum(StakeStateType2, {
    description: "StakeStateType"
  });
}
function getSysvarClockAssertionSerializer() {
  return dataEnum(
    [
      [
        "Slot",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "EpochStartTimestamp",
        struct([
          ["value", i64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Epoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "LeaderScheduleEpoch",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "UnixTimestamp",
        struct([
          ["value", i64()],
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
  return dataEnum(
    [
      [
        "Mint",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Owner",
        struct([
          ["value", publicKey$1()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Amount",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "Delegate",
        struct([
          ["value", option(publicKey$1())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "State",
        struct([
          ["value", u8()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "IsNative",
        struct([
          ["value", option(u64())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "DelegatedAmount",
        struct([
          ["value", u64()],
          ["operator", getIntegerOperatorSerializer()]
        ])
      ],
      [
        "CloseAuthority",
        struct(
          [
            ["value", option(publicKey$1())],
            ["operator", getEquatableOperatorSerializer()]
          ]
        )
      ],
      ["TokenAccountOwnerIsDerived", unit()]
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
  return dataEnum(
    [
      [
        "Authority",
        struct([
          ["value", option(publicKey$1())],
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
  return dataEnum(
    [
      [
        "State",
        struct([
          ["value", getUpgradeableLoaderStateTypeSerializer()],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Buffer",
        struct([["fields", tuple([getUpgradableBufferAssertionSerializer()])]])
      ],
      [
        "Program",
        struct([["fields", tuple([getUpgradeableProgramAssertionSerializer()])]])
      ],
      [
        "ProgramData",
        struct([
          ["fields", tuple([getUpgradeableProgramDataAssertionSerializer()])]
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
  return scalarEnum(UpgradeableLoaderStateType2, {
    description: "UpgradeableLoaderStateType"
  });
}
function getUpgradeableProgramAssertionSerializer() {
  return dataEnum(
    [
      [
        "ProgramDataAddress",
        struct([
          ["value", publicKey$1()],
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
  return dataEnum(
    [
      [
        "UpgradeAuthority",
        struct([
          ["value", option(publicKey$1())],
          ["operator", getEquatableOperatorSerializer()]
        ])
      ],
      [
        "Slot",
        struct(
          [
            ["value", u64()],
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
  return dataEnum(
    [
      [
        "AccountData",
        struct([
          ["offset", u16()],
          ["dataLength", u16()]
        ])
      ],
      [
        "AccountInfoField",
        struct([
          ["fields", tuple([getAccountInfoFieldSerializer()])]
        ])
      ],
      [
        "DataValue",
        struct([
          ["fields", tuple([getDataValueSerializer()])]
        ])
      ],
      [
        "Clock",
        struct([
          ["fields", tuple([getClockFieldSerializer()])]
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
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["offset", u16()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountDeltaInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountInfoInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertAccountInfoMultiInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", array(getAccountInfoAssertionSerializer())]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertBubblegumTreeConfigAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMerkleTreeAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMintAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertMintAccountMultiInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", array(getMintAccountAssertionSerializer())]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertStakeAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertStakeAccountMultiInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", array(getStakeAccountAssertionSerializer())]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertSysvarClockInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertTokenAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertTokenAccountMultiInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", array(getTokenAccountAssertionSerializer())]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertUpgradeableLoaderAccountInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["logLevel", getLogLevelSerializer()],
        ["assertions", array(getUpgradeableLoaderStateAssertionSerializer())]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getMemoryCloseInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["memoryId", u8()],
        ["memoryBump", u8()]
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
  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain }
  ]);
}
function getMemoryWriteInstructionDataSerializer() {
  return mapSerializer(
    struct(
      [
        ["discriminator", u8()],
        ["memoryId", u8()],
        ["memoryBump", u8()],
        ["writeOffset", u16()],
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
  return transactionBuilder([
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
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("memory"),
      seeds.payer.toBuffer(),
      Buffer.from([seeds.memoryId])
    ],
    config.programAddress ?? new PublicKey(LIGHTHOUSE_PROGRAM_ID)
  );
}

export { AccountBorrowFailedError, AccountDiscriminatorValidationFailedError, AccountFundedValidationFailedError, AccountInfoField, AccountKeyMismatchError, AccountNotInitializedError, AccountOwnerMismatchError, AccountOwnerValidationFailedError, AccountValidationFailedError, AssertionFailedError, BumpNotFoundError, ClockField, CrossProgramInvokeViolationError, EquatableOperator6 as EquatableOperator, FailedToDeserializeError, FailedToSerializeError, IndexOutOfBoundsError, IntegerOperator6 as IntegerOperator, InvalidInstructionDataError, KnownProgram2 as KnownProgram, LIGHTHOUSE_PROGRAM_ID, LogLevel, NotEnoughAccountsError, RangeOutOfBoundsError, StakeStateType2 as StakeStateType, UpgradeableLoaderStateType2 as UpgradeableLoaderStateType, accountDeltaAssertion, accountInfoAssertion, accountInfoDeltaAssertion, assertAccountData, assertAccountDelta, assertAccountInfo, assertAccountInfoMulti, assertBubblegumTreeConfigAccount, assertMerkleTreeAccount, assertMintAccount, assertMintAccountMulti, assertStakeAccount, assertStakeAccountMulti, assertSysvarClock, assertTokenAccount, assertTokenAccountMulti, assertUpgradeableLoaderAccount, assertUpgradeableLoaderAccountMulti, assertionResult, bubblegumTreeConfigAssertion, createLighthouseProgram, dataValue, dataValueAssertion, dataValueDeltaAssertion, expectPda, expectPublicKey, expectSome, findMemoryPda, getAccountDeltaAssertionSerializer, getAccountInfoAssertionSerializer, getAccountInfoDeltaAssertionSerializer, getAccountInfoFieldSerializer, getAccountMetasAndSigners, getAssertAccountDataInstructionDataSerializer, getAssertAccountDeltaInstructionDataSerializer, getAssertAccountInfoInstructionDataSerializer, getAssertAccountInfoMultiInstructionDataSerializer, getAssertBubblegumTreeConfigAccountInstructionDataSerializer, getAssertMerkleTreeAccountInstructionDataSerializer, getAssertMintAccountInstructionDataSerializer, getAssertMintAccountMultiInstructionDataSerializer, getAssertStakeAccountInstructionDataSerializer, getAssertStakeAccountMultiInstructionDataSerializer, getAssertSysvarClockInstructionDataSerializer, getAssertTokenAccountInstructionDataSerializer, getAssertTokenAccountMultiInstructionDataSerializer, getAssertUpgradeableLoaderAccountInstructionDataSerializer, getAssertUpgradeableLoaderAccountMultiInstructionDataSerializer, getAssertionResultSerializer, getBubblegumTreeConfigAssertionSerializer, getClockFieldSerializer, getDataValueAssertionSerializer, getDataValueDeltaAssertionSerializer, getDataValueSerializer, getEquatableOperatorSerializer, getIntegerOperatorSerializer, getKnownProgramSerializer, getLighthouseErrorFromCode, getLighthouseErrorFromName, getLighthouseProgram, getLighthouseProgramId, getLogLevelSerializer, getMemoryCloseInstructionDataSerializer, getMemoryWriteInstructionDataSerializer, getMerkleTreeAssertionSerializer, getMetaAssertionSerializer, getMintAccountAssertionSerializer, getStakeAccountAssertionSerializer, getStakeAssertionSerializer, getStakeStateTypeSerializer, getSysvarClockAssertionSerializer, getTokenAccountAssertionSerializer, getUpgradableBufferAssertionSerializer, getUpgradeableLoaderStateAssertionSerializer, getUpgradeableLoaderStateTypeSerializer, getUpgradeableProgramAssertionSerializer, getUpgradeableProgramDataAssertionSerializer, getWriteTypeSerializer, isAccountDeltaAssertion, isAccountInfoAssertion, isAccountInfoDeltaAssertion, isAssertionResult, isBubblegumTreeConfigAssertion, isDataValue, isDataValueAssertion, isDataValueDeltaAssertion, isMerkleTreeAssertion, isMetaAssertion, isMintAccountAssertion, isStakeAccountAssertion, isStakeAssertion, isSysvarClockAssertion, isTokenAccountAssertion, isUpgradableBufferAssertion, isUpgradeableLoaderStateAssertion, isUpgradeableProgramAssertion, isUpgradeableProgramDataAssertion, isWriteType, memoryClose, memoryWrite, merkleTreeAssertion, metaAssertion, mintAccountAssertion, stakeAccountAssertion, stakeAssertion, sysvarClockAssertion, tokenAccountAssertion, upgradableBufferAssertion, upgradeableLoaderStateAssertion, upgradeableProgramAssertion, upgradeableProgramDataAssertion, writeType };
//# sourceMappingURL=out.js.map
//# sourceMappingURL=index.mjs.map