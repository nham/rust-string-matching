extern crate test;

// naive
// horspool
// quick-search
// tuned BM

mod naive;
mod naive2;
mod naive3;
mod two_way;

pub struct StringMatcher<'a, S> {
    haystack: &'a str,
    needle: &'a str,
    searcher: S,
}

impl<'a, S> StringMatcher<'a, S> {
    fn new(haystack: &'a str, needle: &'a str, searcher: S)
    -> StringMatcher<'a, S> {
        StringMatcher {
            haystack: haystack, 
            needle: needle,
            searcher: searcher,
        }
    }
}

// matches result in (i, j) where i is the first index and j is the index after 
// the last index in the haystack. so if haystack = "abcwoofxyz" and needle = "woof",
// the match indices are (3, 7)
impl<'a> Iterator<(uint, uint)> for StringMatcher<'a, naive::Searcher> {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint)> {
        self.searcher.next(self.haystack.as_bytes(), self.needle.as_bytes())
    }
}

pub fn naive_contains<'a>(haystack: &'a str, needle: &'a str) -> bool {
    if needle.is_empty() {
        true
    } else {
        let mut sm = StringMatcher::new(haystack, needle, naive::Searcher::new());
        sm.next().is_some()
    }
}


impl<'a> Iterator<(uint, uint)> for StringMatcher<'a, naive2::Searcher> {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint)> {
        self.searcher.next(self.haystack.as_bytes(), self.needle.as_bytes())
    }
}

pub fn naive2_contains<'a>(haystack: &'a str, needle: &'a str) -> bool {
    if needle.is_empty() {
        true
    } else {
        let mut sm = StringMatcher::new(haystack, needle, naive2::Searcher::new());
        sm.next().is_some()
    }
}

impl<'a> Iterator<(uint, uint)> for StringMatcher<'a, naive3::Searcher> {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint)> {
        self.searcher.next(self.haystack.as_bytes(), self.needle.as_bytes())
    }
}

pub fn naive3_contains<'a>(haystack: &'a str, needle: &'a str) -> bool {
    if needle.is_empty() {
        true
    } else {
        let mut sm = StringMatcher::new(haystack, needle, naive3::Searcher::new());
        sm.next().is_some()
    }
}

impl<'a> Iterator<(uint, uint)> for StringMatcher<'a, two_way::Searcher> {
    #[inline]
    fn next(&mut self) -> Option<(uint, uint)> {
        self.searcher.next(self.haystack.as_bytes(), self.needle.as_bytes())
    }
}

pub fn two_way_contains<'a>(haystack: &'a str, needle: &'a str) -> bool {
    if needle.is_empty() {
        true
    } else {
        let mut sm = StringMatcher::new(haystack, needle,
                                        two_way::Searcher::new(needle.as_bytes()));
        sm.next().is_some()
    }
}


#[cfg(test)]
mod bench {
    use super::{naive_contains, naive2_contains, naive3_contains, two_way_contains};
    use test::Bencher;
    // following benchmarks were stolen from libcollections/str.rs
    static sh_sh_haystack: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    static sh_sh_needle: &'static str = "sit";

    static sh_lo_haystack: &'static str = "\
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis lorem sit amet dolor \
    ultricies condimentum. Praesent iaculis purus elit, ac malesuada quam malesuada in. Duis sed orci \
    eros. Suspendisse sit amet magna mollis, mollis nunc luctus, imperdiet mi. Integer fringilla non \
    sem ut lacinia. Fusce varius tortor a risus porttitor hendrerit. Morbi mauris dui, ultricies nec \
    tempus vel, gravida nec quam.

    In est dui, tincidunt sed tempus interdum, adipiscing laoreet ante. Etiam tempor, tellus quis \
    sagittis interdum, nulla purus mattis sem, quis auctor erat odio ac tellus. In nec nunc sit amet \
    diam volutpat molestie at sed ipsum. Vestibulum laoreet consequat vulputate. Integer accumsan \
    lorem ac dignissim placerat. Suspendisse convallis faucibus lorem. Aliquam erat volutpat. In vel \
    eleifend felis. Sed suscipit nulla lorem, sed mollis est sollicitudin et. Nam fermentum egestas \
    interdum. Curabitur ut nisi justo.

    Sed sollicitudin ipsum tellus, ut condimentum leo eleifend nec. Cras ut velit ante. Phasellus nec \
    mollis odio. Mauris molestie erat in arcu mattis, at aliquet dolor vehicula. Quisque malesuada \
    lectus sit amet nisi pretium, a condimentum ipsum porta. Morbi at dapibus diam. Praesent egestas \
    est sed risus elementum, eu rutrum metus ultrices. Etiam fermentum consectetur magna, id rutrum \
    felis accumsan a. Aliquam ut pellentesque libero. Sed mi nulla, lobortis eu tortor id, suscipit \
    ultricies neque. Morbi iaculis sit amet risus at iaculis. Praesent eget ligula quis turpis \
    feugiat suscipit vel non arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. \
    Aliquam sit amet placerat lorem.

    Cras a lacus vel ante posuere elementum. Nunc est leo, bibendum ut facilisis vel, bibendum at \
    mauris. Nullam adipiscing diam vel odio ornare, luctus adipiscing mi luctus. Nulla facilisi. \
    Mauris adipiscing bibendum neque, quis adipiscing lectus tempus et. Sed feugiat erat et nisl \
    lobortis pharetra. Donec vitae erat enim. Nullam sit amet felis et quam lacinia tincidunt. Aliquam \
    suscipit dapibus urna. Sed volutpat urna in magna pulvinar volutpat. Phasellus nec tellus ac diam \
    cursus accumsan.

    Nam lectus enim, dapibus non nisi tempor, consectetur convallis massa. Maecenas eleifend dictum \
    feugiat. Etiam quis mauris vel risus luctus mattis a a nunc. Nullam orci quam, imperdiet id \
    vehicula in, porttitor ut nibh. Duis sagittis adipiscing nisl vitae congue. Donec mollis risus eu \
    leo suscipit, varius porttitor nulla porta. Pellentesque ut sem nec nisi euismod vehicula. Nulla \
    malesuada sollicitudin quam eu fermentum.";
    static sh_lo_needle: &'static str = "english";

    static bad_naive_haystack: &'static str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    static bad_naive_needle: &'static str = "aaaaaaaab";

    static equal_haystack: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    static equal_needle: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";


    static all_substrings_haystack: &'static str = "All mimsy were the borogoves.";

    #[bench]
    fn naive_contains_short_short(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive_contains(sh_sh_haystack, sh_sh_needle));
        })
    }

    #[bench]
    fn naive_contains_short_long(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive_contains(sh_lo_haystack, sh_lo_needle));
        })
    }

    #[bench]
    fn naive_contains_bad_naive(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive_contains(bad_naive_haystack, bad_naive_needle));
        })
    }

    #[bench]
    fn naive_contains_equal(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive_contains(equal_haystack, equal_needle));
        })
    }

    #[bench]
    fn naive_contains_all_substrings(b: &mut Bencher) {
        let n = all_substrings_haystack.len();
        b.iter(|| {
            assert!(naive_contains(all_substrings_haystack, ""));
            for i in range(0, n) {
                for j in range(i+1, n + 1) {
                    assert!(naive_contains(all_substrings_haystack,
                                           all_substrings_haystack.slice(i, j)));
                }
            }
        })
    }

    #[bench]
    fn naive2_contains_short_short(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive2_contains(sh_sh_haystack, sh_sh_needle));
        })
    }

    #[bench]
    fn naive2_contains_short_long(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive2_contains(sh_lo_haystack, sh_lo_needle));
        })
    }

    #[bench]
    fn naive2_contains_bad_naive(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive2_contains(bad_naive_haystack, bad_naive_needle));
        })
    }

    #[bench]
    fn naive2_contains_equal(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive2_contains(equal_haystack, equal_needle));
        })
    }

    #[bench]
    fn naive2_contains_all_substrings(b: &mut Bencher) {
        let n = all_substrings_haystack.len();
        b.iter(|| {
            assert!(naive2_contains(all_substrings_haystack, ""));
            for i in range(0, n) {
                for j in range(i+1, n + 1) {
                    assert!(naive2_contains(all_substrings_haystack,
                                           all_substrings_haystack.slice(i, j)));
                }
            }
        })
    }

    #[bench]
    fn naive3_contains_short_short(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive3_contains(sh_sh_haystack, sh_sh_needle));
        })
    }

    #[bench]
    fn naive3_contains_short_long(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive3_contains(sh_lo_haystack, sh_lo_needle));
        })
    }

    #[bench]
    fn naive3_contains_bad_naive(b: &mut Bencher) {
        b.iter(|| {
            assert!(!naive3_contains(bad_naive_haystack, bad_naive_needle));
        })
    }

    #[bench]
    fn naive3_contains_equal(b: &mut Bencher) {
        b.iter(|| {
            assert!(naive3_contains(equal_haystack, equal_needle));
        })
    }

    #[bench]
    fn naive3_contains_all_substrings(b: &mut Bencher) {
        let n = all_substrings_haystack.len();
        b.iter(|| {
            assert!(naive3_contains(all_substrings_haystack, ""));
            for i in range(0, n) {
                for j in range(i+1, n + 1) {
                    assert!(naive3_contains(all_substrings_haystack,
                                           all_substrings_haystack.slice(i, j)));
                }
            }
        })
    }

    #[bench]
    fn two_way_contains_short_short(b: &mut Bencher) {
        b.iter(|| {
            assert!(two_way_contains(sh_sh_haystack, sh_sh_needle));
        })
    }

    #[bench]
    fn two_way_contains_short_long(b: &mut Bencher) {
        b.iter(|| {
            assert!(!two_way_contains(sh_lo_haystack, sh_lo_needle));
        })
    }

    #[bench]
    fn two_way_contains_bad_naive(b: &mut Bencher) {
        b.iter(|| {
            assert!(!two_way_contains(bad_naive_haystack, bad_naive_needle));
        })
    }

    #[bench]
    fn two_way_contains_equal(b: &mut Bencher) {
        b.iter(|| {
            assert!(two_way_contains(equal_haystack, equal_needle));
        })
    }

    #[bench]
    fn two_way_contains_all_substrings(b: &mut Bencher) {
        let n = all_substrings_haystack.len();
        b.iter(|| {
            assert!(two_way_contains(all_substrings_haystack, ""));
            for i in range(0, n) {
                for j in range(i+1, n + 1) {
                    assert!(two_way_contains(all_substrings_haystack,
                                             all_substrings_haystack.slice(i, j)));
                }
            }
        })
    }
}

fn main() {

}
