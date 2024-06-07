import { Kind, renderKind } from '../kind';
import fs from 'fs';
import Handlebars from 'handlebars';
import { LanguageOutput } from '..';

export type Operator =
  | {
      type: 'Equatable';
      enum: EquatableOperator;
    }
  | {
      type: 'Integer';
      enum: IntegerOperator;
    };

export enum EquatableOperator {
  Equal = 'Equal',
  NotEqual = 'NotEqual',
}

export enum IntegerOperator {
  Equal = 'Equal',
  NotEqual = 'NotEqual',
  GreaterThan = 'GreaterThan',
  LessThan = 'LessThan',
  GreaterThanOrEqual = 'GreaterThanOrEqual',
  LessThanOrEqual = 'LessThanOrEqual',
  Contains = 'Contains',
  DoesNotContain = 'DoesNotContain',
}

export type AssertionTemplateArgs = {
  assertionName: string;
  variants: VariantArgs[];
};

export type VariantArgs = {
  name: string;
  kind: Kind;
  assertions: AssertionArgs[];
};

export type AssertionArgs = {
  kind: Kind;
  value: ((language: LanguageOutput) => string) | string | number;
  valueOperator: Operator | ((language: LanguageOutput) => string);
  offset: number;
};

export type RenderedAssertionTemplateArgs = {
  assertionName: string;
  variants: RenderedVariantArgs[];
};

export type RenderedVariantArgs = {
  name: string;
  valueType: string;
  operatorType: string;
  assertions: RenderedAssertionArgs[];
};

export type RenderedAssertionArgs = {
  kind: string;
  value: any;
  valueOperator: string;
  offset: number;
};

function renderOperatorValue(
  operator: Operator | ((language: LanguageOutput) => string),
  language: LanguageOutput
): string {
  if (typeof operator === 'function') {
    return operator(language);
  }

  switch (language) {
    case 'typescript-preview':
    case 'typescript':
      if (operator.type === 'Equatable') {
        return `EquatableOperator.${operator.enum}`;
      } else {
        return `IntegerOperator.${operator.enum}`;
      }
    case 'rust':
      if (operator.type === 'Equatable') {
        return `crate::types::EquatableOperator::${EquatableOperator[operator.enum]}`;
      } else {
        return `crate::types::IntegerOperator::${IntegerOperator[operator.enum]}`;
      }
    default:
      throw new Error('Unsupported language');
  }
}

export function renderAssertionArgs<
  T extends {
    assertionName: string;
    variants: VariantArgs[];
  },
>(templateArgs: T, language: LanguageOutput) {
  const renderedVariants: RenderedVariantArgs[] = templateArgs.variants.map(
    (variant) => {
      const renderedAssertions: RenderedAssertionArgs[] =
        variant.assertions.map((assertion) => {
          return {
            value:
              assertion.value instanceof Function
                ? assertion.value(language)
                : assertion.value,
            valueOperator: renderOperatorValue(
              assertion.valueOperator,
              language
            ),
            offset: assertion.offset,
            ...renderKind(assertion.kind, language),
          };
        });

      return {
        name: variant.name,
        ...renderKind(variant.kind, language),
        assertions: renderedAssertions,
      };
    }
  );

  return {
    assertionName: templateArgs.assertionName,
    variants: renderedVariants,
  };
}
