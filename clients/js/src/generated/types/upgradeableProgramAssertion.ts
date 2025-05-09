/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
  dataEnum,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  EquatableOperator,
  EquatableOperatorArgs,
  getEquatableOperatorSerializer,
} from '.';

export type UpgradeableProgramAssertion = {
  __kind: 'ProgramDataAddress';
  value: PublicKey;
  operator: EquatableOperator;
};

export type UpgradeableProgramAssertionArgs = {
  __kind: 'ProgramDataAddress';
  value: PublicKey;
  operator: EquatableOperatorArgs;
};

export function getUpgradeableProgramAssertionSerializer(): Serializer<
  UpgradeableProgramAssertionArgs,
  UpgradeableProgramAssertion
> {
  return dataEnum<UpgradeableProgramAssertion>(
    [
      [
        'ProgramDataAddress',
        struct<
          GetDataEnumKindContent<
            UpgradeableProgramAssertion,
            'ProgramDataAddress'
          >
        >([
          ['value', publicKeySerializer()],
          ['operator', getEquatableOperatorSerializer()],
        ]),
      ],
    ],
    { description: 'UpgradeableProgramAssertion' }
  ) as Serializer<UpgradeableProgramAssertionArgs, UpgradeableProgramAssertion>;
}

// Data Enum Helpers.
export function upgradeableProgramAssertion(
  kind: 'ProgramDataAddress',
  data: GetDataEnumKindContent<
    UpgradeableProgramAssertionArgs,
    'ProgramDataAddress'
  >
): GetDataEnumKind<UpgradeableProgramAssertionArgs, 'ProgramDataAddress'>;
export function upgradeableProgramAssertion<
  K extends UpgradeableProgramAssertionArgs['__kind'],
>(
  kind: K,
  data?: any
): Extract<UpgradeableProgramAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}
export function isUpgradeableProgramAssertion<
  K extends UpgradeableProgramAssertion['__kind'],
>(
  kind: K,
  value: UpgradeableProgramAssertion
): value is UpgradeableProgramAssertion & { __kind: K } {
  return value.__kind === kind;
}
