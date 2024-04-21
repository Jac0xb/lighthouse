module.exports = {
  extends: ['airbnb-base', 'airbnb-typescript/base', 'prettier'],
  plugins: ['prettier'],
  overrides: [],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: 'tsconfig.json',
    tsconfigRootDir: __dirname,
  },
  rules: {
    '@typescript-eslint/no-use-before-define': 'off',
    '@typescript-eslint/no-unused-vars': 'off',
    'class-methods-use-this': 'off',
    'import/prefer-default-export': 'off',
    'import/no-cycle': 'off',
    'no-underscore-dangle': 'off',
    'max-classes-per-file': 'off',
    'no-param-reassign': 'off',
    'func-names': 'off',
    'prefer-destructuring': 'off',
  },
  ignorePatterns: ['dist/**', '.eslintrc.js', 'tsup.config.ts', 'env-shim.ts'],
};
