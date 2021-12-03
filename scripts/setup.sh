#!/usr/bin/env bash
set -euo pipefail

ROOT="$(dirname "$(dirname "$0")")"

DAY="${1:-}"
YEAR="${2:-$(date +%Y)}"

if [[ $# -ne 1 && $# -ne 2 ]]; then
  echo >&2 "USAGE: $0 <day> <year=$(date +%Y)>"
  exit 127
fi

printf -v DAY_PADDED "%02d" "${DAY}"

_is_odd_day() {
  [[ $((DAY % 2)) -eq 1 ]]
}

if [[ "${YEAR}" == "2020" ]]; then
  if _is_odd_day; then
    LANGUAGE=go
  else
    LANGUAGE=rust
  fi
elif [[ "${YEAR}" == "2021" ]]; then
  if _is_odd_day; then
    LANGUAGE=nix
  else
    LANGUAGE=python
  fi
fi

sed -e "s,YEAR,${YEAR},g" -e "s,DAY,${DAY_PADDED},g" "${ROOT}/scripts/setup-${LANGUAGE}.patch" | patch --strip=1 --directory="${ROOT}"
