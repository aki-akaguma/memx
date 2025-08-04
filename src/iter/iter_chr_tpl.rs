#[inline]
pub fn memchr_tpl_iter(
    haystack: &[u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
) -> MemchrTplIter<'_> {
    MemchrTplIter::new(haystack, needle1, needle2, needle3)
}

pub struct MemchrTplIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl MemchrTplIter<'_> {
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> MemchrTplIter<'_> {
        MemchrTplIter {
            needle1,
            needle2,
            needle3,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for MemchrTplIter<'_> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match crate::memchr_tpl(
            &self.haystack[self.position..],
            self.needle1,
            self.needle2,
            self.needle3,
        ) {
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

impl DoubleEndedIterator for MemchrTplIter<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match crate::memrchr_tpl(
            &self.haystack[..(self.position - 1)],
            self.needle1,
            self.needle2,
            self.needle3,
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
