#!/bin/bash
# fetch.sh <url-list> <cache-dir>: fetch each URL once (browser UA), cache by md5; log status
UA='Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36'
cache=$2
fetch_one() { u=$1; f="$cache/$(echo -n "$u" | md5sum | cut -c1-32).html"; [ -s "$f" ] && return; code=$(curl -s -L -A "$UA" --max-time 60 -o "$f" -w '%{http_code}' "$u"); echo -e "$u\t$code\t$(date -u +%FT%TZ)" >> "$cache/fetch-log.tsv"; }
export -f fetch_one; export UA cache
sort -u "$1" | xargs -P 4 -I{} bash -c 'fetch_one "$@"' _ {}
