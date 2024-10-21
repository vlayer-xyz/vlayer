import js from "@eslint/js";
import ts from "typescript-eslint";
import prettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";

export default [
  js.configs.recommended,
  ...ts.configs.recommendedTypeChecked,
  prettierRecommended,
  {
    rules: {
       // Rules enabled as a baseline after enabling recommendedTypeChecked.
       // These should be eventually upgraded to "error" and all related issues should be resolved.
      "@typescript-eslint/no-misused-promises": "warn",
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        Bun: false,
      },
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
];
