// from core::str

pub struct Searcher {
    position: uint
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher { position: 0 }
    }

    pub fn next(&mut self, haystack: &[u8], needle: &[u8]) 
    -> Option<(uint, uint)> {
        while self.position + needle.len() <= haystack.len() {
            if haystack.slice(self.position, self.position + needle.len()) == needle {
                let match_pos = self.position;
                self.position += needle.len(); // add 1 for all matches
                return Some((match_pos, match_pos + needle.len()));
            } else {
                self.position += 1;
            }
        }
        None
    }
}
