'use strict';

var codecs = require('@solana/codecs');
var instructions = require('@solana/instructions');
var addresses = require('@solana/addresses');
var signers = require('@solana/signers');

// env-shim.ts
var __DEV__ = /* @__PURE__ */ (() => process["env"].NODE_ENV === "development")();

// src/generated/errors/lighthouse.ts
var LighthouseProgramErrorCode = /* @__PURE__ */ ((LighthouseProgramErrorCode3) => {
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["INVALID_INSTRUCTION_DATA"] = 6e3] = "INVALID_INSTRUCTION_DATA";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ASSERTION_FAILED"] = 6001] = "ASSERTION_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["NOT_ENOUGH_ACCOUNTS"] = 6002] = "NOT_ENOUGH_ACCOUNTS";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["BUMP_NOT_FOUND"] = 6003] = "BUMP_NOT_FOUND";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_BORROW_FAILED"] = 6004] = "ACCOUNT_BORROW_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["RANGE_OUT_OF_BOUNDS"] = 6005] = "RANGE_OUT_OF_BOUNDS";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["INDEX_OUT_OF_BOUNDS"] = 6006] = "INDEX_OUT_OF_BOUNDS";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["FAILED_TO_DESERIALIZE"] = 6007] = "FAILED_TO_DESERIALIZE";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["FAILED_TO_SERIALIZE"] = 6008] = "FAILED_TO_SERIALIZE";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_OWNER_MISMATCH"] = 6009] = "ACCOUNT_OWNER_MISMATCH";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_KEY_MISMATCH"] = 6010] = "ACCOUNT_KEY_MISMATCH";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_NOT_INITIALIZED"] = 6011] = "ACCOUNT_NOT_INITIALIZED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_OWNER_VALIDATION_FAILED"] = 6012] = "ACCOUNT_OWNER_VALIDATION_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_FUNDED_VALIDATION_FAILED"] = 6013] = "ACCOUNT_FUNDED_VALIDATION_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_DISCRIMINATOR_VALIDATION_FAILED"] = 6014] = "ACCOUNT_DISCRIMINATOR_VALIDATION_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["ACCOUNT_VALIDATION_FAILED"] = 6015] = "ACCOUNT_VALIDATION_FAILED";
  LighthouseProgramErrorCode3[LighthouseProgramErrorCode3["CROSS_PROGRAM_INVOKE_VIOLATION"] = 6016] = "CROSS_PROGRAM_INVOKE_VIOLATION";
  return LighthouseProgramErrorCode3;
})(LighthouseProgramErrorCode || {});
var LighthouseProgramError = class extends Error {
  name = "LighthouseProgramError";
  code;
  cause;
  constructor(code, name, message, cause) {
    super(`${name} (${code}): ${message}`);
    this.code = code;
    this.cause = cause;
  }
};
var lighthouseProgramErrorCodeMap;
if (__DEV__) {
  lighthouseProgramErrorCodeMap = {
    [6e3 /* INVALID_INSTRUCTION_DATA */]: [
      "InvalidInstructionData",
      `Invalid instruction`
    ],
    [6001 /* ASSERTION_FAILED */]: [
      "AssertionFailed",
      `AssertionFailed`
    ],
    [6002 /* NOT_ENOUGH_ACCOUNTS */]: [
      "NotEnoughAccounts",
      `NotEnoughAccounts`
    ],
    [6003 /* BUMP_NOT_FOUND */]: [
      "BumpNotFound",
      `BumpNotFound`
    ],
    [6004 /* ACCOUNT_BORROW_FAILED */]: [
      "AccountBorrowFailed",
      `AccountBorrowFailed`
    ],
    [6005 /* RANGE_OUT_OF_BOUNDS */]: [
      "RangeOutOfBounds",
      `RangeOutOfBounds`
    ],
    [6006 /* INDEX_OUT_OF_BOUNDS */]: [
      "IndexOutOfBounds",
      `IndexOutOfBounds`
    ],
    [6007 /* FAILED_TO_DESERIALIZE */]: [
      "FailedToDeserialize",
      `FailedToDeserialize`
    ],
    [6008 /* FAILED_TO_SERIALIZE */]: [
      "FailedToSerialize",
      `FailedToSerialize`
    ],
    [6009 /* ACCOUNT_OWNER_MISMATCH */]: [
      "AccountOwnerMismatch",
      `AccountOwnerMismatch`
    ],
    [6010 /* ACCOUNT_KEY_MISMATCH */]: [
      "AccountKeyMismatch",
      `AccountKeyMismatch`
    ],
    [6011 /* ACCOUNT_NOT_INITIALIZED */]: [
      "AccountNotInitialized",
      `AccountNotInitialized`
    ],
    [6012 /* ACCOUNT_OWNER_VALIDATION_FAILED */]: [
      "AccountOwnerValidationFailed",
      `AccountOwnerValidationFailed`
    ],
    [6013 /* ACCOUNT_FUNDED_VALIDATION_FAILED */]: [
      "AccountFundedValidationFailed",
      `AccountFundedValidationFailed`
    ],
    [6014 /* ACCOUNT_DISCRIMINATOR_VALIDATION_FAILED */]: [
      "AccountDiscriminatorValidationFailed",
      `AccountDiscriminatorValidationFailed`
    ],
    [6015 /* ACCOUNT_VALIDATION_FAILED */]: [
      "AccountValidationFailed",
      `AccountValidaitonFailed`
    ],
    [6016 /* CROSS_PROGRAM_INVOKE_VIOLATION */]: [
      "CrossProgramInvokeViolation",
      `CrossProgramInvokeViolation`
    ]
  };
}
function getLighthouseProgramErrorFromCode(code, cause) {
  if (__DEV__) {
    return new LighthouseProgramError(
      code,
      ...lighthouseProgramErrorCodeMap[code],
      cause
    );
  }
  return new LighthouseProgramError(
    code,
    "Unknown",
    "Error message not available in production bundles. Compile with __DEV__ set to true to see more information.",
    cause
  );
}
function expectSome(value) {
  if (value == null) {
    throw new Error("Expected a value but received null or undefined.");
  }
  return value;
}
function expectAddress(value) {
  if (!value) {
    throw new Error("Expected a Address.");
  }
  if (typeof value === "object" && "address" in value) {
    return value.address;
  }
  if (Array.isArray(value)) {
    return value[0];
  }
  return value;
}
function expectProgramDerivedAddress(value) {
  if (!value || !Array.isArray(value) || !addresses.isProgramDerivedAddress(value)) {
    throw new Error("Expected a ProgramDerivedAddress.");
  }
  return value;
}
function expectTransactionSigner(value) {
  if (!value || !isTransactionSigner(value)) {
    throw new Error("Expected a TransactionSigner.");
  }
  return value;
}
function accountMetaWithDefault(account, role) {
  if (account === void 0)
    return void 0;
  return typeof account === "string" ? { address: account, role } : account;
}
function getAccountMetasWithSigners(accounts, optionalAccountStrategy, programAddress) {
  const accountMetas = {};
  Object.keys(accounts).forEach((key) => {
    const account = accounts[key];
    if (!account.value) {
      if (optionalAccountStrategy === "omitted")
        return;
      accountMetas[key] = {
        address: programAddress,
        role: instructions.AccountRole.READONLY
      };
      return;
    }
    const writableRole = account.isWritable ? instructions.AccountRole.WRITABLE : instructions.AccountRole.READONLY;
    accountMetas[key] = Object.freeze({
      address: expectAddress(account.value),
      role: isTransactionSigner(account.value) ? instructions.upgradeRoleToSigner(writableRole) : writableRole,
      ...isTransactionSigner(account.value) ? { signer: account.value } : {}
    });
  });
  return accountMetas;
}
function isTransactionSigner(value) {
  return !!value && typeof value === "object" && "address" in value && signers.isTransactionSigner(value);
}
function memcmp(data, bytes, offset) {
  const slice = data.slice(offset, offset + bytes.length);
  if (slice.length !== bytes.length)
    return false;
  return bytes.every((b, i) => b === slice[i]);
}
function getAccountDeltaAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "AccountInfo",
      codecs.getStructEncoder([
        ["aOffset", codecs.getU16Encoder()],
        ["assertion", getAccountInfoDeltaAssertionEncoder()]
      ])
    ],
    [
      "Data",
      codecs.getStructEncoder([
        ["aOffset", codecs.getU16Encoder()],
        ["bOffset", codecs.getU16Encoder()],
        ["assertion", getDataValueDeltaAssertionEncoder()]
      ])
    ]
  ]);
}
function getAccountDeltaAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "AccountInfo",
      codecs.getStructDecoder([
        ["aOffset", codecs.getU16Decoder()],
        ["assertion", getAccountInfoDeltaAssertionDecoder()]
      ])
    ],
    [
      "Data",
      codecs.getStructDecoder([
        ["aOffset", codecs.getU16Decoder()],
        ["bOffset", codecs.getU16Decoder()],
        ["assertion", getDataValueDeltaAssertionDecoder()]
      ])
    ]
  ]);
}
function getAccountDeltaAssertionCodec() {
  return codecs.combineCodec(
    getAccountDeltaAssertionEncoder(),
    getAccountDeltaAssertionDecoder()
  );
}
function accountDeltaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAccountDeltaAssertion(kind, value) {
  return value.__kind === kind;
}
function getAccountInfoAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Lamports",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DataLength",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Owner",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "KnownOwner",
      codecs.getStructEncoder([
        ["value", getKnownProgramEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "RentEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsSigner",
      codecs.getStructEncoder([
        ["value", codecs.getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "IsWritable",
      codecs.getStructEncoder([
        ["value", codecs.getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Executable",
      codecs.getStructEncoder([
        ["value", codecs.getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "VerifyDatahash",
      codecs.getStructEncoder([
        ["expectedHash", codecs.getBytesEncoder({ size: 32 })],
        ["start", codecs.getOptionEncoder(codecs.getU16Encoder())],
        ["length", codecs.getOptionEncoder(codecs.getU16Encoder())]
      ])
    ]
  ]);
}
function getAccountInfoAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Lamports",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DataLength",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Owner",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "KnownOwner",
      codecs.getStructDecoder([
        ["value", getKnownProgramDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "RentEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsSigner",
      codecs.getStructDecoder([
        ["value", codecs.getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "IsWritable",
      codecs.getStructDecoder([
        ["value", codecs.getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Executable",
      codecs.getStructDecoder([
        ["value", codecs.getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "VerifyDatahash",
      codecs.getStructDecoder([
        ["expectedHash", codecs.getBytesDecoder({ size: 32 })],
        ["start", codecs.getOptionDecoder(codecs.getU16Decoder())],
        ["length", codecs.getOptionDecoder(codecs.getU16Decoder())]
      ])
    ]
  ]);
}
function getAccountInfoAssertionCodec() {
  return codecs.combineCodec(
    getAccountInfoAssertionEncoder(),
    getAccountInfoAssertionDecoder()
  );
}
function accountInfoAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAccountInfoAssertion(kind, value) {
  return value.__kind === kind;
}
function getAccountInfoDeltaAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Lamports",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DataLength",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    ["Owner", codecs.getStructEncoder([["operator", getEquatableOperatorEncoder()]])],
    [
      "RentEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getAccountInfoDeltaAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Lamports",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DataLength",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    ["Owner", codecs.getStructDecoder([["operator", getEquatableOperatorDecoder()]])],
    [
      "RentEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getAccountInfoDeltaAssertionCodec() {
  return codecs.combineCodec(
    getAccountInfoDeltaAssertionEncoder(),
    getAccountInfoDeltaAssertionDecoder()
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
function getAccountInfoFieldEncoder() {
  return codecs.getScalarEnumEncoder(AccountInfoField);
}
function getAccountInfoFieldDecoder() {
  return codecs.getScalarEnumDecoder(AccountInfoField);
}
function getAccountInfoFieldCodec() {
  return codecs.combineCodec(
    getAccountInfoFieldEncoder(),
    getAccountInfoFieldDecoder()
  );
}
function getAssertionResultEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "U8",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getU8Encoder()),
            codecs.getOptionEncoder(codecs.getU8Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U16",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getU16Encoder()),
            codecs.getOptionEncoder(codecs.getU16Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U32",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getU32Encoder()),
            codecs.getOptionEncoder(codecs.getU32Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U64",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getU64Encoder()),
            codecs.getOptionEncoder(codecs.getU64Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U128",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getU128Encoder()),
            codecs.getOptionEncoder(codecs.getU128Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I8",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getI8Encoder()),
            codecs.getOptionEncoder(codecs.getI8Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I16",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getI16Encoder()),
            codecs.getOptionEncoder(codecs.getI16Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I32",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getI32Encoder()),
            codecs.getOptionEncoder(codecs.getI32Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I64",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getI64Encoder()),
            codecs.getOptionEncoder(codecs.getI64Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I128",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getI128Encoder()),
            codecs.getOptionEncoder(codecs.getI128Encoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(addresses.getAddressEncoder()),
            codecs.getOptionEncoder(addresses.getAddressEncoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Bytes",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getBytesEncoder({ size: codecs.getU32Encoder() }),
            codecs.getBytesEncoder({ size: codecs.getU32Encoder() }),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Bool",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([
            codecs.getOptionEncoder(codecs.getBooleanEncoder()),
            codecs.getOptionEncoder(codecs.getBooleanEncoder()),
            codecs.getU8Encoder(),
            codecs.getBooleanEncoder()
          ])
        ]
      ])
    ]
  ]);
}
function getAssertionResultDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "U8",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getU8Decoder()),
            codecs.getOptionDecoder(codecs.getU8Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U16",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getU16Decoder()),
            codecs.getOptionDecoder(codecs.getU16Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U32",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getU32Decoder()),
            codecs.getOptionDecoder(codecs.getU32Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U64",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getU64Decoder()),
            codecs.getOptionDecoder(codecs.getU64Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U128",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getU128Decoder()),
            codecs.getOptionDecoder(codecs.getU128Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I8",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getI8Decoder()),
            codecs.getOptionDecoder(codecs.getI8Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I16",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getI16Decoder()),
            codecs.getOptionDecoder(codecs.getI16Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I32",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getI32Decoder()),
            codecs.getOptionDecoder(codecs.getI32Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I64",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getI64Decoder()),
            codecs.getOptionDecoder(codecs.getI64Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I128",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getI128Decoder()),
            codecs.getOptionDecoder(codecs.getI128Decoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(addresses.getAddressDecoder()),
            codecs.getOptionDecoder(addresses.getAddressDecoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Bytes",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getBytesDecoder({ size: codecs.getU32Decoder() }),
            codecs.getBytesDecoder({ size: codecs.getU32Decoder() }),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Bool",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([
            codecs.getOptionDecoder(codecs.getBooleanDecoder()),
            codecs.getOptionDecoder(codecs.getBooleanDecoder()),
            codecs.getU8Decoder(),
            codecs.getBooleanDecoder()
          ])
        ]
      ])
    ]
  ]);
}
function getAssertionResultCodec() {
  return codecs.combineCodec(getAssertionResultEncoder(), getAssertionResultDecoder());
}
function assertionResult(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isAssertionResult(kind, value) {
  return value.__kind === kind;
}
var ByteSliceOperator = /* @__PURE__ */ ((ByteSliceOperator4) => {
  ByteSliceOperator4[ByteSliceOperator4["Equal"] = 0] = "Equal";
  ByteSliceOperator4[ByteSliceOperator4["NotEqual"] = 1] = "NotEqual";
  return ByteSliceOperator4;
})(ByteSliceOperator || {});
function getByteSliceOperatorEncoder() {
  return codecs.getScalarEnumEncoder(ByteSliceOperator);
}
function getByteSliceOperatorDecoder() {
  return codecs.getScalarEnumDecoder(ByteSliceOperator);
}
function getByteSliceOperatorCodec() {
  return codecs.combineCodec(
    getByteSliceOperatorEncoder(),
    getByteSliceOperatorDecoder()
  );
}
var ClockField = /* @__PURE__ */ ((ClockField3) => {
  ClockField3[ClockField3["Slot"] = 0] = "Slot";
  ClockField3[ClockField3["EpochStartTimestamp"] = 1] = "EpochStartTimestamp";
  ClockField3[ClockField3["Epoch"] = 2] = "Epoch";
  ClockField3[ClockField3["LeaderScheduleEpoch"] = 3] = "LeaderScheduleEpoch";
  ClockField3[ClockField3["UnixTimestamp"] = 4] = "UnixTimestamp";
  return ClockField3;
})(ClockField || {});
function getClockFieldEncoder() {
  return codecs.getScalarEnumEncoder(ClockField);
}
function getClockFieldDecoder() {
  return codecs.getScalarEnumDecoder(ClockField);
}
function getClockFieldCodec() {
  return codecs.combineCodec(getClockFieldEncoder(), getClockFieldDecoder());
}
function getDataValueEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Bool",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getBooleanEncoder()])]])
    ],
    ["U8", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getU8Encoder()])]])],
    ["I8", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getI8Encoder()])]])],
    ["U16", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getU16Encoder()])]])],
    ["I16", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getI16Encoder()])]])],
    ["U32", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getU32Encoder()])]])],
    ["I32", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getI32Encoder()])]])],
    ["U64", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getU64Encoder()])]])],
    ["I64", codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getI64Encoder()])]])],
    [
      "U128",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getU128Encoder()])]])
    ],
    [
      "I128",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([codecs.getI128Encoder()])]])
    ],
    [
      "Bytes",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([codecs.getBytesEncoder({ size: codecs.getU32Encoder() })])
        ]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([addresses.getAddressEncoder()])]])
    ]
  ]);
}
function getDataValueDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Bool",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getBooleanDecoder()])]])
    ],
    ["U8", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getU8Decoder()])]])],
    ["I8", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getI8Decoder()])]])],
    ["U16", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getU16Decoder()])]])],
    ["I16", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getI16Decoder()])]])],
    ["U32", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getU32Decoder()])]])],
    ["I32", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getI32Decoder()])]])],
    ["U64", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getU64Decoder()])]])],
    ["I64", codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getI64Decoder()])]])],
    [
      "U128",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getU128Decoder()])]])
    ],
    [
      "I128",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([codecs.getI128Decoder()])]])
    ],
    [
      "Bytes",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([codecs.getBytesDecoder({ size: codecs.getU32Decoder() })])
        ]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([addresses.getAddressDecoder()])]])
    ]
  ]);
}
function getDataValueCodec() {
  return codecs.combineCodec(getDataValueEncoder(), getDataValueDecoder());
}
function dataValue(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValue(kind, value) {
  return value.__kind === kind;
}
function getDataValueAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Bool",
      codecs.getStructEncoder([
        ["value", codecs.getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "U8",
      codecs.getStructEncoder([
        ["value", codecs.getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I8",
      codecs.getStructEncoder([
        ["value", codecs.getI8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U16",
      codecs.getStructEncoder([
        ["value", codecs.getU16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I16",
      codecs.getStructEncoder([
        ["value", codecs.getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U32",
      codecs.getStructEncoder([
        ["value", codecs.getU32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I32",
      codecs.getStructEncoder([
        ["value", codecs.getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U64",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I64",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U128",
      codecs.getStructEncoder([
        ["value", codecs.getU128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I128",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Bytes",
      codecs.getStructEncoder([
        ["value", codecs.getBytesEncoder({ size: codecs.getU32Encoder() })],
        ["operator", getByteSliceOperatorEncoder()]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getDataValueAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Bool",
      codecs.getStructDecoder([
        ["value", codecs.getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "U8",
      codecs.getStructDecoder([
        ["value", codecs.getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I8",
      codecs.getStructDecoder([
        ["value", codecs.getI8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U16",
      codecs.getStructDecoder([
        ["value", codecs.getU16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I16",
      codecs.getStructDecoder([
        ["value", codecs.getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U32",
      codecs.getStructDecoder([
        ["value", codecs.getU32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I32",
      codecs.getStructDecoder([
        ["value", codecs.getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U64",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I64",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U128",
      codecs.getStructDecoder([
        ["value", codecs.getU128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I128",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Bytes",
      codecs.getStructDecoder([
        ["value", codecs.getBytesDecoder({ size: codecs.getU32Decoder() })],
        ["operator", getByteSliceOperatorDecoder()]
      ])
    ],
    [
      "Pubkey",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getDataValueAssertionCodec() {
  return codecs.combineCodec(
    getDataValueAssertionEncoder(),
    getDataValueAssertionDecoder()
  );
}
function dataValueAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValueAssertion(kind, value) {
  return value.__kind === kind;
}
function getDataValueDeltaAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "U8",
      codecs.getStructEncoder([
        ["value", codecs.getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I8",
      codecs.getStructEncoder([
        ["value", codecs.getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U16",
      codecs.getStructEncoder([
        ["value", codecs.getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I16",
      codecs.getStructEncoder([
        ["value", codecs.getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U32",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I32",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U64",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I64",
      codecs.getStructEncoder([
        ["value", codecs.getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Bytes",
      codecs.getStructEncoder([
        ["length", codecs.getU16Encoder()],
        ["operator", getByteSliceOperatorEncoder()]
      ])
    ]
  ]);
}
function getDataValueDeltaAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "U8",
      codecs.getStructDecoder([
        ["value", codecs.getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I8",
      codecs.getStructDecoder([
        ["value", codecs.getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U16",
      codecs.getStructDecoder([
        ["value", codecs.getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I16",
      codecs.getStructDecoder([
        ["value", codecs.getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U32",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I32",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U64",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I64",
      codecs.getStructDecoder([
        ["value", codecs.getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Bytes",
      codecs.getStructDecoder([
        ["length", codecs.getU16Decoder()],
        ["operator", getByteSliceOperatorDecoder()]
      ])
    ]
  ]);
}
function getDataValueDeltaAssertionCodec() {
  return codecs.combineCodec(
    getDataValueDeltaAssertionEncoder(),
    getDataValueDeltaAssertionDecoder()
  );
}
function dataValueDeltaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValueDeltaAssertion(kind, value) {
  return value.__kind === kind;
}
var EquatableOperator4 = /* @__PURE__ */ ((EquatableOperator14) => {
  EquatableOperator14[EquatableOperator14["Equal"] = 0] = "Equal";
  EquatableOperator14[EquatableOperator14["NotEqual"] = 1] = "NotEqual";
  return EquatableOperator14;
})(EquatableOperator4 || {});
function getEquatableOperatorEncoder() {
  return codecs.getScalarEnumEncoder(EquatableOperator4);
}
function getEquatableOperatorDecoder() {
  return codecs.getScalarEnumDecoder(EquatableOperator4);
}
function getEquatableOperatorCodec() {
  return codecs.combineCodec(
    getEquatableOperatorEncoder(),
    getEquatableOperatorDecoder()
  );
}
var IntegerOperator5 = /* @__PURE__ */ ((IntegerOperator13) => {
  IntegerOperator13[IntegerOperator13["Equal"] = 0] = "Equal";
  IntegerOperator13[IntegerOperator13["NotEqual"] = 1] = "NotEqual";
  IntegerOperator13[IntegerOperator13["GreaterThan"] = 2] = "GreaterThan";
  IntegerOperator13[IntegerOperator13["LessThan"] = 3] = "LessThan";
  IntegerOperator13[IntegerOperator13["GreaterThanOrEqual"] = 4] = "GreaterThanOrEqual";
  IntegerOperator13[IntegerOperator13["LessThanOrEqual"] = 5] = "LessThanOrEqual";
  IntegerOperator13[IntegerOperator13["Contains"] = 6] = "Contains";
  IntegerOperator13[IntegerOperator13["DoesNotContain"] = 7] = "DoesNotContain";
  return IntegerOperator13;
})(IntegerOperator5 || {});
function getIntegerOperatorEncoder() {
  return codecs.getScalarEnumEncoder(IntegerOperator5);
}
function getIntegerOperatorDecoder() {
  return codecs.getScalarEnumDecoder(IntegerOperator5);
}
function getIntegerOperatorCodec() {
  return codecs.combineCodec(getIntegerOperatorEncoder(), getIntegerOperatorDecoder());
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
function getKnownProgramEncoder() {
  return codecs.getScalarEnumEncoder(KnownProgram2);
}
function getKnownProgramDecoder() {
  return codecs.getScalarEnumDecoder(KnownProgram2);
}
function getKnownProgramCodec() {
  return codecs.combineCodec(getKnownProgramEncoder(), getKnownProgramDecoder());
}
var LogLevel = /* @__PURE__ */ ((LogLevel2) => {
  LogLevel2[LogLevel2["Silent"] = 0] = "Silent";
  LogLevel2[LogLevel2["PlaintextMessage"] = 1] = "PlaintextMessage";
  LogLevel2[LogLevel2["EncodedMessage"] = 2] = "EncodedMessage";
  LogLevel2[LogLevel2["EncodedNoop"] = 3] = "EncodedNoop";
  return LogLevel2;
})(LogLevel || {});
function getLogLevelEncoder() {
  return codecs.getScalarEnumEncoder(LogLevel);
}
function getLogLevelDecoder() {
  return codecs.getScalarEnumDecoder(LogLevel);
}
function getLogLevelCodec() {
  return codecs.combineCodec(getLogLevelEncoder(), getLogLevelDecoder());
}
function getMerkleTreeAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "VerifyLeaf",
      codecs.getStructEncoder([
        ["leafIndex", codecs.getU32Encoder()],
        ["leafHash", codecs.getBytesEncoder({ size: 32 })]
      ])
    ]
  ]);
}
function getMerkleTreeAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "VerifyLeaf",
      codecs.getStructDecoder([
        ["leafIndex", codecs.getU32Decoder()],
        ["leafHash", codecs.getBytesDecoder({ size: 32 })]
      ])
    ]
  ]);
}
function getMerkleTreeAssertionCodec() {
  return codecs.combineCodec(
    getMerkleTreeAssertionEncoder(),
    getMerkleTreeAssertionDecoder()
  );
}
function merkleTreeAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMerkleTreeAssertion(kind, value) {
  return value.__kind === kind;
}
function getMetaAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "RentExemptReserve",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "AuthorizedStaker",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "AuthorizedWithdrawer",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "LockupUnixTimestamp",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LockupEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LockupCustodian",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getMetaAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "RentExemptReserve",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "AuthorizedStaker",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "AuthorizedWithdrawer",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "LockupUnixTimestamp",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LockupEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LockupCustodian",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getMetaAssertionCodec() {
  return codecs.combineCodec(getMetaAssertionEncoder(), getMetaAssertionDecoder());
}
function metaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMetaAssertion(kind, value) {
  return value.__kind === kind;
}
function getMintAccountAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "MintAuthority",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Supply",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Decimals",
      codecs.getStructEncoder([
        ["value", codecs.getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsInitialized",
      codecs.getStructEncoder([
        ["value", codecs.getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "FreezeAuthority",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getMintAccountAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "MintAuthority",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Supply",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Decimals",
      codecs.getStructDecoder([
        ["value", codecs.getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsInitialized",
      codecs.getStructDecoder([
        ["value", codecs.getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "FreezeAuthority",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getMintAccountAssertionCodec() {
  return codecs.combineCodec(
    getMintAccountAssertionEncoder(),
    getMintAccountAssertionDecoder()
  );
}
function mintAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMintAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getStakeAccountAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "State",
      codecs.getStructEncoder([
        ["value", getStakeStateTypeEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "MetaAssertion",
      codecs.getStructEncoder([
        ["fields", codecs.getTupleEncoder([getMetaAssertionEncoder()])]
      ])
    ],
    [
      "StakeAssertion",
      codecs.getStructEncoder([
        ["fields", codecs.getTupleEncoder([getStakeAssertionEncoder()])]
      ])
    ],
    [
      "StakeFlags",
      codecs.getStructEncoder([
        ["value", codecs.getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getStakeAccountAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "State",
      codecs.getStructDecoder([
        ["value", getStakeStateTypeDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "MetaAssertion",
      codecs.getStructDecoder([
        ["fields", codecs.getTupleDecoder([getMetaAssertionDecoder()])]
      ])
    ],
    [
      "StakeAssertion",
      codecs.getStructDecoder([
        ["fields", codecs.getTupleDecoder([getStakeAssertionDecoder()])]
      ])
    ],
    [
      "StakeFlags",
      codecs.getStructDecoder([
        ["value", codecs.getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getStakeAccountAssertionCodec() {
  return codecs.combineCodec(
    getStakeAccountAssertionEncoder(),
    getStakeAccountAssertionDecoder()
  );
}
function stakeAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isStakeAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getStakeAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "DelegationVoterPubkey",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "DelegationStake",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DelegationActivationEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DelegationDeactivationEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "CreditsObserved",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getStakeAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "DelegationVoterPubkey",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "DelegationStake",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DelegationActivationEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DelegationDeactivationEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "CreditsObserved",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getStakeAssertionCodec() {
  return codecs.combineCodec(getStakeAssertionEncoder(), getStakeAssertionDecoder());
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
function getStakeStateTypeEncoder() {
  return codecs.getScalarEnumEncoder(StakeStateType2);
}
function getStakeStateTypeDecoder() {
  return codecs.getScalarEnumDecoder(StakeStateType2);
}
function getStakeStateTypeCodec() {
  return codecs.combineCodec(getStakeStateTypeEncoder(), getStakeStateTypeDecoder());
}
function getSysvarClockAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Slot",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "EpochStartTimestamp",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Epoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LeaderScheduleEpoch",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "UnixTimestamp",
      codecs.getStructEncoder([
        ["value", codecs.getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getSysvarClockAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Slot",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "EpochStartTimestamp",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Epoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LeaderScheduleEpoch",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "UnixTimestamp",
      codecs.getStructDecoder([
        ["value", codecs.getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getSysvarClockAssertionCodec() {
  return codecs.combineCodec(
    getSysvarClockAssertionEncoder(),
    getSysvarClockAssertionDecoder()
  );
}
function sysvarClockAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isSysvarClockAssertion(kind, value) {
  return value.__kind === kind;
}
function getTokenAccountAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Mint",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Owner",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Amount",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Delegate",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "State",
      codecs.getStructEncoder([
        ["value", codecs.getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsNative",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(codecs.getU64Encoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "DelegatedAmount",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "CloseAuthority",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    ["TokenAccountOwnerIsDerived", codecs.getUnitEncoder()]
  ]);
}
function getTokenAccountAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Mint",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Owner",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Amount",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Delegate",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "State",
      codecs.getStructDecoder([
        ["value", codecs.getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsNative",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(codecs.getU64Decoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "DelegatedAmount",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "CloseAuthority",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    ["TokenAccountOwnerIsDerived", codecs.getUnitDecoder()]
  ]);
}
function getTokenAccountAssertionCodec() {
  return codecs.combineCodec(
    getTokenAccountAssertionEncoder(),
    getTokenAccountAssertionDecoder()
  );
}
function tokenAccountAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isTokenAccountAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradableBufferAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "Authority",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradableBufferAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "Authority",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradableBufferAssertionCodec() {
  return codecs.combineCodec(
    getUpgradableBufferAssertionEncoder(),
    getUpgradableBufferAssertionDecoder()
  );
}
function upgradableBufferAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradableBufferAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradeableLoaderStateAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "State",
      codecs.getStructEncoder([
        ["value", getUpgradeableLoaderStateTypeEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Buffer",
      codecs.getStructEncoder([
        ["fields", codecs.getTupleEncoder([getUpgradableBufferAssertionEncoder()])]
      ])
    ],
    [
      "Program",
      codecs.getStructEncoder([
        ["fields", codecs.getTupleEncoder([getUpgradeableProgramAssertionEncoder()])]
      ])
    ],
    [
      "ProgramData",
      codecs.getStructEncoder([
        [
          "fields",
          codecs.getTupleEncoder([getUpgradeableProgramDataAssertionEncoder()])
        ]
      ])
    ]
  ]);
}
function getUpgradeableLoaderStateAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "State",
      codecs.getStructDecoder([
        ["value", getUpgradeableLoaderStateTypeDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Buffer",
      codecs.getStructDecoder([
        ["fields", codecs.getTupleDecoder([getUpgradableBufferAssertionDecoder()])]
      ])
    ],
    [
      "Program",
      codecs.getStructDecoder([
        ["fields", codecs.getTupleDecoder([getUpgradeableProgramAssertionDecoder()])]
      ])
    ],
    [
      "ProgramData",
      codecs.getStructDecoder([
        [
          "fields",
          codecs.getTupleDecoder([getUpgradeableProgramDataAssertionDecoder()])
        ]
      ])
    ]
  ]);
}
function getUpgradeableLoaderStateAssertionCodec() {
  return codecs.combineCodec(
    getUpgradeableLoaderStateAssertionEncoder(),
    getUpgradeableLoaderStateAssertionDecoder()
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
function getUpgradeableLoaderStateTypeEncoder() {
  return codecs.getScalarEnumEncoder(UpgradeableLoaderStateType2);
}
function getUpgradeableLoaderStateTypeDecoder() {
  return codecs.getScalarEnumDecoder(UpgradeableLoaderStateType2);
}
function getUpgradeableLoaderStateTypeCodec() {
  return codecs.combineCodec(
    getUpgradeableLoaderStateTypeEncoder(),
    getUpgradeableLoaderStateTypeDecoder()
  );
}
function getUpgradeableProgramAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "ProgramDataAddress",
      codecs.getStructEncoder([
        ["value", addresses.getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "ProgramDataAddress",
      codecs.getStructDecoder([
        ["value", addresses.getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramAssertionCodec() {
  return codecs.combineCodec(
    getUpgradeableProgramAssertionEncoder(),
    getUpgradeableProgramAssertionDecoder()
  );
}
function upgradeableProgramAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradeableProgramAssertion(kind, value) {
  return value.__kind === kind;
}
function getUpgradeableProgramDataAssertionEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "UpgradeAuthority",
      codecs.getStructEncoder([
        ["value", codecs.getOptionEncoder(addresses.getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Slot",
      codecs.getStructEncoder([
        ["value", codecs.getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramDataAssertionDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "UpgradeAuthority",
      codecs.getStructDecoder([
        ["value", codecs.getOptionDecoder(addresses.getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Slot",
      codecs.getStructDecoder([
        ["value", codecs.getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramDataAssertionCodec() {
  return codecs.combineCodec(
    getUpgradeableProgramDataAssertionEncoder(),
    getUpgradeableProgramDataAssertionDecoder()
  );
}
function upgradeableProgramDataAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isUpgradeableProgramDataAssertion(kind, value) {
  return value.__kind === kind;
}
function getWriteTypeEncoder() {
  return codecs.getDataEnumEncoder([
    [
      "AccountData",
      codecs.getStructEncoder([
        ["offset", codecs.getU16Encoder()],
        ["dataLength", codecs.getU16Encoder()]
      ])
    ],
    [
      "AccountInfoField",
      codecs.getStructEncoder([
        ["fields", codecs.getTupleEncoder([getAccountInfoFieldEncoder()])]
      ])
    ],
    [
      "DataValue",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([getDataValueEncoder()])]])
    ],
    [
      "Clock",
      codecs.getStructEncoder([["fields", codecs.getTupleEncoder([getClockFieldEncoder()])]])
    ]
  ]);
}
function getWriteTypeDecoder() {
  return codecs.getDataEnumDecoder([
    [
      "AccountData",
      codecs.getStructDecoder([
        ["offset", codecs.getU16Decoder()],
        ["dataLength", codecs.getU16Decoder()]
      ])
    ],
    [
      "AccountInfoField",
      codecs.getStructDecoder([
        ["fields", codecs.getTupleDecoder([getAccountInfoFieldDecoder()])]
      ])
    ],
    [
      "DataValue",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([getDataValueDecoder()])]])
    ],
    [
      "Clock",
      codecs.getStructDecoder([["fields", codecs.getTupleDecoder([getClockFieldDecoder()])]])
    ]
  ]);
}
function getWriteTypeCodec() {
  return codecs.combineCodec(getWriteTypeEncoder(), getWriteTypeDecoder());
}
function writeType(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isWriteType(kind, value) {
  return value.__kind === kind;
}

// src/generated/instructions/assertAccountData.ts
function getAssertAccountDataInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["offset", codecs.getU16Encoder()],
      ["assertion", getDataValueAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 2,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertAccountDataInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["offset", codecs.getU16Decoder()],
    ["assertion", getDataValueAssertionDecoder()]
  ]);
}
function getAssertAccountDataInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertAccountDataInstructionDataEncoder(),
    getAssertAccountDataInstructionDataDecoder()
  );
}
function getAssertAccountDataInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertAccountDataInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertAccountDataInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertAccountDataInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertAccountDataInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertAccountDataInstructionDataDecoder().decode(instruction.data)
  };
}
function getAssertAccountDeltaInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getAccountDeltaAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 3,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertAccountDeltaInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getAccountDeltaAssertionDecoder()]
  ]);
}
function getAssertAccountDeltaInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertAccountDeltaInstructionDataEncoder(),
    getAssertAccountDeltaInstructionDataDecoder()
  );
}
function getAssertAccountDeltaInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    accountA: { value: input.accountA ?? null, isWritable: false },
    accountB: { value: input.accountB ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertAccountDeltaInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertAccountDeltaInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.accountA, instructions.AccountRole.READONLY),
      accountMetaWithDefault(accounts.accountB, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertAccountDeltaInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertAccountDeltaInstruction(instruction) {
  if (instruction.accounts.length < 2) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      accountA: getNextAccount(),
      accountB: getNextAccount()
    },
    data: getAssertAccountDeltaInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertAccountInfoInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getAccountInfoAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 4,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertAccountInfoInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getAccountInfoAssertionDecoder()]
  ]);
}
function getAssertAccountInfoInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertAccountInfoInstructionDataEncoder(),
    getAssertAccountInfoInstructionDataDecoder()
  );
}
function getAssertAccountInfoInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertAccountInfoInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertAccountInfoInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertAccountInfoInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertAccountInfoInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertAccountInfoInstructionDataDecoder().decode(instruction.data)
  };
}
function getAssertAccountInfoMultiInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", codecs.getArrayEncoder(getAccountInfoAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 5,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertAccountInfoMultiInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", codecs.getArrayDecoder(getAccountInfoAssertionDecoder())]
  ]);
}
function getAssertAccountInfoMultiInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertAccountInfoMultiInstructionDataEncoder(),
    getAssertAccountInfoMultiInstructionDataDecoder()
  );
}
function getAssertAccountInfoMultiInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertAccountInfoMultiInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertAccountInfoMultiInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertAccountInfoMultiInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertAccountInfoMultiInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertAccountInfoMultiInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertMerkleTreeAccountInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getMerkleTreeAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 15,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertMerkleTreeAccountInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getMerkleTreeAssertionDecoder()]
  ]);
}
function getAssertMerkleTreeAccountInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertMerkleTreeAccountInstructionDataEncoder(),
    getAssertMerkleTreeAccountInstructionDataDecoder()
  );
}
function getAssertMerkleTreeAccountInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetMerkleTree: {
      value: input.targetMerkleTree ?? null,
      isWritable: false
    },
    root: { value: input.root ?? null, isWritable: false },
    splAccountCompression: {
      value: input.splAccountCompression ?? null,
      isWritable: false
    }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertMerkleTreeAccountInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertMerkleTreeAccountInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetMerkleTree, instructions.AccountRole.READONLY),
      accountMetaWithDefault(accounts.root, instructions.AccountRole.READONLY),
      accountMetaWithDefault(
        accounts.splAccountCompression,
        instructions.AccountRole.READONLY
      ),
      ...remainingAccounts ?? []
    ],
    data: getAssertMerkleTreeAccountInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertMerkleTreeAccountInstruction(instruction) {
  if (instruction.accounts.length < 3) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetMerkleTree: getNextAccount(),
      root: getNextAccount(),
      splAccountCompression: getNextAccount()
    },
    data: getAssertMerkleTreeAccountInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertMintAccountInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getMintAccountAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 6,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertMintAccountInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getMintAccountAssertionDecoder()]
  ]);
}
function getAssertMintAccountInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertMintAccountInstructionDataEncoder(),
    getAssertMintAccountInstructionDataDecoder()
  );
}
function getAssertMintAccountInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertMintAccountInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertMintAccountInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertMintAccountInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertMintAccountInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertMintAccountInstructionDataDecoder().decode(instruction.data)
  };
}
function getAssertMintAccountMultiInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", codecs.getArrayEncoder(getMintAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 7,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertMintAccountMultiInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", codecs.getArrayDecoder(getMintAccountAssertionDecoder())]
  ]);
}
function getAssertMintAccountMultiInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertMintAccountMultiInstructionDataEncoder(),
    getAssertMintAccountMultiInstructionDataDecoder()
  );
}
function getAssertMintAccountMultiInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertMintAccountMultiInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertMintAccountMultiInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertMintAccountMultiInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertMintAccountMultiInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertMintAccountMultiInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertStakeAccountInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getStakeAccountAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 10,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertStakeAccountInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getStakeAccountAssertionDecoder()]
  ]);
}
function getAssertStakeAccountInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertStakeAccountInstructionDataEncoder(),
    getAssertStakeAccountInstructionDataDecoder()
  );
}
function getAssertStakeAccountInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertStakeAccountInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertStakeAccountInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertStakeAccountInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertStakeAccountInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertStakeAccountInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertStakeAccountMultiInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", codecs.getArrayEncoder(getStakeAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 11,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertStakeAccountMultiInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", codecs.getArrayDecoder(getStakeAccountAssertionDecoder())]
  ]);
}
function getAssertStakeAccountMultiInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertStakeAccountMultiInstructionDataEncoder(),
    getAssertStakeAccountMultiInstructionDataDecoder()
  );
}
function getAssertStakeAccountMultiInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertStakeAccountMultiInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertStakeAccountMultiInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertStakeAccountMultiInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertStakeAccountMultiInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertStakeAccountMultiInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertSysvarClockInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getSysvarClockAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 14,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertSysvarClockInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getSysvarClockAssertionDecoder()]
  ]);
}
function getAssertSysvarClockInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertSysvarClockInstructionDataEncoder(),
    getAssertSysvarClockInstructionDataDecoder()
  );
}
function getAssertSysvarClockInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const args = { ...input };
  const instruction = getAssertSysvarClockInstructionRaw(
    args,
    programAddress
  );
  return instruction;
}
function getAssertSysvarClockInstructionRaw(args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: remainingAccounts ?? [],
    data: getAssertSysvarClockInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertSysvarClockInstruction(instruction) {
  return {
    programAddress: instruction.programAddress,
    data: getAssertSysvarClockInstructionDataDecoder().decode(instruction.data)
  };
}
function getAssertTokenAccountInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getTokenAccountAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 8,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertTokenAccountInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getTokenAccountAssertionDecoder()]
  ]);
}
function getAssertTokenAccountInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertTokenAccountInstructionDataEncoder(),
    getAssertTokenAccountInstructionDataDecoder()
  );
}
function getAssertTokenAccountInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertTokenAccountInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertTokenAccountInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertTokenAccountInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertTokenAccountInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertTokenAccountInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertTokenAccountMultiInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", codecs.getArrayEncoder(getTokenAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 9,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertTokenAccountMultiInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", codecs.getArrayDecoder(getTokenAccountAssertionDecoder())]
  ]);
}
function getAssertTokenAccountMultiInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertTokenAccountMultiInstructionDataEncoder(),
    getAssertTokenAccountMultiInstructionDataDecoder()
  );
}
function getAssertTokenAccountMultiInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertTokenAccountMultiInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertTokenAccountMultiInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertTokenAccountMultiInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseAssertTokenAccountMultiInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertTokenAccountMultiInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertUpgradeableLoaderAccountInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertion", getUpgradeableLoaderStateAssertionEncoder()]
    ]),
    (value) => ({
      ...value,
      discriminator: 12,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertUpgradeableLoaderAccountInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getUpgradeableLoaderStateAssertionDecoder()]
  ]);
}
function getAssertUpgradeableLoaderAccountInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertUpgradeableLoaderAccountInstructionDataEncoder(),
    getAssertUpgradeableLoaderAccountInstructionDataDecoder()
  );
}
function getAssertUpgradeableLoaderAccountInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertUpgradeableLoaderAccountInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertUpgradeableLoaderAccountInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertUpgradeableLoaderAccountInstructionDataEncoder().encode(
      args
    ),
    programAddress
  };
}
function parseAssertUpgradeableLoaderAccountInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertUpgradeableLoaderAccountInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      [
        "assertions",
        codecs.getArrayEncoder(getUpgradeableLoaderStateAssertionEncoder())
      ]
    ]),
    (value) => ({
      ...value,
      discriminator: 13,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    [
      "assertions",
      codecs.getArrayDecoder(getUpgradeableLoaderStateAssertionDecoder())
    ]
  ]);
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataCodec() {
  return codecs.combineCodec(
    getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder(),
    getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder()
  );
}
function getAssertUpgradeableLoaderAccountMultiInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    targetAccount: { value: input.targetAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getAssertUpgradeableLoaderAccountMultiInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getAssertUpgradeableLoaderAccountMultiInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(accounts.targetAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder().encode(
      args
    ),
    programAddress
  };
}
function parseAssertUpgradeableLoaderAccountMultiInstruction(instruction) {
  if (instruction.accounts.length < 1) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      targetAccount: getNextAccount()
    },
    data: getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder().decode(
      instruction.data
    )
  };
}
function getMemoryCloseInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["memoryId", codecs.getU8Encoder()],
      ["memoryBump", codecs.getU8Encoder()]
    ]),
    (value) => ({ ...value, discriminator: 1 })
  );
}
function getMemoryCloseInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["memoryId", codecs.getU8Decoder()],
    ["memoryBump", codecs.getU8Decoder()]
  ]);
}
function getMemoryCloseInstructionDataCodec() {
  return codecs.combineCodec(
    getMemoryCloseInstructionDataEncoder(),
    getMemoryCloseInstructionDataDecoder()
  );
}
function getMemoryCloseInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    programId: { value: input.programId ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    memory: { value: input.memory ?? null, isWritable: true }
  };
  const args = { ...input };
  if (!accounts.programId.value) {
    accounts.programId.value = programAddress;
    accounts.programId.isWritable = false;
  }
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getMemoryCloseInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getMemoryCloseInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(
        accounts.programId ?? {
          address: "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK",
          role: instructions.AccountRole.READONLY
        },
        instructions.AccountRole.READONLY
      ),
      accountMetaWithDefault(accounts.payer, instructions.AccountRole.WRITABLE_SIGNER),
      accountMetaWithDefault(accounts.memory, instructions.AccountRole.WRITABLE),
      ...remainingAccounts ?? []
    ],
    data: getMemoryCloseInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseMemoryCloseInstruction(instruction) {
  if (instruction.accounts.length < 3) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      programId: getNextAccount(),
      payer: getNextAccount(),
      memory: getNextAccount()
    },
    data: getMemoryCloseInstructionDataDecoder().decode(instruction.data)
  };
}
function getMemoryWriteInstructionDataEncoder() {
  return codecs.mapEncoder(
    codecs.getStructEncoder([
      ["discriminator", codecs.getU8Encoder()],
      ["memoryId", codecs.getU8Encoder()],
      ["memoryBump", codecs.getU8Encoder()],
      ["writeOffset", codecs.getU16Encoder()],
      ["writeType", getWriteTypeEncoder()]
    ]),
    (value) => ({ ...value, discriminator: 0, memoryId: value.memoryId ?? 0 })
  );
}
function getMemoryWriteInstructionDataDecoder() {
  return codecs.getStructDecoder([
    ["discriminator", codecs.getU8Decoder()],
    ["memoryId", codecs.getU8Decoder()],
    ["memoryBump", codecs.getU8Decoder()],
    ["writeOffset", codecs.getU16Decoder()],
    ["writeType", getWriteTypeDecoder()]
  ]);
}
function getMemoryWriteInstructionDataCodec() {
  return codecs.combineCodec(
    getMemoryWriteInstructionDataEncoder(),
    getMemoryWriteInstructionDataDecoder()
  );
}
function getMemoryWriteInstruction(input) {
  const programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
  const accounts = {
    programId: { value: input.programId ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    memory: { value: input.memory ?? null, isWritable: true },
    sourceAccount: { value: input.sourceAccount ?? null, isWritable: false }
  };
  const args = { ...input };
  if (!accounts.programId.value) {
    accounts.programId.value = programAddress;
    accounts.programId.isWritable = false;
  }
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value = "11111111111111111111111111111111";
  }
  const accountMetas = getAccountMetasWithSigners(
    accounts,
    "programId",
    programAddress
  );
  const instruction = getMemoryWriteInstructionRaw(
    accountMetas,
    args,
    programAddress
  );
  return instruction;
}
function getMemoryWriteInstructionRaw(accounts, args, programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK", remainingAccounts) {
  return {
    accounts: [
      accountMetaWithDefault(
        accounts.programId ?? {
          address: "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK",
          role: instructions.AccountRole.READONLY
        },
        instructions.AccountRole.READONLY
      ),
      accountMetaWithDefault(
        accounts.systemProgram ?? "11111111111111111111111111111111",
        instructions.AccountRole.READONLY
      ),
      accountMetaWithDefault(accounts.payer, instructions.AccountRole.WRITABLE_SIGNER),
      accountMetaWithDefault(accounts.memory, instructions.AccountRole.WRITABLE),
      accountMetaWithDefault(accounts.sourceAccount, instructions.AccountRole.READONLY),
      ...remainingAccounts ?? []
    ],
    data: getMemoryWriteInstructionDataEncoder().encode(args),
    programAddress
  };
}
function parseMemoryWriteInstruction(instruction) {
  if (instruction.accounts.length < 5) {
    throw new Error("Not enough accounts");
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts[accountIndex];
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      programId: getNextAccount(),
      systemProgram: getNextAccount(),
      payer: getNextAccount(),
      memory: getNextAccount(),
      sourceAccount: getNextAccount()
    },
    data: getMemoryWriteInstructionDataDecoder().decode(instruction.data)
  };
}
async function findMemoryPda(seeds, config = {}) {
  const {
    programAddress = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK"
  } = config;
  return addresses.getProgramDerivedAddress({
    programAddress,
    seeds: [
      codecs.getStringEncoder({ size: "variable" }).encode("memory"),
      addresses.getAddressEncoder().encode(seeds.payer),
      codecs.getU8Encoder().encode(seeds.memoryId)
    ]
  });
}
var LIGHTHOUSE_PROGRAM_ADDRESS = "L1TEVtgA75k273wWz1s6XMmDhQY5i3MwcvKb4VbZzfK";
function getLighthouseProgram() {
  return {
    name: "lighthouse",
    address: LIGHTHOUSE_PROGRAM_ADDRESS,
    getErrorFromCode(code, cause) {
      return getLighthouseProgramErrorFromCode(code, cause);
    }
  };
}
var LighthouseInstruction = /* @__PURE__ */ ((LighthouseInstruction2) => {
  LighthouseInstruction2[LighthouseInstruction2["MemoryWrite"] = 0] = "MemoryWrite";
  LighthouseInstruction2[LighthouseInstruction2["MemoryClose"] = 1] = "MemoryClose";
  LighthouseInstruction2[LighthouseInstruction2["AssertAccountData"] = 2] = "AssertAccountData";
  LighthouseInstruction2[LighthouseInstruction2["AssertAccountDelta"] = 3] = "AssertAccountDelta";
  LighthouseInstruction2[LighthouseInstruction2["AssertAccountInfo"] = 4] = "AssertAccountInfo";
  LighthouseInstruction2[LighthouseInstruction2["AssertAccountInfoMulti"] = 5] = "AssertAccountInfoMulti";
  LighthouseInstruction2[LighthouseInstruction2["AssertMintAccount"] = 6] = "AssertMintAccount";
  LighthouseInstruction2[LighthouseInstruction2["AssertMintAccountMulti"] = 7] = "AssertMintAccountMulti";
  LighthouseInstruction2[LighthouseInstruction2["AssertTokenAccount"] = 8] = "AssertTokenAccount";
  LighthouseInstruction2[LighthouseInstruction2["AssertTokenAccountMulti"] = 9] = "AssertTokenAccountMulti";
  LighthouseInstruction2[LighthouseInstruction2["AssertStakeAccount"] = 10] = "AssertStakeAccount";
  LighthouseInstruction2[LighthouseInstruction2["AssertStakeAccountMulti"] = 11] = "AssertStakeAccountMulti";
  LighthouseInstruction2[LighthouseInstruction2["AssertUpgradeableLoaderAccount"] = 12] = "AssertUpgradeableLoaderAccount";
  LighthouseInstruction2[LighthouseInstruction2["AssertUpgradeableLoaderAccountMulti"] = 13] = "AssertUpgradeableLoaderAccountMulti";
  LighthouseInstruction2[LighthouseInstruction2["AssertSysvarClock"] = 14] = "AssertSysvarClock";
  LighthouseInstruction2[LighthouseInstruction2["AssertMerkleTreeAccount"] = 15] = "AssertMerkleTreeAccount";
  return LighthouseInstruction2;
})(LighthouseInstruction || {});
function identifyLighthouseInstruction(instruction) {
  const data = instruction instanceof Uint8Array ? instruction : instruction.data;
  if (memcmp(data, codecs.getU8Encoder().encode(0), 0)) {
    return 0 /* MemoryWrite */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(1), 0)) {
    return 1 /* MemoryClose */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(2), 0)) {
    return 2 /* AssertAccountData */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(3), 0)) {
    return 3 /* AssertAccountDelta */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(4), 0)) {
    return 4 /* AssertAccountInfo */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(5), 0)) {
    return 5 /* AssertAccountInfoMulti */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(6), 0)) {
    return 6 /* AssertMintAccount */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(7), 0)) {
    return 7 /* AssertMintAccountMulti */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(8), 0)) {
    return 8 /* AssertTokenAccount */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(9), 0)) {
    return 9 /* AssertTokenAccountMulti */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(10), 0)) {
    return 10 /* AssertStakeAccount */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(11), 0)) {
    return 11 /* AssertStakeAccountMulti */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(12), 0)) {
    return 12 /* AssertUpgradeableLoaderAccount */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(13), 0)) {
    return 13 /* AssertUpgradeableLoaderAccountMulti */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(14), 0)) {
    return 14 /* AssertSysvarClock */;
  }
  if (memcmp(data, codecs.getU8Encoder().encode(15), 0)) {
    return 15 /* AssertMerkleTreeAccount */;
  }
  throw new Error(
    "The provided instruction could not be identified as a lighthouse instruction."
  );
}

exports.AccountInfoField = AccountInfoField;
exports.ByteSliceOperator = ByteSliceOperator;
exports.ClockField = ClockField;
exports.EquatableOperator = EquatableOperator4;
exports.IntegerOperator = IntegerOperator5;
exports.KnownProgram = KnownProgram2;
exports.LIGHTHOUSE_PROGRAM_ADDRESS = LIGHTHOUSE_PROGRAM_ADDRESS;
exports.LighthouseInstruction = LighthouseInstruction;
exports.LighthouseProgramError = LighthouseProgramError;
exports.LighthouseProgramErrorCode = LighthouseProgramErrorCode;
exports.LogLevel = LogLevel;
exports.StakeStateType = StakeStateType2;
exports.UpgradeableLoaderStateType = UpgradeableLoaderStateType2;
exports.accountDeltaAssertion = accountDeltaAssertion;
exports.accountInfoAssertion = accountInfoAssertion;
exports.accountInfoDeltaAssertion = accountInfoDeltaAssertion;
exports.accountMetaWithDefault = accountMetaWithDefault;
exports.assertionResult = assertionResult;
exports.dataValue = dataValue;
exports.dataValueAssertion = dataValueAssertion;
exports.dataValueDeltaAssertion = dataValueDeltaAssertion;
exports.expectAddress = expectAddress;
exports.expectProgramDerivedAddress = expectProgramDerivedAddress;
exports.expectSome = expectSome;
exports.expectTransactionSigner = expectTransactionSigner;
exports.findMemoryPda = findMemoryPda;
exports.getAccountDeltaAssertionCodec = getAccountDeltaAssertionCodec;
exports.getAccountDeltaAssertionDecoder = getAccountDeltaAssertionDecoder;
exports.getAccountDeltaAssertionEncoder = getAccountDeltaAssertionEncoder;
exports.getAccountInfoAssertionCodec = getAccountInfoAssertionCodec;
exports.getAccountInfoAssertionDecoder = getAccountInfoAssertionDecoder;
exports.getAccountInfoAssertionEncoder = getAccountInfoAssertionEncoder;
exports.getAccountInfoDeltaAssertionCodec = getAccountInfoDeltaAssertionCodec;
exports.getAccountInfoDeltaAssertionDecoder = getAccountInfoDeltaAssertionDecoder;
exports.getAccountInfoDeltaAssertionEncoder = getAccountInfoDeltaAssertionEncoder;
exports.getAccountInfoFieldCodec = getAccountInfoFieldCodec;
exports.getAccountInfoFieldDecoder = getAccountInfoFieldDecoder;
exports.getAccountInfoFieldEncoder = getAccountInfoFieldEncoder;
exports.getAccountMetasWithSigners = getAccountMetasWithSigners;
exports.getAssertAccountDataInstruction = getAssertAccountDataInstruction;
exports.getAssertAccountDataInstructionDataCodec = getAssertAccountDataInstructionDataCodec;
exports.getAssertAccountDataInstructionDataDecoder = getAssertAccountDataInstructionDataDecoder;
exports.getAssertAccountDataInstructionDataEncoder = getAssertAccountDataInstructionDataEncoder;
exports.getAssertAccountDataInstructionRaw = getAssertAccountDataInstructionRaw;
exports.getAssertAccountDeltaInstruction = getAssertAccountDeltaInstruction;
exports.getAssertAccountDeltaInstructionDataCodec = getAssertAccountDeltaInstructionDataCodec;
exports.getAssertAccountDeltaInstructionDataDecoder = getAssertAccountDeltaInstructionDataDecoder;
exports.getAssertAccountDeltaInstructionDataEncoder = getAssertAccountDeltaInstructionDataEncoder;
exports.getAssertAccountDeltaInstructionRaw = getAssertAccountDeltaInstructionRaw;
exports.getAssertAccountInfoInstruction = getAssertAccountInfoInstruction;
exports.getAssertAccountInfoInstructionDataCodec = getAssertAccountInfoInstructionDataCodec;
exports.getAssertAccountInfoInstructionDataDecoder = getAssertAccountInfoInstructionDataDecoder;
exports.getAssertAccountInfoInstructionDataEncoder = getAssertAccountInfoInstructionDataEncoder;
exports.getAssertAccountInfoInstructionRaw = getAssertAccountInfoInstructionRaw;
exports.getAssertAccountInfoMultiInstruction = getAssertAccountInfoMultiInstruction;
exports.getAssertAccountInfoMultiInstructionDataCodec = getAssertAccountInfoMultiInstructionDataCodec;
exports.getAssertAccountInfoMultiInstructionDataDecoder = getAssertAccountInfoMultiInstructionDataDecoder;
exports.getAssertAccountInfoMultiInstructionDataEncoder = getAssertAccountInfoMultiInstructionDataEncoder;
exports.getAssertAccountInfoMultiInstructionRaw = getAssertAccountInfoMultiInstructionRaw;
exports.getAssertMerkleTreeAccountInstruction = getAssertMerkleTreeAccountInstruction;
exports.getAssertMerkleTreeAccountInstructionDataCodec = getAssertMerkleTreeAccountInstructionDataCodec;
exports.getAssertMerkleTreeAccountInstructionDataDecoder = getAssertMerkleTreeAccountInstructionDataDecoder;
exports.getAssertMerkleTreeAccountInstructionDataEncoder = getAssertMerkleTreeAccountInstructionDataEncoder;
exports.getAssertMerkleTreeAccountInstructionRaw = getAssertMerkleTreeAccountInstructionRaw;
exports.getAssertMintAccountInstruction = getAssertMintAccountInstruction;
exports.getAssertMintAccountInstructionDataCodec = getAssertMintAccountInstructionDataCodec;
exports.getAssertMintAccountInstructionDataDecoder = getAssertMintAccountInstructionDataDecoder;
exports.getAssertMintAccountInstructionDataEncoder = getAssertMintAccountInstructionDataEncoder;
exports.getAssertMintAccountInstructionRaw = getAssertMintAccountInstructionRaw;
exports.getAssertMintAccountMultiInstruction = getAssertMintAccountMultiInstruction;
exports.getAssertMintAccountMultiInstructionDataCodec = getAssertMintAccountMultiInstructionDataCodec;
exports.getAssertMintAccountMultiInstructionDataDecoder = getAssertMintAccountMultiInstructionDataDecoder;
exports.getAssertMintAccountMultiInstructionDataEncoder = getAssertMintAccountMultiInstructionDataEncoder;
exports.getAssertMintAccountMultiInstructionRaw = getAssertMintAccountMultiInstructionRaw;
exports.getAssertStakeAccountInstruction = getAssertStakeAccountInstruction;
exports.getAssertStakeAccountInstructionDataCodec = getAssertStakeAccountInstructionDataCodec;
exports.getAssertStakeAccountInstructionDataDecoder = getAssertStakeAccountInstructionDataDecoder;
exports.getAssertStakeAccountInstructionDataEncoder = getAssertStakeAccountInstructionDataEncoder;
exports.getAssertStakeAccountInstructionRaw = getAssertStakeAccountInstructionRaw;
exports.getAssertStakeAccountMultiInstruction = getAssertStakeAccountMultiInstruction;
exports.getAssertStakeAccountMultiInstructionDataCodec = getAssertStakeAccountMultiInstructionDataCodec;
exports.getAssertStakeAccountMultiInstructionDataDecoder = getAssertStakeAccountMultiInstructionDataDecoder;
exports.getAssertStakeAccountMultiInstructionDataEncoder = getAssertStakeAccountMultiInstructionDataEncoder;
exports.getAssertStakeAccountMultiInstructionRaw = getAssertStakeAccountMultiInstructionRaw;
exports.getAssertSysvarClockInstruction = getAssertSysvarClockInstruction;
exports.getAssertSysvarClockInstructionDataCodec = getAssertSysvarClockInstructionDataCodec;
exports.getAssertSysvarClockInstructionDataDecoder = getAssertSysvarClockInstructionDataDecoder;
exports.getAssertSysvarClockInstructionDataEncoder = getAssertSysvarClockInstructionDataEncoder;
exports.getAssertSysvarClockInstructionRaw = getAssertSysvarClockInstructionRaw;
exports.getAssertTokenAccountInstruction = getAssertTokenAccountInstruction;
exports.getAssertTokenAccountInstructionDataCodec = getAssertTokenAccountInstructionDataCodec;
exports.getAssertTokenAccountInstructionDataDecoder = getAssertTokenAccountInstructionDataDecoder;
exports.getAssertTokenAccountInstructionDataEncoder = getAssertTokenAccountInstructionDataEncoder;
exports.getAssertTokenAccountInstructionRaw = getAssertTokenAccountInstructionRaw;
exports.getAssertTokenAccountMultiInstruction = getAssertTokenAccountMultiInstruction;
exports.getAssertTokenAccountMultiInstructionDataCodec = getAssertTokenAccountMultiInstructionDataCodec;
exports.getAssertTokenAccountMultiInstructionDataDecoder = getAssertTokenAccountMultiInstructionDataDecoder;
exports.getAssertTokenAccountMultiInstructionDataEncoder = getAssertTokenAccountMultiInstructionDataEncoder;
exports.getAssertTokenAccountMultiInstructionRaw = getAssertTokenAccountMultiInstructionRaw;
exports.getAssertUpgradeableLoaderAccountInstruction = getAssertUpgradeableLoaderAccountInstruction;
exports.getAssertUpgradeableLoaderAccountInstructionDataCodec = getAssertUpgradeableLoaderAccountInstructionDataCodec;
exports.getAssertUpgradeableLoaderAccountInstructionDataDecoder = getAssertUpgradeableLoaderAccountInstructionDataDecoder;
exports.getAssertUpgradeableLoaderAccountInstructionDataEncoder = getAssertUpgradeableLoaderAccountInstructionDataEncoder;
exports.getAssertUpgradeableLoaderAccountInstructionRaw = getAssertUpgradeableLoaderAccountInstructionRaw;
exports.getAssertUpgradeableLoaderAccountMultiInstruction = getAssertUpgradeableLoaderAccountMultiInstruction;
exports.getAssertUpgradeableLoaderAccountMultiInstructionDataCodec = getAssertUpgradeableLoaderAccountMultiInstructionDataCodec;
exports.getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder = getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder;
exports.getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder = getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder;
exports.getAssertUpgradeableLoaderAccountMultiInstructionRaw = getAssertUpgradeableLoaderAccountMultiInstructionRaw;
exports.getAssertionResultCodec = getAssertionResultCodec;
exports.getAssertionResultDecoder = getAssertionResultDecoder;
exports.getAssertionResultEncoder = getAssertionResultEncoder;
exports.getByteSliceOperatorCodec = getByteSliceOperatorCodec;
exports.getByteSliceOperatorDecoder = getByteSliceOperatorDecoder;
exports.getByteSliceOperatorEncoder = getByteSliceOperatorEncoder;
exports.getClockFieldCodec = getClockFieldCodec;
exports.getClockFieldDecoder = getClockFieldDecoder;
exports.getClockFieldEncoder = getClockFieldEncoder;
exports.getDataValueAssertionCodec = getDataValueAssertionCodec;
exports.getDataValueAssertionDecoder = getDataValueAssertionDecoder;
exports.getDataValueAssertionEncoder = getDataValueAssertionEncoder;
exports.getDataValueCodec = getDataValueCodec;
exports.getDataValueDecoder = getDataValueDecoder;
exports.getDataValueDeltaAssertionCodec = getDataValueDeltaAssertionCodec;
exports.getDataValueDeltaAssertionDecoder = getDataValueDeltaAssertionDecoder;
exports.getDataValueDeltaAssertionEncoder = getDataValueDeltaAssertionEncoder;
exports.getDataValueEncoder = getDataValueEncoder;
exports.getEquatableOperatorCodec = getEquatableOperatorCodec;
exports.getEquatableOperatorDecoder = getEquatableOperatorDecoder;
exports.getEquatableOperatorEncoder = getEquatableOperatorEncoder;
exports.getIntegerOperatorCodec = getIntegerOperatorCodec;
exports.getIntegerOperatorDecoder = getIntegerOperatorDecoder;
exports.getIntegerOperatorEncoder = getIntegerOperatorEncoder;
exports.getKnownProgramCodec = getKnownProgramCodec;
exports.getKnownProgramDecoder = getKnownProgramDecoder;
exports.getKnownProgramEncoder = getKnownProgramEncoder;
exports.getLighthouseProgram = getLighthouseProgram;
exports.getLighthouseProgramErrorFromCode = getLighthouseProgramErrorFromCode;
exports.getLogLevelCodec = getLogLevelCodec;
exports.getLogLevelDecoder = getLogLevelDecoder;
exports.getLogLevelEncoder = getLogLevelEncoder;
exports.getMemoryCloseInstruction = getMemoryCloseInstruction;
exports.getMemoryCloseInstructionDataCodec = getMemoryCloseInstructionDataCodec;
exports.getMemoryCloseInstructionDataDecoder = getMemoryCloseInstructionDataDecoder;
exports.getMemoryCloseInstructionDataEncoder = getMemoryCloseInstructionDataEncoder;
exports.getMemoryCloseInstructionRaw = getMemoryCloseInstructionRaw;
exports.getMemoryWriteInstruction = getMemoryWriteInstruction;
exports.getMemoryWriteInstructionDataCodec = getMemoryWriteInstructionDataCodec;
exports.getMemoryWriteInstructionDataDecoder = getMemoryWriteInstructionDataDecoder;
exports.getMemoryWriteInstructionDataEncoder = getMemoryWriteInstructionDataEncoder;
exports.getMemoryWriteInstructionRaw = getMemoryWriteInstructionRaw;
exports.getMerkleTreeAssertionCodec = getMerkleTreeAssertionCodec;
exports.getMerkleTreeAssertionDecoder = getMerkleTreeAssertionDecoder;
exports.getMerkleTreeAssertionEncoder = getMerkleTreeAssertionEncoder;
exports.getMetaAssertionCodec = getMetaAssertionCodec;
exports.getMetaAssertionDecoder = getMetaAssertionDecoder;
exports.getMetaAssertionEncoder = getMetaAssertionEncoder;
exports.getMintAccountAssertionCodec = getMintAccountAssertionCodec;
exports.getMintAccountAssertionDecoder = getMintAccountAssertionDecoder;
exports.getMintAccountAssertionEncoder = getMintAccountAssertionEncoder;
exports.getStakeAccountAssertionCodec = getStakeAccountAssertionCodec;
exports.getStakeAccountAssertionDecoder = getStakeAccountAssertionDecoder;
exports.getStakeAccountAssertionEncoder = getStakeAccountAssertionEncoder;
exports.getStakeAssertionCodec = getStakeAssertionCodec;
exports.getStakeAssertionDecoder = getStakeAssertionDecoder;
exports.getStakeAssertionEncoder = getStakeAssertionEncoder;
exports.getStakeStateTypeCodec = getStakeStateTypeCodec;
exports.getStakeStateTypeDecoder = getStakeStateTypeDecoder;
exports.getStakeStateTypeEncoder = getStakeStateTypeEncoder;
exports.getSysvarClockAssertionCodec = getSysvarClockAssertionCodec;
exports.getSysvarClockAssertionDecoder = getSysvarClockAssertionDecoder;
exports.getSysvarClockAssertionEncoder = getSysvarClockAssertionEncoder;
exports.getTokenAccountAssertionCodec = getTokenAccountAssertionCodec;
exports.getTokenAccountAssertionDecoder = getTokenAccountAssertionDecoder;
exports.getTokenAccountAssertionEncoder = getTokenAccountAssertionEncoder;
exports.getUpgradableBufferAssertionCodec = getUpgradableBufferAssertionCodec;
exports.getUpgradableBufferAssertionDecoder = getUpgradableBufferAssertionDecoder;
exports.getUpgradableBufferAssertionEncoder = getUpgradableBufferAssertionEncoder;
exports.getUpgradeableLoaderStateAssertionCodec = getUpgradeableLoaderStateAssertionCodec;
exports.getUpgradeableLoaderStateAssertionDecoder = getUpgradeableLoaderStateAssertionDecoder;
exports.getUpgradeableLoaderStateAssertionEncoder = getUpgradeableLoaderStateAssertionEncoder;
exports.getUpgradeableLoaderStateTypeCodec = getUpgradeableLoaderStateTypeCodec;
exports.getUpgradeableLoaderStateTypeDecoder = getUpgradeableLoaderStateTypeDecoder;
exports.getUpgradeableLoaderStateTypeEncoder = getUpgradeableLoaderStateTypeEncoder;
exports.getUpgradeableProgramAssertionCodec = getUpgradeableProgramAssertionCodec;
exports.getUpgradeableProgramAssertionDecoder = getUpgradeableProgramAssertionDecoder;
exports.getUpgradeableProgramAssertionEncoder = getUpgradeableProgramAssertionEncoder;
exports.getUpgradeableProgramDataAssertionCodec = getUpgradeableProgramDataAssertionCodec;
exports.getUpgradeableProgramDataAssertionDecoder = getUpgradeableProgramDataAssertionDecoder;
exports.getUpgradeableProgramDataAssertionEncoder = getUpgradeableProgramDataAssertionEncoder;
exports.getWriteTypeCodec = getWriteTypeCodec;
exports.getWriteTypeDecoder = getWriteTypeDecoder;
exports.getWriteTypeEncoder = getWriteTypeEncoder;
exports.identifyLighthouseInstruction = identifyLighthouseInstruction;
exports.isAccountDeltaAssertion = isAccountDeltaAssertion;
exports.isAccountInfoAssertion = isAccountInfoAssertion;
exports.isAccountInfoDeltaAssertion = isAccountInfoDeltaAssertion;
exports.isAssertionResult = isAssertionResult;
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
exports.isTransactionSigner = isTransactionSigner;
exports.isUpgradableBufferAssertion = isUpgradableBufferAssertion;
exports.isUpgradeableLoaderStateAssertion = isUpgradeableLoaderStateAssertion;
exports.isUpgradeableProgramAssertion = isUpgradeableProgramAssertion;
exports.isUpgradeableProgramDataAssertion = isUpgradeableProgramDataAssertion;
exports.isWriteType = isWriteType;
exports.memcmp = memcmp;
exports.merkleTreeAssertion = merkleTreeAssertion;
exports.metaAssertion = metaAssertion;
exports.mintAccountAssertion = mintAccountAssertion;
exports.parseAssertAccountDataInstruction = parseAssertAccountDataInstruction;
exports.parseAssertAccountDeltaInstruction = parseAssertAccountDeltaInstruction;
exports.parseAssertAccountInfoInstruction = parseAssertAccountInfoInstruction;
exports.parseAssertAccountInfoMultiInstruction = parseAssertAccountInfoMultiInstruction;
exports.parseAssertMerkleTreeAccountInstruction = parseAssertMerkleTreeAccountInstruction;
exports.parseAssertMintAccountInstruction = parseAssertMintAccountInstruction;
exports.parseAssertMintAccountMultiInstruction = parseAssertMintAccountMultiInstruction;
exports.parseAssertStakeAccountInstruction = parseAssertStakeAccountInstruction;
exports.parseAssertStakeAccountMultiInstruction = parseAssertStakeAccountMultiInstruction;
exports.parseAssertSysvarClockInstruction = parseAssertSysvarClockInstruction;
exports.parseAssertTokenAccountInstruction = parseAssertTokenAccountInstruction;
exports.parseAssertTokenAccountMultiInstruction = parseAssertTokenAccountMultiInstruction;
exports.parseAssertUpgradeableLoaderAccountInstruction = parseAssertUpgradeableLoaderAccountInstruction;
exports.parseAssertUpgradeableLoaderAccountMultiInstruction = parseAssertUpgradeableLoaderAccountMultiInstruction;
exports.parseMemoryCloseInstruction = parseMemoryCloseInstruction;
exports.parseMemoryWriteInstruction = parseMemoryWriteInstruction;
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