import eslint from "@eslint/js";
import prettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";
import eslintPluginReact from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import tseslint from 'typescript-eslint';
import importPlugin from "eslint-plugin-import";
// Unified ESLint configuration for the entire monorepo using flat config
export default tseslint.config(
  {
    ignores: [
      "**/node_modules/**/*", 
      "**/dist/**/*",
      "book/**/*",
      "contracts/**/*",
      "rust/**/*",
      "docker/**/*",
      "**/out/**/*",
      '**/playwright-report/**/*',
      "eslint.config.ts",
    ],
  },
  eslint.configs.recommended,
  // Using the stricter recommendedTypeChecked configuration instead of the standard recommended
  tseslint.configs.recommendedTypeChecked,
  reactHooks.configs['recommended-latest'],
  prettierRecommended,
  {
    plugins: {
      react: eslintPluginReact,
    },
    // Disable the rule for React in JSX scope as React version >= 18 is used
    rules: {
      "react/react-in-jsx-scope": "off",
    },
  },
  {
    ignores: [
      '**/web-proof-commons/**/*',
    ],
    plugins: {
      import: importPlugin,
    },
    rules: {
      "import/no-extraneous-dependencies": "error",
    },
  },
  {
    files: [
      "packages/**/*.{ts,tsx,js,jsx}",
      "examples/**/*.{ts,tsx,js,jsx}",
    ],
    rules: {
      curly: "error",
      "no-console": ["error", { allow: ["warn", "error"] }],
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        // Always find closest tsconfig.json
        projectService: true,
        // @ts-ignore
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  //For now console is allowed in examples 
  {
    files: [
      'examples/**/*.{ts,tsx,js,jsx}',
    ],
    rules: {
      "no-console": "off",
    },
  },
  // Disable type-checking rules for specific files to avoid forcing their inclusion in tsconfig.json
  {
    files: [
      '**/vitest.config.ts',
      '**/vitest.setup.ts',
      '**/vite.config.ts',
      '**/vite-env.d.ts',
      '**/playwright.config.ts',
      '**/*.js'
    ],
    extends: [tseslint.configs.disableTypeChecked],
  },
)
