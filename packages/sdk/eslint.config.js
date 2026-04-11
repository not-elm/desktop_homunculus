import js from '@eslint/js';
import { globalIgnores } from 'eslint/config';
import tseslint from 'typescript-eslint';

export default tseslint.config([
  globalIgnores(['dist', 'rollup.config.ts']),
  {
    files: ['**/*.ts'],
    extends: [js.configs.recommended, tseslint.configs.recommended],
    rules: {
      '@typescript-eslint/no-namespace': 'off',
      '@typescript-eslint/consistent-type-imports': [
        'error',
        {
          prefer: 'type-imports',
          fixStyle: 'inline-type-imports',
        },
      ],
      '@typescript-eslint/ban-ts-comment': [
        'error',
        {
          'ts-ignore': true,
          'ts-expect-error': 'allow-with-description',
        },
      ],
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
        },
      ],
      // Phase 1: warning-only guidance while migrating to function-first style.
      'no-restricted-syntax': [
        'warn',
        {
          selector:
            ":matches(Program, TSModuleBlock) > ExportNamedDeclaration > VariableDeclaration > VariableDeclarator[init.type='ArrowFunctionExpression']",
          message: 'Use function declarations for exported top-level APIs.',
        },
        {
          selector:
            ":matches(Program, TSModuleBlock) > VariableDeclaration > VariableDeclarator[init.type='ArrowFunctionExpression']",
          message: 'Use function declarations for top-level helpers.',
        },
      ],
    },
  },
]);
