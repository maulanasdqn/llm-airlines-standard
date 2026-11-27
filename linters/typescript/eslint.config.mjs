import tseslint from "typescript-eslint";

const DENIED_NAMES = ["addr", "arr", "attr", "attrs", "baz", "btn", "buf", "cb", "cfg", "cnt", "conf", "ctrl", "cur", "curr", "data", "el", "elem", "evt", "fn", "foo", "func", "handler", "helper", "helpers", "idx", "impl", "info", "len", "lst", "manager", "mgr", "misc", "msg", "msgs", "num", "obj", "passwd", "prev", "processor", "pwd", "res", "resp", "ret", "srv", "str", "stuff", "svc", "thing", "things", "tmp", "usr", "util", "utils", "val"];
const SOURCE_FILES = ["**/*.{js,jsx,mjs,cjs,ts,tsx,mts,cts}"];
const TYPED_FILES = ["**/*.{ts,tsx,mts,cts}"];
const BOOLEAN_PREFIXES = ["is", "has", "can", "should"];
const SMALL_NUMBERS = [-1, 0, 1, 2];
const COMMENT_TERMS = ["todo", "fixme", "xxx", "hack", "note", "warning", "caution"];

const coreRules = {
  "id-denylist": ["warn", ...DENIED_NAMES],
  "max-depth": ["warn", 3],
  "max-params": ["warn", 3],
  "max-lines-per-function": [
    "warn",
    { max: 40, skipBlankLines: true, skipComments: true },
  ],
  "max-lines": ["error", { max: 200, skipBlankLines: true, skipComments: false }],
  complexity: ["warn", 10],
  "no-magic-numbers": ["warn", { ignore: SMALL_NUMBERS }],
  "no-nested-ternary": "warn",
  "no-negated-condition": "warn",
  "no-implicit-coercion": "warn",
  "no-cond-assign": ["error", "always"],
  "no-sequences": "error",
  "no-else-return": ["warn", { allowElseIf: false }],
  "no-inline-comments": "error",
  "no-warning-comments": ["error", { terms: COMMENT_TERMS, location: "anywhere" }],
};

const namingRules = [
  "warn",
  { selector: "default", format: ["camelCase"], leadingUnderscore: "allow" },
  { selector: "variable", format: ["camelCase", "UPPER_CASE", "PascalCase"] },
  { selector: "function", format: ["camelCase", "PascalCase"] },
  { selector: "typeLike", format: ["PascalCase"] },
  { selector: "enumMember", format: ["PascalCase", "UPPER_CASE"] },
  { selector: ["objectLiteralProperty", "typeProperty", "objectLiteralMethod"], format: null },
  { selector: "import", format: null },
  { selector: ["variable", "parameter"], types: ["boolean"], format: ["PascalCase"], prefix: BOOLEAN_PREFIXES },
];

const typedRules = {
  "max-params": "off",
  "no-magic-numbers": "off",
  "@typescript-eslint/max-params": ["warn", { max: 3 }],
  "@typescript-eslint/naming-convention": namingRules,
  "@typescript-eslint/no-magic-numbers": [
    "warn",
    {
      ignore: SMALL_NUMBERS,
      ignoreEnums: true,
      ignoreNumericLiteralTypes: true,
      ignoreReadonlyClassProperties: true,
      ignoreTypeIndexes: true,
    },
  ],
  "@typescript-eslint/explicit-module-boundary-types": "warn",
};

export default [
  { files: SOURCE_FILES, rules: coreRules },
  {
    files: TYPED_FILES,
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: { projectService: true, tsconfigRootDir: import.meta.dirname },
    },
    plugins: { "@typescript-eslint": tseslint.plugin },
    rules: typedRules,
  },
];
