import fs from 'fs';
import prettier from 'prettier';
import { AssertionTemplateArgs } from './renderer/assertionTemplate/args';
import { CompiledTypescriptAssertionTemplate } from './renderer/assertionTemplate/typescriptTemplate';
import { CompiledRustAssertionTemplate } from './renderer/assertionTemplate/rustTemplate';
import appRoot from 'app-root-path';
import { LanguageOutput } from './renderer';
import { CompiledTypescriptPreviewAssertionTemplate } from './renderer/assertionTemplate/typescriptPreviewTemplate';
import { to_snake_case } from './utils';
import { voteAccountTemplateArgs } from './registry/spl';

async function generateAssertionFilesFromTemplate(args: AssertionTemplateArgs) {
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
        code = CompiledTypescriptAssertionTemplate(args);
        code = await tsPrettier(code);

        fs.writeFileSync(
          appRoot.resolve(
            `../js/src/registry/${args.assertionName}Assertion.ts`
          ),
          code
        );

        console.log(
          `Generated ${args.assertionName}Assertion.ts for ${language}`
        );

        break;
      case 'typescript-preview':
        code = CompiledTypescriptPreviewAssertionTemplate(args);
        code = await tsPrettier(code);

        fs.writeFileSync(
          appRoot.resolve(
            `../kit-js/src/registry/${args.assertionName}Assertion.ts`
          ),
          code
        );

        console.log(
          `Generated ${args.assertionName}Assertion.ts for ${language}`
        );

        break;
      case 'rust':
        fs.writeFileSync(
          appRoot.resolve(
            `../rust/src/registry/${to_snake_case(
              args.assertionName
            )}_assertion.rs`
          ),
          CompiledRustAssertionTemplate(args)
        );
        console.log(
          `Generated ${to_snake_case(args.assertionName)}_assertion.rs for ${language}`
        );
        break;
    }
  }
}

generateAssertionFilesFromTemplate(voteAccountTemplateArgs);
