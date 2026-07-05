#!/usr/bin/env bash
# Reproducible benchmark for gitb vs gita vs myrepos (mr).
# Creates N temp git repos, times `status` and `pull` on each tool.
# Usage: ./bench/run.sh [repo_count]  (default 50)
set -euo pipefail

REPO_COUNT="${1:-50}"
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT

# Locate the gitb binary: prefer local release build, fall back to PATH
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
if [ -x "$SCRIPT_DIR/target/release/gitb" ]; then
  GITB="$SCRIPT_DIR/target/release/gitb"
elif command -v gitb >/dev/null 2>&1; then
  GITB="gitb"
else
  echo "ERROR: gitb not found. Build it with: cargo build --release" >&2
  exit 1
fi

echo "=== gitb benchmark: $REPO_COUNT repos ==="
echo "workspace: $WORKDIR"
echo "gitb: $GITB ($("$GITB" --version 2>&1 || echo 'unknown'))"
echo ""

# Create N repos with a few commits each
echo "setting up $REPO_COUNT repos..."
for i in $(seq 1 "$REPO_COUNT"); do
  mkdir -p "$WORKDIR/repo-$i"
  cd "$WORKDIR/repo-$i"
  git init -q
  git config user.email "bench@local"
  git config user.name "bench"
  git commit -q --allow-empty -m "init repo-$i"
  git commit -q --allow-empty -m "second commit repo-$i"
done
cd "$WORKDIR"

echo ""
echo "--- gitb status ---"
cd "$WORKDIR"
time "$GITB" status -o quiet

echo ""
echo "--- gitb pull -j 8 ---"
cd "$WORKDIR"
time "$GITB" pull -j 8 2>/dev/null || true

if command -v gita >/dev/null 2>&1; then
  echo ""
  echo "--- gita status ---"
  cd "$WORKDIR"
  time gita ll 2>/dev/null || echo "(gita not configured for this workspace)"
else
  echo ""
  echo "(gita not installed — skipping)"
fi

if command -v mr >/dev/null 2>&1; then
  echo ""
  echo "--- mr status ---"
  cd "$WORKDIR"
  time mr status 2>/dev/null || echo "(mr not configured)"
else
  echo "(mr not installed — skipping)"
fi

echo ""
echo "=== done ==="
echo "Note: timing depends on CPU, disk, and network. Your results may vary."
