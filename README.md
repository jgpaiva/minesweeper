# trapping rainwater problem solver

Currently this solves the trapping rainwater problem

## problem

Given n non-negative integers representing an elevation map where the width of
each bar is 1, compute how much water it is able to trap after raining.
Example:
Input: [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6

(more details [here](https://leetcode.com/problems/trapping-rain-water/))

## solution

This solution takes a naive visual approach to this problem, by first drawing
the map and then iterating over the lines of the map to determine which ones can
trap water.

In order to quicky (and visually) determine correctness, the solution has a
random map generator built-in. The solution is built around the following
infinite loop:

1. generates a map
2. calculates the water it traps
3. prints the map and result
4. waits for new line
5. back to 1.

This makes for a fun interaction and allows correctness to be checked visually.

## how to run

After installing rust and cargo, run `cargo run`. To stop it, press ^C.

## example output

![demo output](imgs/demo.png)

