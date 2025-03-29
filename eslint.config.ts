import js from "@eslint/js";
import ts from "typescript-eslint";
import prettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";
import eslintPluginReact from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";

// One single eslint config for whole monorepo
// thx flat config 
export default [
  {
    ignores: [
      "**/node_modules/**/*", 
      "**/dist/**/*",
      "book/**/*",
      "contracts/**/*",
      "rust/**/*",
      "docker/**/*",
      "eslint.config.ts",
    ],
  },
  js.configs.recommended,
  ...(ts.configs.recommendedTypeChecked).map(config => ({
    ...config,
    ignores: [
      "eslint.config.ts",
    ],
  })),
  prettierRecommended,
  // eslintPluginReact.configs.recommended,
  
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
      },
      parserOptions: {
        // find closest tsconfig.json
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  // React rules for packages that use react 
  // and examples
  // {
  //   files: [
  //     "packages/sdk-hooks/**/*.{ts,tsx,js,jsx}", 
  //     "packages/extension-hooks/**/*.{ts,tsx,js,jsx}", 
  //     "examples/**/*.{ts,tsx,js,jsx}"
  //   ],
  //   rules: {
  //     ...eslintPluginReact.configs.recommended.rules,
  //     ...reactHooks.configs.recommended.rules,
  //   },
  //   settings: {
  //     react: {
  //       version: "detect",
  //     },
  //   },
  // },


  {
    files: [
      "examples/**/*.{ts,tsx,js,jsx}"
    ],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  }

];
