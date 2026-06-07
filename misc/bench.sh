#!/usr/bin/env bash
set -euo pipefail

BINARY="./target/release/benchmark"
RUNS="${1:-20}"

if [[ ! -x "$BINARY" ]]; then
    echo "Error: $BINARY not found or not executable" >&2
    echo "Build it first with: cargo build --release --bin benchmark" >&2
    exit 1
fi

echo "Running $BINARY $RUNS times..."
echo

times=()
for ((i = 1; i <= RUNS; i++)); do
    output=$("$BINARY" 2>&1)
    ms=$(echo "$output" | grep -oP 'Time per frame \K[0-9.]+(?=ms)' | tail -n1)
    if [[ -z "$ms" ]]; then
        echo "Run $i: failed to parse output" >&2
        echo "$output" >&2
        exit 1
    fi
    printf "Run %2d: %sms\n" "$i" "$ms"
    times+=("$ms")
done

echo
printf '%s\n' "${times[@]}" | awk '
    NR == 1 { min = max = sum = $1; next }
    { sum += $1; if ($1 < min) min = $1; if ($1 > max) max = $1 }
    END {
        printf "Runs: %d\n", NR
        printf "Min:  %.3fms\n", min
        printf "Max:  %.3fms\n", max
        printf "Avg:  %.3fms\n", sum / NR
    }
'
