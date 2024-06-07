import { LanguageOutput } from '..';

export function DefaultValueOperatorFn(lang: LanguageOutput) {
  switch (lang) {
    case 'typescript-preview':
      return 'input.assertion.operator';
    case 'typescript':
      return 'input.operator';
    case 'rust':
      return 'operator';
    default:
      throw new Error(`Unsupported language: ${lang}`);
  }
}

export function DefaultValueFn(lang: LanguageOutput) {
  switch (lang) {
    case 'typescript-preview':
      return 'input.assertion.value';
    case 'typescript':
      return 'input.value';
    case 'rust':
      return 'value';
    default:
      throw new Error(`Unsupported language: ${lang}`);
  }
}
