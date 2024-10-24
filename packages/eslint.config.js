import js from "@eslint/js";
import ts from "typescript-eslint";
import prettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";

export default [
  js.configs.recommended,
  ...ts.configs.recommendedTypeChecked,
  prettierRecommended,
  {
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
