pub struct Searcher {
    position: uint
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher { position: 0 }
    }
}

impl super::Searcher for Searcher {
    fn next(&mut self, haystack: &[u8], needle: &[u8])
    -> Option<(uint, uint)> {
        let m = needle.len();
        let n = haystack.len();
        'outer: while self.position + m <= n {
            for j in range(0, m) {
                if haystack[self.position + j] != needle[j] {
                    self.position += 1;
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
