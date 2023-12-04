set -euo pipefail
YEAR=2023
SESSION=$(cat .session)
if [[ -z "$SESSION" ]]; then
    echo "Please set your session cookie in .session"
    exit 1
fi

if [[ -z "$1" ]]; then
    echo "Please provide a day number"
    exit 1
fi

if [[ ! -f "src/bin/day$1.txt" || ! -s "src/bin/day$1.txt" ]]; then
    curl --header "Cookie: session=$SESSION" "https://adventofcode.com/$YEAR/day/$1/input" >"src/bin/day$1.txt"
fi

cargo run --bin day$1
