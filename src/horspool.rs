use std::iter::range_step;

pub struct Searcher {
    position: uint,
    occ: [uint, ..256], // right-most occurrence table
}

impl Searcher {
    pub fn new(needle: &[u8]) -> Searcher {
        let m = needle.len();
        let mut occ: [uint, ..256] = [m, ..256];

        for i in range(0, m - 1) {
            // we're storing *how much we skip by* here
            // if the rightmost occurrence of the end character
            // is at m-2, we skip by 1. if it's at m-3, we skip by 2.
            // ... if it's at 0, we skip by m-1. finally, if its not found,
            // we skip by m
            occ[needle[i] as uint] = m - 1 - i;
        }

        Searcher { position: 0, occ: occ }
    }
}

impl super::Searcher for Searcher {
    fn next(&mut self, haystack: &[u8], needle: &[u8]) 
    -> Option<(uint, uint)> {
        let m = needle.len();
        let n = haystack.len();
        'outer: while self.position + m <= n {
            for j in range(0, m).rev() {
                if haystack[self.position + j] != needle[j] {
                    self.position += self.occ[haystack[self.position + m - 1] as uint];
                    continue 'outer;
                }
            }

            // only way we end up here is if we found a match
            let match_pos = self.position;
            self.position += m; // add 1 for all matches
            return Some((match_pos, match_pos + m));
        }
        None
    }
}
