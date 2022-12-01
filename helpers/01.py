#!/usr/bin/env python

from pathlib import Path

input_path = Path("/home/styty/git/aoc2022/input/2022")

def part1():
    lines = [line.rstrip() for line in open(input_path.joinpath("01.txt")).readlines()]
    print(lines)

if __name__ == "__main__":
    part1()
