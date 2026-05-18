#!/usr/bin/env bash
set -euo pipefail

TOOLS=(cargo-deny cargo-audit cargo-cyclonedx release-plz cargo-machete)
DRY_RUN=0
CARGO_CMD=

usage() {
  cat <<'EOF'
Usage: scripts/bootstrap-dev-tools.sh [--dry-run]

Installs the optional Rust workspace tooling used by local validation,
release automation, and advisory checks.
EOF
}

run_cmd() {
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '+'
    for arg in "$@"; do
      printf ' %q' "$arg"
    done
    printf '\n'
    return 0
  fi

  "$@"
}

for arg in "$@"; do
  case "$arg" in
    --dry-run)
      DRY_RUN=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'Unknown argument: %s\n\n' "$arg" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if command -v cargo >/dev/null 2>&1; then
  CARGO_CMD=cargo
elif command -v cargo.exe >/dev/null 2>&1; then
  CARGO_CMD=cargo.exe
else
  printf 'cargo is required to bootstrap dev tools.\n' >&2
  exit 1
fi

for tool in "${TOOLS[@]}"; do
  run_cmd "$CARGO_CMD" install --locked "$tool"
done
