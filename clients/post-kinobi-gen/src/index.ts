import fs from 'fs';
import prettier from 'prettier';
import {
  AssertionTemplateArgs,
  IntegerOperator,
} from './renderer/assertionTemplate/args';
import { CompiledTypescriptAssertionTemplate } from './renderer/assertionTemplate/typescriptTemplate';
import { CompiledRustAssertionTemplate } from './renderer/assertionTemplate/rustTemplate';
import appRoot from 'app-root-path';
import {
  DefaultValueFn,
  DefaultValueOperatorFn,
} from './renderer/assertionTemplate/valueRenderer';
import { LanguageOutput } from './renderer';
import { CompiledTypescriptPreviewAssertionTemplate } from './renderer/assertionTemplate/typescriptPreviewTemplate';

const assertions: AssertionTemplateArgs = {
  assertionName: 'VoteAccount',
  variants: [
    {
      name: 'AuthorizedWithdrawer',
      kind: 'Pubkey',
      assertions: [
        {
          kind: 'U8',
          value: 2,
          valueOperator: { type: 'Integer', enum: IntegerOperator.Equal },
          offset: 0,
        },
        {
          kind: 'Pubkey',
          value: DefaultValueFn,
          valueOperator: DefaultValueOperatorFn,
          offset: 36,
        },
      ],
    },
  ],
};

(async () => {
  const langauges: LanguageOutput[] = [
    'typescript',
    'typescript-preview',
    'rust',
  ];

  const tsPrettier = (str: string) =>
    prettier.format(str, {
      semi: true,
      singleQuote: true,
      trailingComma: 'es5',
      useTabs: false,
      tabWidth: 2,
      arrowParens: 'always',
      printWidth: 80,
      parser: 'typescript',
    });

  for (const language of langauges) {
    let code = '';

    switch (language) {
      case 'typescript':
        code = CompiledTypescriptAssertionTemplate(assertions);
        // code = await tsPrettier(code);

        fs.writeFileSync(
          appRoot.resolve(
            `../js/src/registry/${assertions.assertionName}Assertion.ts`
          ),
          code
        );

        console.log(
          `Generated ${assertions.assertionName}Assertion.ts for ${language}`
        );

        break;
      case 'typescript-preview':
        code = CompiledTypescriptPreviewAssertionTemplate(assertions);
        // code = await tsPrettier(code);

        fs.writeFileSync(
          appRoot.resolve(
            `../preview-js/src/registry/${assertions.assertionName}Assertion.ts`
          ),
          code
        );

        console.log(
          `Generated ${assertions.assertionName}Assertion.ts for ${language}`
        );

        break;
      case 'rust':
        fs.writeFileSync(
          appRoot.resolve(
            `../rust/src/registry/${to_snake_case(
              assertions.assertionName
            )}_assertion.rs`
          ),
          CompiledRustAssertionTemplate(assertions)
        );
        console.log(
          `Generated ${to_snake_case(assertions.assertionName)}_assertion.rs for ${language}`
        );
        break;
    }
  }
})();

function to_snake_case(str: string) {
  return str
    .replace(/([A-Z])/g, (match) => `_${match.toLowerCase()}`)
    .replace(/^_/, '');
}
