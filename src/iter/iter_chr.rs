#[inline]
pub fn memchr_iter(haystack: &[u8], needle: u8) -> MemchrIter {
    MemchrIter::new(haystack, needle)
}

pub struct MemchrIter<'a> {
    haystack: &'a [u8],
    needle: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl MemchrIter<'_> {
    #[inline]
    pub fn new(haystack: &[u8], needle: u8) -> MemchrIter {
        MemchrIter {
            needle,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for MemchrIter<'_> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match crate::memchr(&self.haystack[self.position..], self.needle) {
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

impl DoubleEndedIterator for MemchrIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match crate::memrchr(&self.haystack[..(self.position - 1)], self.needle) {
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
