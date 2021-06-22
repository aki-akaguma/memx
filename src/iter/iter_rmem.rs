#[inline]
pub fn memrmem_iter<'a>(haystack: &'a [u8], needle: &'a [u8]) -> MemrmemIter<'a> {
    MemrmemIter::new(haystack, needle)
}

pub struct MemrmemIter<'a> {
    haystack: &'a [u8],
    needle: &'a [u8],
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> MemrmemIter<'a> {
    #[inline]
    pub fn new(haystack: &'a [u8], needle: &'a [u8]) -> MemrmemIter<'a> {
        MemrmemIter {
            needle,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for MemrmemIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match crate::memrmem(&self.haystack[..(self.position - 1)], self.needle) {
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
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}

impl<'a> DoubleEndedIterator for MemrmemIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match crate::memmem(&self.haystack[self.position..], self.needle) {
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
}
