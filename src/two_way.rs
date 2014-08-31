// from core::str
use std::uint;
use std::cmp;

pub struct Searcher {
    // constants
    crit_pos: uint,
    period: uint,
    byteset: u64,

    // variables
    position: uint,
    memory: uint
}

// This is the Two-Way search algorithm, which was introduced in the paper:
// Crochemore, M., Perrin, D., 1991, Two-way string-matching, Journal of the ACM 38(3):651-675.
impl Searcher {
    pub fn new(needle: &[u8]) -> Searcher {
        let (crit_pos1, period1) = Searcher::maximal_suffix(needle, false);
        let (crit_pos2, period2) = Searcher::maximal_suffix(needle, true);

        let crit_pos;
        let period;
        if crit_pos1 > crit_pos2 {
            crit_pos = crit_pos1;
            period = period1;
        } else {
            crit_pos = crit_pos2;
            period = period2;
        }

        let byteset = needle.iter()
                            .fold(0, |a, &b| (1 << ((b & 0x3f) as uint)) | a);

        // The logic here (calculating crit_pos and period, the final if statement to see which
        // period to use for the TwoWaySearcher) is essentially an implementation of the
        // "small-period" function from the paper (p. 670)
        //
        // In the paper they check whether `needle.slice_to(crit_pos)` is a suffix of
        // `needle.slice(crit_pos, crit_pos + period)`, which is precisely what this does
        if needle.slice_to(crit_pos) == needle.slice(period, period + crit_pos) {
            Searcher {
                crit_pos: crit_pos,
                period: period,
                byteset: byteset,

                position: 0,
                memory: 0
            }
        } else {
            Searcher {
                crit_pos: crit_pos,
                period: cmp::max(crit_pos, needle.len() - crit_pos) + 1,
                byteset: byteset,

                position: 0,
                memory: uint::MAX // Dummy value to signify that the period is long
            }
        }
    }

    // returns (i, p) where i is the "critical position", the starting index of
    // of maximal suffix, and p is the period of the suffix
    // see p. 668 of the paper
    #[inline]
    fn maximal_suffix(arr: &[u8], reversed: bool) -> (uint, uint) {
        let mut left = -1; // Corresponds to i in the paper
        let mut right = 0; // Corresponds to j in the paper
        let mut offset = 1; // Corresponds to k in the paper
        let mut period = 1; // Corresponds to p in the paper

        while right + offset < arr.len() {
            let a;
            let b;
            if reversed {
                a = arr[left + offset];
                b = arr[right + offset];
            } else {
                a = arr[right + offset];
                b = arr[left + offset];
            }
            if a < b {
                // Suffix is smaller, period is entire prefix so far.
                right += offset;
                offset = 1;
                period = right - left;
            } else if a == b {
                // Advance through repetition of the current period.
                if offset == period {
                    right += offset;
                    offset = 1;
                } else {
                    offset += 1;
                }
            } else {
                // Suffix is larger, start over from current location.
                left = right;
                right += 1;
                offset = 1;
                period = 1;
            }
        }
        (left + 1, period)
    }
}

impl super::Searcher for Searcher {
    #[inline]
    fn next(&mut self, haystack: &[u8], needle: &[u8])
    -> Option<(uint, uint)> {
        let long_period = self.memory == uint::MAX;

        'search: loop {
            // Check that we have room to search in
            if self.position + needle.len() > haystack.len() {
                return None;
            }

            // Quickly skip by large portions unrelated to our substring
            if (self.byteset >>
                    ((haystack[self.position + needle.len() - 1] & 0x3f)
                     as uint)) & 1 == 0 {
                self.position += needle.len();
                continue 'search;
            }

            // See if the right part of the needle matches
            let start = if long_period { self.crit_pos }
                        else { cmp::max(self.crit_pos, self.memory) };
            for i in range(start, needle.len()) {
                if needle[i] != haystack[self.position + i] {
                    self.position += i - self.crit_pos + 1;
                    if !long_period {
                        self.memory = 0;
                    }
                    continue 'search;
                }
            }

            // See if the left part of the needle matches
            let start = if long_period { 0 } else { self.memory };
            for i in range(start, self.crit_pos).rev() {
                if needle[i] != haystack[self.position + i] {
                    self.position += self.period;
                    if !long_period {
                        self.memory = needle.len() - self.period;
                    }
                    continue 'search;
                }
            }

            // We have found a match!
            let match_pos = self.position;
            self.position += needle.len(); // add self.period for all matches
            if !long_period {
                self.memory = 0; // set to needle.len() - self.period for all matches
            }
            return Some((match_pos, match_pos + needle.len()));
        }
    }

}
