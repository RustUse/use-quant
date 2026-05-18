#!/usr/bin/env sh
set -eu

# WARNING: `git push --mirror` updates every ref on the destination remote and
# can delete remote refs. Review each configured remote before using
# `--execute`.

if [ "${1-}" != "--execute" ]; then
  cat <<'EOF'
This script is intentionally dry-run by default.

Expected remote names:
  github
  gitlab
  codeberg
  sourcehut

Commands that will run with --execute:
  git fetch github --prune --tags
  git push --mirror gitlab
  git push --mirror codeberg
  git push --mirror sourcehut

Review configured remotes first:
  git remote -v
EOF
  exit 0
fi

git remote get-url github >/dev/null

git fetch github --prune --tags

for remote in gitlab codeberg sourcehut; do
  if git remote get-url "$remote" >/dev/null 2>&1; then
    git push --mirror "$remote"
  else
    printf 'Skipping %s: remote is not configured.\n' "$remote"
  fi
done
