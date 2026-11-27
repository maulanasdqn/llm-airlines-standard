#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixtures="$root/tests/fixtures"
work="${STE_WORK_DIR:-$(mktemp -d)}"
vale_command="${VALE:-vale}"
golangci_command="${GOLANGCI_LINT:-golangci-lint}"
failures=0

has_command() {
  command -v "$1" >/dev/null 2>&1
}

report_skip() {
  echo "SKIP $1: $2 is not installed."
}

expect_rules() {
  local tool="$1" output="$2" rule
  shift 2
  for rule in "$@"; do
    if ! grep -qF -- "$rule" <<<"$output"; then
      echo "FAIL $tool: the bad fixture did not break $rule."
      failures=$((failures + 1))
    fi
  done
}

expect_clean() {
  if [ -n "$2" ]; then
    echo "FAIL $1: the good fixture has findings:"
    echo "$2"
    failures=$((failures + 1))
  fi
}

report_result() {
  if [ "$failures" -eq "$2" ]; then
    echo "PASS $1"
  fi
}

verify_vale() {
  if ! has_command "$vale_command"; then
    report_skip vale "$vale_command"
    return
  fi
  local config="$root/linters/vale/.vale.ini" before="$failures" bad good
  bad="$("$vale_command" --config "$config" --output line "$fixtures/prose/bad.md" || true)"
  good="$("$vale_command" --config "$config" --output line "$fixtures/prose/good.md" || true)"
  expect_rules vale "$bad" STE.SentenceLength STE.ParagraphLength STE.WordsToAvoid STE.PresentPerfect \
    STE.Passive STE.Progressive STE.Hedges STE.Joiners STE.Markers
  expect_clean vale "$good"
  report_result vale "$before"
}

run_ruff() {
  if has_command ruff; then
    ruff "$@"
  else
    uvx ruff "$@"
  fi
}

verify_ruff() {
  if ! has_command ruff && ! has_command uvx; then
    report_skip ruff "ruff or uvx"
    return
  fi
  local config="$root/linters/python/ruff.toml" before="$failures" bad good
  bad="$(run_ruff check --no-cache --config "$config" --output-format json "$fixtures/code/python/bad.py" || true)"
  good="$(run_ruff check --no-cache --quiet --config "$config" "$fixtures/code/python/good.py" || true)"
  expect_rules ruff "$bad" PLR0917 PLR1702 PLR0915 PLR2004 ANN001 ANN201 FBT001
  expect_clean ruff "$good"
  report_result ruff "$before"
}

install_eslint() {
  local folder="$1"
  mkdir -p "$folder"
  if [ ! -d "$folder/node_modules/typescript-eslint" ]; then
    (cd "$folder" && npm init -y >/dev/null && npm install --no-audit --no-fund --silent eslint typescript-eslint typescript)
  fi
  cp "$root/linters/typescript/eslint.config.mjs" "$folder/eslint.config.mjs"
  rm -rf "$folder/fixture"
  cp -R "$fixtures/code/typescript" "$folder/fixture"
}

verify_eslint() {
  if ! has_command npm; then
    report_skip eslint npm
    return
  fi
  local folder="$work/eslint" before="$failures" bad good
  install_eslint "$folder"
  bad="$(cd "$folder" && npx eslint --format json fixture/bad.ts || true)"
  good="$(cd "$folder" && npx eslint fixture/good.ts || true)"
  expect_rules eslint "$bad" max-depth max-lines-per-function no-nested-ternary no-negated-condition \
    no-implicit-coercion @typescript-eslint/max-params @typescript-eslint/naming-convention \
    @typescript-eslint/no-magic-numbers @typescript-eslint/explicit-module-boundary-types
  expect_clean eslint "$good"
  report_result eslint "$before"
}

verify_golangci() {
  if ! has_command "$golangci_command"; then
    report_skip golangci-lint "$golangci_command"
    return
  fi
  local config="$root/linters/go/.golangci.yml" before="$failures" bad good
  "$golangci_command" config verify --config "$config"
  bad="$(cd "$fixtures/code/go" && "$golangci_command" run --config "$config" ./bad/... || true)"
  good="$(cd "$fixtures/code/go" && "$golangci_command" run --config "$config" ./good/... | grep -v '^0 issues' || true)"
  expect_rules golangci-lint "$bad" argument-limit max-control-nesting error-strings function-length \
    flag-parameter "(mnd)"
  expect_clean golangci-lint "$good"
  report_result golangci-lint "$before"
}

run_clippy() {
  CLIPPY_CONF_DIR="$root/linters/rust" CARGO_TARGET_DIR="$work/cargo-target" \
    cargo clippy --offline --quiet --manifest-path "$fixtures/code/rust/$1/Cargo.toml" --message-format json \
    2>/dev/null | grep -oE '"code":"clippy::[a-z_]+"' || true
}

verify_clippy() {
  if ! has_command cargo; then
    report_skip clippy cargo
    return
  fi
  local before="$failures" bad good
  bad="$(run_clippy bad)"
  good="$(run_clippy good)"
  expect_rules clippy "$bad" clippy::too_many_arguments clippy::too_many_lines clippy::excessive_nesting \
    clippy::disallowed_names clippy::unwrap_used
  expect_clean clippy "$good"
  report_result clippy "$before"
}

main() {
  verify_vale
  verify_ruff
  verify_eslint
  verify_golangci
  verify_clippy
  if [ "$failures" -gt 0 ]; then
    echo "$failures linter checks failed."
    exit 1
  fi
  echo "All installed linters passed."
}

main
