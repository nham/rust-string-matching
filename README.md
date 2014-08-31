As of this writing, `"1234567ah012345678901ah".contains("hah")` returns `true` in Rust, which is quite obviously incorrect. Previously there have been other problems (`"bananas".contains("nana")` used to return false). The problem is with Rust's implementation of the Two-Way algorithm, which is clearly a bit tricky to get right.

Therefore it might make the most sense to switch to an easier-to-implement algorithm. Perhaps it would not be as fast, but at least we'd be more certain that it was correct.

This repo is for implementations and benchmarking of various string matching algorithms to try to determine which algorithm should be used.



