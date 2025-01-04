#[inline]
pub fn memchr_dbl_iter(haystack: &[u8], needle1: u8, needle2: u8) -> MemchrDblIter {
    MemchrDblIter::new(haystack, needle1, needle2)
}

pub struct MemchrDblIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl MemchrDblIter<'_> {
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> MemchrDblIter {
        MemchrDblIter {
            needle1,
            needle2,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for MemchrDblIter<'_> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match crate::memchr_dbl(&self.haystack[self.position..], self.needle1, self.needle2) {
            Some(idx) => {
                let found = self.position + idx;
                self.position = self.position + idx + 1;
                Some(found)
            }
            None => {
                self.position = self.haystack.len() + 1;
                None
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}

impl DoubleEndedIterator for MemchrDblIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match crate::memrchr_dbl(
            &self.haystack[..(self.position - 1)],
            self.needle1,
            self.needle2,
        ) {
            Some(idx) => {
                self.position = idx + 1;
                Some(idx)
            }
            None => {
                self.position = 0;
                None
            }
        }
    }
}
