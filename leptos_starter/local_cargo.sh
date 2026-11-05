#!/usr/bin/env bash
set -e

ROOT="$(cd "$(dirname "$0")" && pwd)"

# `c r` (and `c r r`) route through trunk serve: this is a Leptos app, not a
# native binary. Invoking `cargo run` directly still runs the real binary.
if [ "$1" = "run" ]; then
    shift
    args=()
    for a in "$@"; do
        if [ "$a" != "--release" ]; then
            args+=("$a")
        fi
    done
    cd "$ROOT"
    exec trunk serve "${args[@]}"
fi

exec cargo "$@"
