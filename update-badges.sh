#! /usr/bin/env bash
# this script updates files in img/badges

set -euo pipefail

match=${1:-'*'}

readarray -t SOURCES <img/badges/sources.txt

for src in "${SOURCES[@]}"; do
  if [[ "$src" =~ ^#.* ]]  || [[ -z "$src" ]]; then
    echo "skipping $src"
    continue
  fi

  filename="${src%%:*}"
  url="${src#*: }"

  if [[ ! $filename == "$match" && ! $match == '*' ]]; then
    echo "skipping $filename"
    continue
  fi

  echo "updating $filename from $url"
  curl -fsL "$url" -o "img/badges/$filename" || echo "failed to download $url"
done

echo "badges updated"
