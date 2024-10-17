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
      "no-unused-vars": "warn",
      "no-undef": "warn",

       // Rules enabled as a baseline after enabling recommendedTypeChecked.
       // These should be eventually upgraded to "error" and all related issues should be resolved.
      "@typescript-eslint/no-unsafe-member-access": "warn",
      "@typescript-eslint/no-unsafe-argument": "warn",
      "@typescript-eslint/no-unsafe-assignment": "warn",
      "@typescript-eslint/no-unsafe-return": "warn",
      "@typescript-eslint/no-unnecessary-type-assertion": "warn",
      "@typescript-eslint/no-base-to-string": "warn",
      "@typescript-eslint/restrict-template-expressions": "warn",
      "@typescript-eslint/no-redundant-type-constituents": "warn",

      "@typescript-eslint/no-misused-promises": "warn",
      "@typescript-eslint/require-await": "warn", //VlayerClient.prove() is async for the future http request in it
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
