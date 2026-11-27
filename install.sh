#!/usr/bin/env bash
set -euo pipefail

unset_repository="OWNER/llm-airlines-standard"
repository="${STE_REPOSITORY:-$unset_repository}"
reference="${STE_REF:-main}"
archive_url="${STE_ARCHIVE_URL:-}"
download_folder=""

remove_download() {
  if [ -n "$download_folder" ]; then
    rm -rf "$download_folder"
  fi
}

find_local_root() {
  local source="${BASH_SOURCE[0]:-}"
  if [ -n "$source" ] && [ -f "$(dirname "$source")/tools/install.py" ]; then
    (cd "$(dirname "$source")" && pwd)
  fi
}

can_open_terminal() {
  { : </dev/tty; } 2>/dev/null
}

download_repository() {
  if [ -z "$archive_url" ] && [ "$repository" = "$unset_repository" ]; then
    echo "install.sh: cannot download the skills: the repository is not set. Set STE_REPOSITORY to the GitHub path, such as acme/llm-airlines-standard, or set STE_ARCHIVE_URL." >&2
    exit 2
  fi
  download_folder="$(mktemp -d)"
  trap remove_download EXIT
  curl -fsSL "${archive_url:-https://codeload.github.com/$repository/tar.gz/$reference}" | tar -xz -C "$download_folder" --strip-components=1
}

run_installer() {
  local root="$1"
  shift
  if [ -t 0 ] || ! can_open_terminal; then
    python3 "$root/tools/install.py" "$@"
  else
    python3 "$root/tools/install.py" "$@" </dev/tty
  fi
}

main() {
  local root
  root="$(find_local_root)"
  if [ -z "$root" ]; then
    download_repository
    root="$download_folder"
  fi
  run_installer "$root" "$@"
}

main "$@"
