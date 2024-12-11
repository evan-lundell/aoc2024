#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY=$1
DAY_MODULE="day$(printf "%02d" $DAY)"
DAY_FILE="src/days/$DAY_MODULE.rs"
INPUT_FILE="input/day${DAY}.txt"
DAYS_DIR="src/days"

# Create the new day module file
cat <<EOL > $DAY_FILE
use crate::days::Day;

pub struct Day$DAY;

impl Day for Day$DAY {
    fn part1(contents: &str) -> u64 {
        // Your part1 implementation
        0
    }

    fn part2(contents: &str) -> u64 {
        // Your part2 implementation
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "test input for part1";
        assert_eq!(Day$DAY::part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "test input for part2";
        assert_eq!(Day$DAY::part2(input), 0);
    }
}
EOL

echo "Created $DAY_FILE with function signatures and unit tests"

# Create a blank input file
if [ ! -e "$INPUT_FILE" ]; then
    touch "$INPUT_FILE"
    echo "Created blank input file $INPUT_FILE"
fi

# Update mod.rs
MOD_FILE="$DAYS_DIR/mod.rs"
echo "// Automatically generated mod.rs" > "$MOD_FILE"
echo "pub trait Day {" >> "$MOD_FILE"
echo "    fn part1(contents: &str) -> u64;" >> "$MOD_FILE"
echo "    fn part2(contents: &str) -> u64;" >> "$MOD_FILE"
echo "}" >> "$MOD_FILE"
for file in "$DAYS_DIR"/day*.rs; do
    MODULE_NAME=$(basename "$file" .rs)
    echo "pub mod $MODULE_NAME;" >> "$MOD_FILE"
done

echo "Updated $MOD_FILE"

# Update the match statement in main.rs to include the new day module
sed -i '' "/\/\/ Add more matches here/i\\
        \"$DAY\" => run_day(&contents, $DAY_MODULE::Day$DAY::part1, $DAY_MODULE::Day$DAY::part2),
" src/main.rs

echo "Updated main.rs to include $DAY_MODULE"