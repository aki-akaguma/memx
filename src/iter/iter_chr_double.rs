#[inline]
pub fn memchr_double_iter(haystack: &[u8], needle1: u8, needle2: u8) -> MemchrDoubleIter {
    MemchrDoubleIter::new(haystack, needle1, needle2)
}

pub struct MemchrDoubleIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> MemchrDoubleIter<'a> {
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> MemchrDoubleIter {
        MemchrDoubleIter {
            needle1,
            needle2,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for MemchrDoubleIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match crate::memchr_double(&self.haystack[self.position..], self.needle1, self.needle2) {
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

impl<'a> DoubleEndedIterator for MemchrDoubleIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match crate::memrchr_double(&self.haystack[..(self.position - 1)], self.needle1, self.needle2) {
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
