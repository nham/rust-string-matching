pub struct Searcher {
    position: uint
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher { position: 0 }
    }

    pub fn next(&mut self, haystack: &[u8], needle: &[u8]) 
    -> Option<(uint, uint)> {
        let m = needle.len();
        let n = haystack.len();
        while self.position + m <= n {
            let mut a_match = true;
            for j in range(0, m) {
                if haystack[self.position + j] != needle[j] {
                    a_match = false;
                    break;
                }
            }

            if a_match {
                let match_pos = self.position;
                self.position += m; // add 1 for all matches
                return Some((match_pos, match_pos + m));
            } else {
                self.position += 1;
            }
        }
        None
    }
}

/*

: rustc --test src/main.rs && ./main --bench

running 10 tests
test bench::naive2_contains_all_substrings  ... bench:   1273836 ns/iter (+/- 270569)
test bench::naive2_contains_bad_naive       ... bench:     54441 ns/iter (+/- 11809)
test bench::naive2_contains_equal           ... bench:      5125 ns/iter (+/- 999)
test bench::naive2_contains_short_long      ... bench:    409271 ns/iter (+/- 8510)
test bench::naive2_contains_short_short     ... bench:      3697 ns/iter (+/- 91)
test bench::naive_contains_all_substrings   ... bench:   5137213 ns/iter (+/- 43391)
test bench::naive_contains_bad_naive        ... bench:    143527 ns/iter (+/- 2369)
test bench::naive_contains_equal            ... bench:      9358 ns/iter (+/- 207)
test bench::naive_contains_short_long       ... bench:   2377681 ns/iter (+/- 17070)
test bench::naive_contains_short_short      ... bench:     18391 ns/iter (+/- 155)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured



: rustc --opt-level=1 --test src/main.rs && ./main --bench

running 10 tests
test bench::naive2_contains_all_substrings  ... bench:    575743 ns/iter (+/- 12255)
test bench::naive2_contains_bad_naive       ... bench:     19827 ns/iter (+/- 3291)
test bench::naive2_contains_equal           ... bench:      1904 ns/iter (+/- 314)
test bench::naive2_contains_short_long      ... bench:    138079 ns/iter (+/- 21741)
test bench::naive2_contains_short_short     ... bench:      1375 ns/iter (+/- 34)
test bench::naive_contains_all_substrings   ... bench:   2075218 ns/iter (+/- 9457)
test bench::naive_contains_bad_naive        ... bench:     50433 ns/iter (+/- 322)
test bench::naive_contains_equal            ... bench:      3057 ns/iter (+/- 39)
test bench::naive_contains_short_long       ... bench:    975396 ns/iter (+/- 3685)
test bench::naive_contains_short_short      ... bench:      7549 ns/iter (+/- 24)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured



: rustc --opt-level=2 --test src/main.rs && ./main --bench

running 10 tests
test bench::naive2_contains_all_substrings  ... bench:    112206 ns/iter (+/- 1959)
test bench::naive2_contains_bad_naive       ... bench:      2939 ns/iter (+/- 751)
test bench::naive2_contains_equal           ... bench:         5 ns/iter (+/- 2)
test bench::naive2_contains_short_long      ... bench:     34288 ns/iter (+/- 9102)
test bench::naive2_contains_short_short     ... bench:       322 ns/iter (+/- 47)
test bench::naive_contains_all_substrings   ... bench:    175811 ns/iter (+/- 3029)
test bench::naive_contains_bad_naive        ... bench:      6967 ns/iter (+/- 81)
test bench::naive_contains_equal            ... bench:       664 ns/iter (+/- 14)
test bench::naive_contains_short_long       ... bench:     58032 ns/iter (+/- 851)
test bench::naive_contains_short_short      ... bench:       529 ns/iter (+/- 18)

test result: ok. 0 passed; 0 failed; 0 ignored; 10 measured

*/
