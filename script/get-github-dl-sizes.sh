#!/usr/bin/env bash

set -euo pipefail

usage() {
    printf 'Usage: %s https://github.com/OWNER/REPOSITORY/releases/tag/TAG\n' "${0##*/}" >&2
    exit 2
}

[[ $# -eq 1 ]] || usage

release_url=$1
if [[ $release_url =~ ^https://github\.com/([^/]+)/([^/]+)/releases/tag/([^/?#]+)/*$ ]]; then
    owner=${BASH_REMATCH[1]}
    repository=${BASH_REMATCH[2]}
    tag=${BASH_REMATCH[3]}
else
    printf 'error: expected a GitHub release URL, got: %s\n' "$release_url" >&2
    usage
fi

command -v curl >/dev/null 2>&1 || {
    printf 'error: curl is required\n' >&2
    exit 1
}
command -v jq >/dev/null 2>&1 || {
    printf 'error: jq is required\n' >&2
    exit 1
}

api_url="https://api.github.com/repos/${owner}/${repository}/releases/tags/${tag}?per_page=100"
curl_args=(--fail --silent --show-error --location --header 'Accept: application/vnd.github+json')
if [[ -n ${GITHUB_TOKEN:-} ]]; then
    curl_args+=(--header "Authorization: Bearer ${GITHUB_TOKEN}")
fi

curl "${curl_args[@]}" "$api_url" \
    | jq -er '[.assets[]?.size] | add // 0'
