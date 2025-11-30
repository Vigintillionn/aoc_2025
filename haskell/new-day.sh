#!/bin/bash

if [ -z "$1" ]; then
  echo "Please provide a day number (e.g., 01)"
  exit 1
fi

DAY=$1
HS_FILE="src/Day$DAY.hs"
INPUT_FILE="inputs/day$DAY.txt"

mkdir -p src inputs

if [ -f "$HS_FILE" ]; then
  echo "Error: $HS_FILE already exists."
  exit 1
fi

touch $INPUT_FILE
echo "Created $INPUT_FILE"

cat <<EOF > $HS_FILE
module Day$DAY where

import Data.List
import Data.Char
import System.IO

-- | Parse the input
parseInput :: String -> [String]
parseInput = lines

-- | Part 1 Logic
part1 :: [String] -> Int
part1 input = 0

-- | Part 2 Logic
part2 :: [String] -> Int
part2 input = 0

main :: IO ()
main = do
    contents <- readFile "inputs/day$DAY.txt"
    let input = parseInput contents

    putStrLn "--- Part 1 ---"
    print $ part1 input

    putStrLn "--- Part 2 ---"
    print $ part2 input
EOF

echo "Created $HS_FILE"
