#!/usr/bin/env bash
set -euo pipefail

ROOT="$(dirname "$(dirname "$0")")"

DAY="${1:-}"
YEAR="${2:-2020}"

if [[ $# -ne 1 && $# -ne 2 ]]; then
  echo >&2 "USAGE: $0 <day> <year=2020>"
  exit 127
fi

printf -v DAY_PADDED "%02d" "${DAY}"
LANGUAGE=rust

if [[ $((DAY % 2)) -eq 1 ]]; then
  LANGUAGE=go
fi

sed -e "s,YEAR,${YEAR},g" -e "s,DAY,${DAY_PADDED},g" "${ROOT}/scripts/setup-${LANGUAGE}.patch" | patch --strip=1 --directory="${ROOT}"
