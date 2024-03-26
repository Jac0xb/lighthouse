import { getDataEnumEncoder, getStructEncoder, getU16Encoder, getDataEnumDecoder, getStructDecoder, getU16Decoder, combineCodec, getU64Encoder, getBooleanEncoder, getBytesEncoder, getOptionEncoder, getU64Decoder, getBooleanDecoder, getBytesDecoder, getOptionDecoder, getI128Encoder, getI128Decoder, getScalarEnumEncoder, getScalarEnumDecoder, getTupleEncoder, getU8Encoder, getU32Encoder, getU128Encoder, getI8Encoder, getI16Encoder, getI32Encoder, getI64Encoder, getTupleDecoder, getU8Decoder, getU32Decoder, getU128Decoder, getI8Decoder, getI16Decoder, getI32Decoder, getI64Decoder, getUnitEncoder, getUnitDecoder, mapEncoder, getArrayEncoder, getArrayDecoder, getStringEncoder } from '@solana/codecs';
import { AccountRole, upgradeRoleToSigner } from '@solana/instructions';
import { isProgramDerivedAddress, getAddressEncoder, getAddressDecoder, getProgramDerivedAddress } from '@solana/addresses';
import { isTransactionSigner as isTransactionSigner$1 } from '@solana/signers';

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
  if (!value || !Array.isArray(value) || !isProgramDerivedAddress(value)) {
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
        role: AccountRole.READONLY
      };
      return;
    }
    const writableRole = account.isWritable ? AccountRole.WRITABLE : AccountRole.READONLY;
    accountMetas[key] = Object.freeze({
      address: expectAddress(account.value),
      role: isTransactionSigner(account.value) ? upgradeRoleToSigner(writableRole) : writableRole,
      ...isTransactionSigner(account.value) ? { signer: account.value } : {}
    });
  });
  return accountMetas;
}
function isTransactionSigner(value) {
  return !!value && typeof value === "object" && "address" in value && isTransactionSigner$1(value);
}
function memcmp(data, bytes, offset) {
  const slice = data.slice(offset, offset + bytes.length);
  if (slice.length !== bytes.length)
    return false;
  return bytes.every((b, i) => b === slice[i]);
}
function getAccountDeltaAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "AccountInfo",
      getStructEncoder([
        ["aOffset", getU16Encoder()],
        ["assertion", getAccountInfoDeltaAssertionEncoder()]
      ])
    ],
    [
      "Data",
      getStructEncoder([
        ["aOffset", getU16Encoder()],
        ["bOffset", getU16Encoder()],
        ["assertion", getDataValueDeltaAssertionEncoder()]
      ])
    ]
  ]);
}
function getAccountDeltaAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "AccountInfo",
      getStructDecoder([
        ["aOffset", getU16Decoder()],
        ["assertion", getAccountInfoDeltaAssertionDecoder()]
      ])
    ],
    [
      "Data",
      getStructDecoder([
        ["aOffset", getU16Decoder()],
        ["bOffset", getU16Decoder()],
        ["assertion", getDataValueDeltaAssertionDecoder()]
      ])
    ]
  ]);
}
function getAccountDeltaAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "Lamports",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DataLength",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Owner",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "KnownOwner",
      getStructEncoder([
        ["value", getKnownProgramEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "RentEpoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsSigner",
      getStructEncoder([
        ["value", getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "IsWritable",
      getStructEncoder([
        ["value", getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Executable",
      getStructEncoder([
        ["value", getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "VerifyDatahash",
      getStructEncoder([
        ["expectedHash", getBytesEncoder({ size: 32 })],
        ["start", getOptionEncoder(getU16Encoder())],
        ["length", getOptionEncoder(getU16Encoder())]
      ])
    ]
  ]);
}
function getAccountInfoAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Lamports",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DataLength",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Owner",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "KnownOwner",
      getStructDecoder([
        ["value", getKnownProgramDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "RentEpoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsSigner",
      getStructDecoder([
        ["value", getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "IsWritable",
      getStructDecoder([
        ["value", getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Executable",
      getStructDecoder([
        ["value", getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "VerifyDatahash",
      getStructDecoder([
        ["expectedHash", getBytesDecoder({ size: 32 })],
        ["start", getOptionDecoder(getU16Decoder())],
        ["length", getOptionDecoder(getU16Decoder())]
      ])
    ]
  ]);
}
function getAccountInfoAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "Lamports",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DataLength",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    ["Owner", getStructEncoder([["operator", getEquatableOperatorEncoder()]])],
    [
      "RentEpoch",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getAccountInfoDeltaAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Lamports",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DataLength",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    ["Owner", getStructDecoder([["operator", getEquatableOperatorDecoder()]])],
    [
      "RentEpoch",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getAccountInfoDeltaAssertionCodec() {
  return combineCodec(
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
  return getScalarEnumEncoder(AccountInfoField);
}
function getAccountInfoFieldDecoder() {
  return getScalarEnumDecoder(AccountInfoField);
}
function getAccountInfoFieldCodec() {
  return combineCodec(
    getAccountInfoFieldEncoder(),
    getAccountInfoFieldDecoder()
  );
}
function getAssertionResultEncoder() {
  return getDataEnumEncoder([
    [
      "U8",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getU8Encoder()),
            getOptionEncoder(getU8Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U16",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getU16Encoder()),
            getOptionEncoder(getU16Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U32",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getU32Encoder()),
            getOptionEncoder(getU32Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U64",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getU64Encoder()),
            getOptionEncoder(getU64Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "U128",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getU128Encoder()),
            getOptionEncoder(getU128Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I8",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getI8Encoder()),
            getOptionEncoder(getI8Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I16",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getI16Encoder()),
            getOptionEncoder(getI16Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I32",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getI32Encoder()),
            getOptionEncoder(getI32Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I64",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getI64Encoder()),
            getOptionEncoder(getI64Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "I128",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getI128Encoder()),
            getOptionEncoder(getI128Encoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Pubkey",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getAddressEncoder()),
            getOptionEncoder(getAddressEncoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Bytes",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getBytesEncoder({ size: getU32Encoder() }),
            getBytesEncoder({ size: getU32Encoder() }),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ],
    [
      "Bool",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([
            getOptionEncoder(getBooleanEncoder()),
            getOptionEncoder(getBooleanEncoder()),
            getU8Encoder(),
            getBooleanEncoder()
          ])
        ]
      ])
    ]
  ]);
}
function getAssertionResultDecoder() {
  return getDataEnumDecoder([
    [
      "U8",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getU8Decoder()),
            getOptionDecoder(getU8Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U16",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getU16Decoder()),
            getOptionDecoder(getU16Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U32",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getU32Decoder()),
            getOptionDecoder(getU32Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U64",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getU64Decoder()),
            getOptionDecoder(getU64Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "U128",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getU128Decoder()),
            getOptionDecoder(getU128Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I8",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getI8Decoder()),
            getOptionDecoder(getI8Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I16",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getI16Decoder()),
            getOptionDecoder(getI16Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I32",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getI32Decoder()),
            getOptionDecoder(getI32Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I64",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getI64Decoder()),
            getOptionDecoder(getI64Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "I128",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getI128Decoder()),
            getOptionDecoder(getI128Decoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Pubkey",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getAddressDecoder()),
            getOptionDecoder(getAddressDecoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Bytes",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getBytesDecoder({ size: getU32Decoder() }),
            getBytesDecoder({ size: getU32Decoder() }),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ],
    [
      "Bool",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([
            getOptionDecoder(getBooleanDecoder()),
            getOptionDecoder(getBooleanDecoder()),
            getU8Decoder(),
            getBooleanDecoder()
          ])
        ]
      ])
    ]
  ]);
}
function getAssertionResultCodec() {
  return combineCodec(getAssertionResultEncoder(), getAssertionResultDecoder());
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
  return getScalarEnumEncoder(ByteSliceOperator);
}
function getByteSliceOperatorDecoder() {
  return getScalarEnumDecoder(ByteSliceOperator);
}
function getByteSliceOperatorCodec() {
  return combineCodec(
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
  return getScalarEnumEncoder(ClockField);
}
function getClockFieldDecoder() {
  return getScalarEnumDecoder(ClockField);
}
function getClockFieldCodec() {
  return combineCodec(getClockFieldEncoder(), getClockFieldDecoder());
}
function getDataValueEncoder() {
  return getDataEnumEncoder([
    [
      "Bool",
      getStructEncoder([["fields", getTupleEncoder([getBooleanEncoder()])]])
    ],
    ["U8", getStructEncoder([["fields", getTupleEncoder([getU8Encoder()])]])],
    ["I8", getStructEncoder([["fields", getTupleEncoder([getI8Encoder()])]])],
    ["U16", getStructEncoder([["fields", getTupleEncoder([getU16Encoder()])]])],
    ["I16", getStructEncoder([["fields", getTupleEncoder([getI16Encoder()])]])],
    ["U32", getStructEncoder([["fields", getTupleEncoder([getU32Encoder()])]])],
    ["I32", getStructEncoder([["fields", getTupleEncoder([getI32Encoder()])]])],
    ["U64", getStructEncoder([["fields", getTupleEncoder([getU64Encoder()])]])],
    ["I64", getStructEncoder([["fields", getTupleEncoder([getI64Encoder()])]])],
    [
      "U128",
      getStructEncoder([["fields", getTupleEncoder([getU128Encoder()])]])
    ],
    [
      "I128",
      getStructEncoder([["fields", getTupleEncoder([getI128Encoder()])]])
    ],
    [
      "Bytes",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([getBytesEncoder({ size: getU32Encoder() })])
        ]
      ])
    ],
    [
      "Pubkey",
      getStructEncoder([["fields", getTupleEncoder([getAddressEncoder()])]])
    ]
  ]);
}
function getDataValueDecoder() {
  return getDataEnumDecoder([
    [
      "Bool",
      getStructDecoder([["fields", getTupleDecoder([getBooleanDecoder()])]])
    ],
    ["U8", getStructDecoder([["fields", getTupleDecoder([getU8Decoder()])]])],
    ["I8", getStructDecoder([["fields", getTupleDecoder([getI8Decoder()])]])],
    ["U16", getStructDecoder([["fields", getTupleDecoder([getU16Decoder()])]])],
    ["I16", getStructDecoder([["fields", getTupleDecoder([getI16Decoder()])]])],
    ["U32", getStructDecoder([["fields", getTupleDecoder([getU32Decoder()])]])],
    ["I32", getStructDecoder([["fields", getTupleDecoder([getI32Decoder()])]])],
    ["U64", getStructDecoder([["fields", getTupleDecoder([getU64Decoder()])]])],
    ["I64", getStructDecoder([["fields", getTupleDecoder([getI64Decoder()])]])],
    [
      "U128",
      getStructDecoder([["fields", getTupleDecoder([getU128Decoder()])]])
    ],
    [
      "I128",
      getStructDecoder([["fields", getTupleDecoder([getI128Decoder()])]])
    ],
    [
      "Bytes",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([getBytesDecoder({ size: getU32Decoder() })])
        ]
      ])
    ],
    [
      "Pubkey",
      getStructDecoder([["fields", getTupleDecoder([getAddressDecoder()])]])
    ]
  ]);
}
function getDataValueCodec() {
  return combineCodec(getDataValueEncoder(), getDataValueDecoder());
}
function dataValue(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isDataValue(kind, value) {
  return value.__kind === kind;
}
function getDataValueAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "Bool",
      getStructEncoder([
        ["value", getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "U8",
      getStructEncoder([
        ["value", getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I8",
      getStructEncoder([
        ["value", getI8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U16",
      getStructEncoder([
        ["value", getU16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I16",
      getStructEncoder([
        ["value", getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U32",
      getStructEncoder([
        ["value", getU32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I32",
      getStructEncoder([
        ["value", getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U64",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I64",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U128",
      getStructEncoder([
        ["value", getU128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I128",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Bytes",
      getStructEncoder([
        ["value", getBytesEncoder({ size: getU32Encoder() })],
        ["operator", getByteSliceOperatorEncoder()]
      ])
    ],
    [
      "Pubkey",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getDataValueAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Bool",
      getStructDecoder([
        ["value", getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "U8",
      getStructDecoder([
        ["value", getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I8",
      getStructDecoder([
        ["value", getI8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U16",
      getStructDecoder([
        ["value", getU16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I16",
      getStructDecoder([
        ["value", getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U32",
      getStructDecoder([
        ["value", getU32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I32",
      getStructDecoder([
        ["value", getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U64",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I64",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U128",
      getStructDecoder([
        ["value", getU128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I128",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Bytes",
      getStructDecoder([
        ["value", getBytesDecoder({ size: getU32Decoder() })],
        ["operator", getByteSliceOperatorDecoder()]
      ])
    ],
    [
      "Pubkey",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getDataValueAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "U8",
      getStructEncoder([
        ["value", getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I8",
      getStructEncoder([
        ["value", getI16Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U16",
      getStructEncoder([
        ["value", getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I16",
      getStructEncoder([
        ["value", getI32Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U32",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I32",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "U64",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "I64",
      getStructEncoder([
        ["value", getI128Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Bytes",
      getStructEncoder([
        ["length", getU16Encoder()],
        ["operator", getByteSliceOperatorEncoder()]
      ])
    ]
  ]);
}
function getDataValueDeltaAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "U8",
      getStructDecoder([
        ["value", getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I8",
      getStructDecoder([
        ["value", getI16Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U16",
      getStructDecoder([
        ["value", getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I16",
      getStructDecoder([
        ["value", getI32Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U32",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I32",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "U64",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "I64",
      getStructDecoder([
        ["value", getI128Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Bytes",
      getStructDecoder([
        ["length", getU16Decoder()],
        ["operator", getByteSliceOperatorDecoder()]
      ])
    ]
  ]);
}
function getDataValueDeltaAssertionCodec() {
  return combineCodec(
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
  return getScalarEnumEncoder(EquatableOperator4);
}
function getEquatableOperatorDecoder() {
  return getScalarEnumDecoder(EquatableOperator4);
}
function getEquatableOperatorCodec() {
  return combineCodec(
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
  return getScalarEnumEncoder(IntegerOperator5);
}
function getIntegerOperatorDecoder() {
  return getScalarEnumDecoder(IntegerOperator5);
}
function getIntegerOperatorCodec() {
  return combineCodec(getIntegerOperatorEncoder(), getIntegerOperatorDecoder());
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
  return getScalarEnumEncoder(KnownProgram2);
}
function getKnownProgramDecoder() {
  return getScalarEnumDecoder(KnownProgram2);
}
function getKnownProgramCodec() {
  return combineCodec(getKnownProgramEncoder(), getKnownProgramDecoder());
}
var LogLevel = /* @__PURE__ */ ((LogLevel2) => {
  LogLevel2[LogLevel2["Silent"] = 0] = "Silent";
  LogLevel2[LogLevel2["PlaintextMessage"] = 1] = "PlaintextMessage";
  LogLevel2[LogLevel2["EncodedMessage"] = 2] = "EncodedMessage";
  LogLevel2[LogLevel2["EncodedNoop"] = 3] = "EncodedNoop";
  return LogLevel2;
})(LogLevel || {});
function getLogLevelEncoder() {
  return getScalarEnumEncoder(LogLevel);
}
function getLogLevelDecoder() {
  return getScalarEnumDecoder(LogLevel);
}
function getLogLevelCodec() {
  return combineCodec(getLogLevelEncoder(), getLogLevelDecoder());
}
function getMerkleTreeAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "VerifyLeaf",
      getStructEncoder([
        ["leafIndex", getU32Encoder()],
        ["leafHash", getBytesEncoder({ size: 32 })]
      ])
    ]
  ]);
}
function getMerkleTreeAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "VerifyLeaf",
      getStructDecoder([
        ["leafIndex", getU32Decoder()],
        ["leafHash", getBytesDecoder({ size: 32 })]
      ])
    ]
  ]);
}
function getMerkleTreeAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "RentExemptReserve",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "AuthorizedStaker",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "AuthorizedWithdrawer",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "LockupUnixTimestamp",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LockupEpoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LockupCustodian",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getMetaAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "RentExemptReserve",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "AuthorizedStaker",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "AuthorizedWithdrawer",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "LockupUnixTimestamp",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LockupEpoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LockupCustodian",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getMetaAssertionCodec() {
  return combineCodec(getMetaAssertionEncoder(), getMetaAssertionDecoder());
}
function metaAssertion(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isMetaAssertion(kind, value) {
  return value.__kind === kind;
}
function getMintAccountAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "MintAuthority",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Supply",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Decimals",
      getStructEncoder([
        ["value", getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsInitialized",
      getStructEncoder([
        ["value", getBooleanEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "FreezeAuthority",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getMintAccountAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "MintAuthority",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Supply",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Decimals",
      getStructDecoder([
        ["value", getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsInitialized",
      getStructDecoder([
        ["value", getBooleanDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "FreezeAuthority",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getMintAccountAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "State",
      getStructEncoder([
        ["value", getStakeStateTypeEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "MetaAssertion",
      getStructEncoder([
        ["fields", getTupleEncoder([getMetaAssertionEncoder()])]
      ])
    ],
    [
      "StakeAssertion",
      getStructEncoder([
        ["fields", getTupleEncoder([getStakeAssertionEncoder()])]
      ])
    ],
    [
      "StakeFlags",
      getStructEncoder([
        ["value", getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getStakeAccountAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "State",
      getStructDecoder([
        ["value", getStakeStateTypeDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "MetaAssertion",
      getStructDecoder([
        ["fields", getTupleDecoder([getMetaAssertionDecoder()])]
      ])
    ],
    [
      "StakeAssertion",
      getStructDecoder([
        ["fields", getTupleDecoder([getStakeAssertionDecoder()])]
      ])
    ],
    [
      "StakeFlags",
      getStructDecoder([
        ["value", getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getStakeAccountAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "DelegationVoterPubkey",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "DelegationStake",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DelegationActivationEpoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "DelegationDeactivationEpoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "CreditsObserved",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getStakeAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "DelegationVoterPubkey",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "DelegationStake",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DelegationActivationEpoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "DelegationDeactivationEpoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "CreditsObserved",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getStakeAssertionCodec() {
  return combineCodec(getStakeAssertionEncoder(), getStakeAssertionDecoder());
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
  return getScalarEnumEncoder(StakeStateType2);
}
function getStakeStateTypeDecoder() {
  return getScalarEnumDecoder(StakeStateType2);
}
function getStakeStateTypeCodec() {
  return combineCodec(getStakeStateTypeEncoder(), getStakeStateTypeDecoder());
}
function getSysvarClockAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "Slot",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "EpochStartTimestamp",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Epoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "LeaderScheduleEpoch",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "UnixTimestamp",
      getStructEncoder([
        ["value", getI64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getSysvarClockAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Slot",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "EpochStartTimestamp",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Epoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "LeaderScheduleEpoch",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "UnixTimestamp",
      getStructDecoder([
        ["value", getI64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getSysvarClockAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "Mint",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Owner",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Amount",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "Delegate",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "State",
      getStructEncoder([
        ["value", getU8Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "IsNative",
      getStructEncoder([
        ["value", getOptionEncoder(getU64Encoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "DelegatedAmount",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ],
    [
      "CloseAuthority",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    ["TokenAccountOwnerIsDerived", getUnitEncoder()]
  ]);
}
function getTokenAccountAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Mint",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Owner",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Amount",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "Delegate",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "State",
      getStructDecoder([
        ["value", getU8Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "IsNative",
      getStructDecoder([
        ["value", getOptionDecoder(getU64Decoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "DelegatedAmount",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ],
    [
      "CloseAuthority",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    ["TokenAccountOwnerIsDerived", getUnitDecoder()]
  ]);
}
function getTokenAccountAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "Authority",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradableBufferAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "Authority",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradableBufferAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "State",
      getStructEncoder([
        ["value", getUpgradeableLoaderStateTypeEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Buffer",
      getStructEncoder([
        ["fields", getTupleEncoder([getUpgradableBufferAssertionEncoder()])]
      ])
    ],
    [
      "Program",
      getStructEncoder([
        ["fields", getTupleEncoder([getUpgradeableProgramAssertionEncoder()])]
      ])
    ],
    [
      "ProgramData",
      getStructEncoder([
        [
          "fields",
          getTupleEncoder([getUpgradeableProgramDataAssertionEncoder()])
        ]
      ])
    ]
  ]);
}
function getUpgradeableLoaderStateAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "State",
      getStructDecoder([
        ["value", getUpgradeableLoaderStateTypeDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Buffer",
      getStructDecoder([
        ["fields", getTupleDecoder([getUpgradableBufferAssertionDecoder()])]
      ])
    ],
    [
      "Program",
      getStructDecoder([
        ["fields", getTupleDecoder([getUpgradeableProgramAssertionDecoder()])]
      ])
    ],
    [
      "ProgramData",
      getStructDecoder([
        [
          "fields",
          getTupleDecoder([getUpgradeableProgramDataAssertionDecoder()])
        ]
      ])
    ]
  ]);
}
function getUpgradeableLoaderStateAssertionCodec() {
  return combineCodec(
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
  return getScalarEnumEncoder(UpgradeableLoaderStateType2);
}
function getUpgradeableLoaderStateTypeDecoder() {
  return getScalarEnumDecoder(UpgradeableLoaderStateType2);
}
function getUpgradeableLoaderStateTypeCodec() {
  return combineCodec(
    getUpgradeableLoaderStateTypeEncoder(),
    getUpgradeableLoaderStateTypeDecoder()
  );
}
function getUpgradeableProgramAssertionEncoder() {
  return getDataEnumEncoder([
    [
      "ProgramDataAddress",
      getStructEncoder([
        ["value", getAddressEncoder()],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "ProgramDataAddress",
      getStructDecoder([
        ["value", getAddressDecoder()],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "UpgradeAuthority",
      getStructEncoder([
        ["value", getOptionEncoder(getAddressEncoder())],
        ["operator", getEquatableOperatorEncoder()]
      ])
    ],
    [
      "Slot",
      getStructEncoder([
        ["value", getU64Encoder()],
        ["operator", getIntegerOperatorEncoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramDataAssertionDecoder() {
  return getDataEnumDecoder([
    [
      "UpgradeAuthority",
      getStructDecoder([
        ["value", getOptionDecoder(getAddressDecoder())],
        ["operator", getEquatableOperatorDecoder()]
      ])
    ],
    [
      "Slot",
      getStructDecoder([
        ["value", getU64Decoder()],
        ["operator", getIntegerOperatorDecoder()]
      ])
    ]
  ]);
}
function getUpgradeableProgramDataAssertionCodec() {
  return combineCodec(
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
  return getDataEnumEncoder([
    [
      "AccountData",
      getStructEncoder([
        ["offset", getU16Encoder()],
        ["dataLength", getU16Encoder()]
      ])
    ],
    [
      "AccountInfoField",
      getStructEncoder([
        ["fields", getTupleEncoder([getAccountInfoFieldEncoder()])]
      ])
    ],
    [
      "DataValue",
      getStructEncoder([["fields", getTupleEncoder([getDataValueEncoder()])]])
    ],
    [
      "Clock",
      getStructEncoder([["fields", getTupleEncoder([getClockFieldEncoder()])]])
    ]
  ]);
}
function getWriteTypeDecoder() {
  return getDataEnumDecoder([
    [
      "AccountData",
      getStructDecoder([
        ["offset", getU16Decoder()],
        ["dataLength", getU16Decoder()]
      ])
    ],
    [
      "AccountInfoField",
      getStructDecoder([
        ["fields", getTupleDecoder([getAccountInfoFieldDecoder()])]
      ])
    ],
    [
      "DataValue",
      getStructDecoder([["fields", getTupleDecoder([getDataValueDecoder()])]])
    ],
    [
      "Clock",
      getStructDecoder([["fields", getTupleDecoder([getClockFieldDecoder()])]])
    ]
  ]);
}
function getWriteTypeCodec() {
  return combineCodec(getWriteTypeEncoder(), getWriteTypeDecoder());
}
function writeType(kind, data) {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...data ?? {} };
}
function isWriteType(kind, value) {
  return value.__kind === kind;
}

// src/generated/instructions/assertAccountData.ts
function getAssertAccountDataInstructionDataEncoder() {
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["offset", getU16Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["offset", getU16Decoder()],
    ["assertion", getDataValueAssertionDecoder()]
  ]);
}
function getAssertAccountDataInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getAccountDeltaAssertionDecoder()]
  ]);
}
function getAssertAccountDeltaInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.accountA, AccountRole.READONLY),
      accountMetaWithDefault(accounts.accountB, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getAccountInfoAssertionDecoder()]
  ]);
}
function getAssertAccountInfoInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", getArrayEncoder(getAccountInfoAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 5,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertAccountInfoMultiInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", getArrayDecoder(getAccountInfoAssertionDecoder())]
  ]);
}
function getAssertAccountInfoMultiInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getMerkleTreeAssertionDecoder()]
  ]);
}
function getAssertMerkleTreeAccountInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetMerkleTree, AccountRole.READONLY),
      accountMetaWithDefault(accounts.root, AccountRole.READONLY),
      accountMetaWithDefault(
        accounts.splAccountCompression,
        AccountRole.READONLY
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getMintAccountAssertionDecoder()]
  ]);
}
function getAssertMintAccountInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", getArrayEncoder(getMintAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 7,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertMintAccountMultiInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", getArrayDecoder(getMintAccountAssertionDecoder())]
  ]);
}
function getAssertMintAccountMultiInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getStakeAccountAssertionDecoder()]
  ]);
}
function getAssertStakeAccountInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", getArrayEncoder(getStakeAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 11,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertStakeAccountMultiInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", getArrayDecoder(getStakeAccountAssertionDecoder())]
  ]);
}
function getAssertStakeAccountMultiInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getSysvarClockAssertionDecoder()]
  ]);
}
function getAssertSysvarClockInstructionDataCodec() {
  return combineCodec(
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getTokenAccountAssertionDecoder()]
  ]);
}
function getAssertTokenAccountInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      ["assertions", getArrayEncoder(getTokenAccountAssertionEncoder())]
    ]),
    (value) => ({
      ...value,
      discriminator: 9,
      logLevel: value.logLevel ?? 0 /* Silent */
    })
  );
}
function getAssertTokenAccountMultiInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertions", getArrayDecoder(getTokenAccountAssertionDecoder())]
  ]);
}
function getAssertTokenAccountMultiInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    ["assertion", getUpgradeableLoaderStateAssertionDecoder()]
  ]);
}
function getAssertUpgradeableLoaderAccountInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["logLevel", getLogLevelEncoder()],
      [
        "assertions",
        getArrayEncoder(getUpgradeableLoaderStateAssertionEncoder())
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
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["logLevel", getLogLevelDecoder()],
    [
      "assertions",
      getArrayDecoder(getUpgradeableLoaderStateAssertionDecoder())
    ]
  ]);
}
function getAssertUpgradeableLoaderAccountMultiInstructionDataCodec() {
  return combineCodec(
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
      accountMetaWithDefault(accounts.targetAccount, AccountRole.READONLY),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["memoryId", getU8Encoder()],
      ["memoryBump", getU8Encoder()]
    ]),
    (value) => ({ ...value, discriminator: 1 })
  );
}
function getMemoryCloseInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["memoryId", getU8Decoder()],
    ["memoryBump", getU8Decoder()]
  ]);
}
function getMemoryCloseInstructionDataCodec() {
  return combineCodec(
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
          role: AccountRole.READONLY
        },
        AccountRole.READONLY
      ),
      accountMetaWithDefault(accounts.payer, AccountRole.WRITABLE_SIGNER),
      accountMetaWithDefault(accounts.memory, AccountRole.WRITABLE),
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
  return mapEncoder(
    getStructEncoder([
      ["discriminator", getU8Encoder()],
      ["memoryId", getU8Encoder()],
      ["memoryBump", getU8Encoder()],
      ["writeOffset", getU16Encoder()],
      ["writeType", getWriteTypeEncoder()]
    ]),
    (value) => ({ ...value, discriminator: 0, memoryId: value.memoryId ?? 0 })
  );
}
function getMemoryWriteInstructionDataDecoder() {
  return getStructDecoder([
    ["discriminator", getU8Decoder()],
    ["memoryId", getU8Decoder()],
    ["memoryBump", getU8Decoder()],
    ["writeOffset", getU16Decoder()],
    ["writeType", getWriteTypeDecoder()]
  ]);
}
function getMemoryWriteInstructionDataCodec() {
  return combineCodec(
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
          role: AccountRole.READONLY
        },
        AccountRole.READONLY
      ),
      accountMetaWithDefault(
        accounts.systemProgram ?? "11111111111111111111111111111111",
        AccountRole.READONLY
      ),
      accountMetaWithDefault(accounts.payer, AccountRole.WRITABLE_SIGNER),
      accountMetaWithDefault(accounts.memory, AccountRole.WRITABLE),
      accountMetaWithDefault(accounts.sourceAccount, AccountRole.READONLY),
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
  return getProgramDerivedAddress({
    programAddress,
    seeds: [
      getStringEncoder({ size: "variable" }).encode("memory"),
      getAddressEncoder().encode(seeds.payer),
      getU8Encoder().encode(seeds.memoryId)
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
  if (memcmp(data, getU8Encoder().encode(0), 0)) {
    return 0 /* MemoryWrite */;
  }
  if (memcmp(data, getU8Encoder().encode(1), 0)) {
    return 1 /* MemoryClose */;
  }
  if (memcmp(data, getU8Encoder().encode(2), 0)) {
    return 2 /* AssertAccountData */;
  }
  if (memcmp(data, getU8Encoder().encode(3), 0)) {
    return 3 /* AssertAccountDelta */;
  }
  if (memcmp(data, getU8Encoder().encode(4), 0)) {
    return 4 /* AssertAccountInfo */;
  }
  if (memcmp(data, getU8Encoder().encode(5), 0)) {
    return 5 /* AssertAccountInfoMulti */;
  }
  if (memcmp(data, getU8Encoder().encode(6), 0)) {
    return 6 /* AssertMintAccount */;
  }
  if (memcmp(data, getU8Encoder().encode(7), 0)) {
    return 7 /* AssertMintAccountMulti */;
  }
  if (memcmp(data, getU8Encoder().encode(8), 0)) {
    return 8 /* AssertTokenAccount */;
  }
  if (memcmp(data, getU8Encoder().encode(9), 0)) {
    return 9 /* AssertTokenAccountMulti */;
  }
  if (memcmp(data, getU8Encoder().encode(10), 0)) {
    return 10 /* AssertStakeAccount */;
  }
  if (memcmp(data, getU8Encoder().encode(11), 0)) {
    return 11 /* AssertStakeAccountMulti */;
  }
  if (memcmp(data, getU8Encoder().encode(12), 0)) {
    return 12 /* AssertUpgradeableLoaderAccount */;
  }
  if (memcmp(data, getU8Encoder().encode(13), 0)) {
    return 13 /* AssertUpgradeableLoaderAccountMulti */;
  }
  if (memcmp(data, getU8Encoder().encode(14), 0)) {
    return 14 /* AssertSysvarClock */;
  }
  if (memcmp(data, getU8Encoder().encode(15), 0)) {
    return 15 /* AssertMerkleTreeAccount */;
  }
  throw new Error(
    "The provided instruction could not be identified as a lighthouse instruction."
  );
}

export { AccountInfoField, ByteSliceOperator, ClockField, EquatableOperator4 as EquatableOperator, IntegerOperator5 as IntegerOperator, KnownProgram2 as KnownProgram, LIGHTHOUSE_PROGRAM_ADDRESS, LighthouseInstruction, LighthouseProgramError, LighthouseProgramErrorCode, LogLevel, StakeStateType2 as StakeStateType, UpgradeableLoaderStateType2 as UpgradeableLoaderStateType, accountDeltaAssertion, accountInfoAssertion, accountInfoDeltaAssertion, accountMetaWithDefault, assertionResult, dataValue, dataValueAssertion, dataValueDeltaAssertion, expectAddress, expectProgramDerivedAddress, expectSome, expectTransactionSigner, findMemoryPda, getAccountDeltaAssertionCodec, getAccountDeltaAssertionDecoder, getAccountDeltaAssertionEncoder, getAccountInfoAssertionCodec, getAccountInfoAssertionDecoder, getAccountInfoAssertionEncoder, getAccountInfoDeltaAssertionCodec, getAccountInfoDeltaAssertionDecoder, getAccountInfoDeltaAssertionEncoder, getAccountInfoFieldCodec, getAccountInfoFieldDecoder, getAccountInfoFieldEncoder, getAccountMetasWithSigners, getAssertAccountDataInstruction, getAssertAccountDataInstructionDataCodec, getAssertAccountDataInstructionDataDecoder, getAssertAccountDataInstructionDataEncoder, getAssertAccountDataInstructionRaw, getAssertAccountDeltaInstruction, getAssertAccountDeltaInstructionDataCodec, getAssertAccountDeltaInstructionDataDecoder, getAssertAccountDeltaInstructionDataEncoder, getAssertAccountDeltaInstructionRaw, getAssertAccountInfoInstruction, getAssertAccountInfoInstructionDataCodec, getAssertAccountInfoInstructionDataDecoder, getAssertAccountInfoInstructionDataEncoder, getAssertAccountInfoInstructionRaw, getAssertAccountInfoMultiInstruction, getAssertAccountInfoMultiInstructionDataCodec, getAssertAccountInfoMultiInstructionDataDecoder, getAssertAccountInfoMultiInstructionDataEncoder, getAssertAccountInfoMultiInstructionRaw, getAssertMerkleTreeAccountInstruction, getAssertMerkleTreeAccountInstructionDataCodec, getAssertMerkleTreeAccountInstructionDataDecoder, getAssertMerkleTreeAccountInstructionDataEncoder, getAssertMerkleTreeAccountInstructionRaw, getAssertMintAccountInstruction, getAssertMintAccountInstructionDataCodec, getAssertMintAccountInstructionDataDecoder, getAssertMintAccountInstructionDataEncoder, getAssertMintAccountInstructionRaw, getAssertMintAccountMultiInstruction, getAssertMintAccountMultiInstructionDataCodec, getAssertMintAccountMultiInstructionDataDecoder, getAssertMintAccountMultiInstructionDataEncoder, getAssertMintAccountMultiInstructionRaw, getAssertStakeAccountInstruction, getAssertStakeAccountInstructionDataCodec, getAssertStakeAccountInstructionDataDecoder, getAssertStakeAccountInstructionDataEncoder, getAssertStakeAccountInstructionRaw, getAssertStakeAccountMultiInstruction, getAssertStakeAccountMultiInstructionDataCodec, getAssertStakeAccountMultiInstructionDataDecoder, getAssertStakeAccountMultiInstructionDataEncoder, getAssertStakeAccountMultiInstructionRaw, getAssertSysvarClockInstruction, getAssertSysvarClockInstructionDataCodec, getAssertSysvarClockInstructionDataDecoder, getAssertSysvarClockInstructionDataEncoder, getAssertSysvarClockInstructionRaw, getAssertTokenAccountInstruction, getAssertTokenAccountInstructionDataCodec, getAssertTokenAccountInstructionDataDecoder, getAssertTokenAccountInstructionDataEncoder, getAssertTokenAccountInstructionRaw, getAssertTokenAccountMultiInstruction, getAssertTokenAccountMultiInstructionDataCodec, getAssertTokenAccountMultiInstructionDataDecoder, getAssertTokenAccountMultiInstructionDataEncoder, getAssertTokenAccountMultiInstructionRaw, getAssertUpgradeableLoaderAccountInstruction, getAssertUpgradeableLoaderAccountInstructionDataCodec, getAssertUpgradeableLoaderAccountInstructionDataDecoder, getAssertUpgradeableLoaderAccountInstructionDataEncoder, getAssertUpgradeableLoaderAccountInstructionRaw, getAssertUpgradeableLoaderAccountMultiInstruction, getAssertUpgradeableLoaderAccountMultiInstructionDataCodec, getAssertUpgradeableLoaderAccountMultiInstructionDataDecoder, getAssertUpgradeableLoaderAccountMultiInstructionDataEncoder, getAssertUpgradeableLoaderAccountMultiInstructionRaw, getAssertionResultCodec, getAssertionResultDecoder, getAssertionResultEncoder, getByteSliceOperatorCodec, getByteSliceOperatorDecoder, getByteSliceOperatorEncoder, getClockFieldCodec, getClockFieldDecoder, getClockFieldEncoder, getDataValueAssertionCodec, getDataValueAssertionDecoder, getDataValueAssertionEncoder, getDataValueCodec, getDataValueDecoder, getDataValueDeltaAssertionCodec, getDataValueDeltaAssertionDecoder, getDataValueDeltaAssertionEncoder, getDataValueEncoder, getEquatableOperatorCodec, getEquatableOperatorDecoder, getEquatableOperatorEncoder, getIntegerOperatorCodec, getIntegerOperatorDecoder, getIntegerOperatorEncoder, getKnownProgramCodec, getKnownProgramDecoder, getKnownProgramEncoder, getLighthouseProgram, getLighthouseProgramErrorFromCode, getLogLevelCodec, getLogLevelDecoder, getLogLevelEncoder, getMemoryCloseInstruction, getMemoryCloseInstructionDataCodec, getMemoryCloseInstructionDataDecoder, getMemoryCloseInstructionDataEncoder, getMemoryCloseInstructionRaw, getMemoryWriteInstruction, getMemoryWriteInstructionDataCodec, getMemoryWriteInstructionDataDecoder, getMemoryWriteInstructionDataEncoder, getMemoryWriteInstructionRaw, getMerkleTreeAssertionCodec, getMerkleTreeAssertionDecoder, getMerkleTreeAssertionEncoder, getMetaAssertionCodec, getMetaAssertionDecoder, getMetaAssertionEncoder, getMintAccountAssertionCodec, getMintAccountAssertionDecoder, getMintAccountAssertionEncoder, getStakeAccountAssertionCodec, getStakeAccountAssertionDecoder, getStakeAccountAssertionEncoder, getStakeAssertionCodec, getStakeAssertionDecoder, getStakeAssertionEncoder, getStakeStateTypeCodec, getStakeStateTypeDecoder, getStakeStateTypeEncoder, getSysvarClockAssertionCodec, getSysvarClockAssertionDecoder, getSysvarClockAssertionEncoder, getTokenAccountAssertionCodec, getTokenAccountAssertionDecoder, getTokenAccountAssertionEncoder, getUpgradableBufferAssertionCodec, getUpgradableBufferAssertionDecoder, getUpgradableBufferAssertionEncoder, getUpgradeableLoaderStateAssertionCodec, getUpgradeableLoaderStateAssertionDecoder, getUpgradeableLoaderStateAssertionEncoder, getUpgradeableLoaderStateTypeCodec, getUpgradeableLoaderStateTypeDecoder, getUpgradeableLoaderStateTypeEncoder, getUpgradeableProgramAssertionCodec, getUpgradeableProgramAssertionDecoder, getUpgradeableProgramAssertionEncoder, getUpgradeableProgramDataAssertionCodec, getUpgradeableProgramDataAssertionDecoder, getUpgradeableProgramDataAssertionEncoder, getWriteTypeCodec, getWriteTypeDecoder, getWriteTypeEncoder, identifyLighthouseInstruction, isAccountDeltaAssertion, isAccountInfoAssertion, isAccountInfoDeltaAssertion, isAssertionResult, isDataValue, isDataValueAssertion, isDataValueDeltaAssertion, isMerkleTreeAssertion, isMetaAssertion, isMintAccountAssertion, isStakeAccountAssertion, isStakeAssertion, isSysvarClockAssertion, isTokenAccountAssertion, isTransactionSigner, isUpgradableBufferAssertion, isUpgradeableLoaderStateAssertion, isUpgradeableProgramAssertion, isUpgradeableProgramDataAssertion, isWriteType, memcmp, merkleTreeAssertion, metaAssertion, mintAccountAssertion, parseAssertAccountDataInstruction, parseAssertAccountDeltaInstruction, parseAssertAccountInfoInstruction, parseAssertAccountInfoMultiInstruction, parseAssertMerkleTreeAccountInstruction, parseAssertMintAccountInstruction, parseAssertMintAccountMultiInstruction, parseAssertStakeAccountInstruction, parseAssertStakeAccountMultiInstruction, parseAssertSysvarClockInstruction, parseAssertTokenAccountInstruction, parseAssertTokenAccountMultiInstruction, parseAssertUpgradeableLoaderAccountInstruction, parseAssertUpgradeableLoaderAccountMultiInstruction, parseMemoryCloseInstruction, parseMemoryWriteInstruction, stakeAccountAssertion, stakeAssertion, sysvarClockAssertion, tokenAccountAssertion, upgradableBufferAssertion, upgradeableLoaderStateAssertion, upgradeableProgramAssertion, upgradeableProgramDataAssertion, writeType };
//# sourceMappingURL=out.js.map
//# sourceMappingURL=index.mjs.map