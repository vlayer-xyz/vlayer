import js from "@eslint/js";
import ts from 'typescript-eslint';
import globals from "globals";

export default [
    js.configs.recommended,
    ...ts.configs.recommended,
   {
      rules: {
        "no-unused-vars": "warn",
        "no-undef": "warn"
      },
      languageOptions: {
        globals: {
            ...globals.browser,
        }   
      }   
   }
];