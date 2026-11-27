#!/usr/bin/env bash
set -euo pipefail

repository="${STE_REPOSITORY:-maulanasdqn/llm-airlines-standard}"
version="${STE_VERSION:-latest}"
install_folder="${STE_INSTALL_DIR:-$HOME/.local/bin}"
download_url="${STE_DOWNLOAD_URL:-}"
work_folder=""

remove_work() {
  if [ -n "$work_folder" ]; then
    rm -rf "$work_folder"
  fi
}

can_open_terminal() {
  { : </dev/tty; } 2>/dev/null
}

find_target() {
  local system machine
  system="$(uname -s)"
  machine="$(uname -m)"
  case "$machine" in
    x86_64 | amd64) machine="x86_64" ;;
    arm64 | aarch64) machine="aarch64" ;;
    *)
      echo "install.sh: cannot install ste: no release build exists for the CPU type $machine. Run cargo install --git https://github.com/$repository." >&2
      exit 2
      ;;
  esac
  case "$system" in
    Darwin) echo "$machine-apple-darwin" ;;
    Linux) echo "$machine-unknown-linux-musl" ;;
    *)
      echo "install.sh: cannot install ste: no release build exists for $system. Download a build from https://github.com/$repository/releases." >&2
      exit 2
      ;;
  esac
}

find_base_url() {
  if [ -n "$download_url" ]; then
    echo "$download_url"
  elif [ "$version" = "latest" ]; then
    echo "https://github.com/$repository/releases/latest/download"
  else
    echo "https://github.com/$repository/releases/download/$version"
  fi
}

compute_sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d' ' -f1
  else
    shasum -a 256 "$1" | cut -d' ' -f1
  fi
}

install_binary() {
  local archive base expected actual
  archive="ste-$(find_target).tar.gz"
  base="$(find_base_url)"
  work_folder="$(mktemp -d)"
  trap remove_work EXIT
  curl -fsSL "$base/$archive" -o "$work_folder/$archive"
  curl -fsSL "$base/$archive.sha256" -o "$work_folder/$archive.sha256"
  expected="$(cut -d' ' -f1 <"$work_folder/$archive.sha256")"
  actual="$(compute_sha256 "$work_folder/$archive")"
  if [ "$expected" != "$actual" ]; then
    echo "install.sh: the SHA-256 of $archive does not match the release. Do not use this download. Try again later." >&2
    exit 1
  fi
  tar -xzf "$work_folder/$archive" -C "$work_folder"
  mkdir -p "$install_folder"
  install -m 0755 "$work_folder/ste" "$install_folder/ste"
  echo "Installed ste in $install_folder/ste."
  case ":$PATH:" in
    *":$install_folder:"*) ;;
    *) echo "Add $install_folder to your PATH to run ste." ;;
  esac
}

run_skill_installer() {
  local binary="$1"
  shift
  if [ -t 0 ] || ! can_open_terminal; then
    "$binary" install "$@"
  else
    "$binary" install "$@" </dev/tty
  fi
}

main() {
  local binary="${STE_BINARY:-}"
  local is_binary_only="no"
  if [ "${1:-}" = "--binary-only" ]; then
    is_binary_only="yes"
    shift
  fi
  if [ -z "$binary" ]; then
    install_binary
    binary="$install_folder/ste"
  fi
  if [ "$is_binary_only" = "no" ]; then
    run_skill_installer "$binary" "$@"
  fi
}

main "$@"
