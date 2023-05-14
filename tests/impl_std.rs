pub fn _std_memchr(buf: &[u8], by1: u8) -> Option<usize> {
    buf.iter().position(|&x| x == by1)
}

pub fn _std_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x == by1)
}

pub fn _std_memnechr(buf: &[u8], by1: u8) -> Option<usize> {
    buf.iter().position(|&x| x != by1)
}

pub fn _std_memrnechr(buf: &[u8], by1: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x != by1)
}

pub fn _std_memcmp(buf: &[u8], pat_bytes: &[u8]) -> std::cmp::Ordering {
    buf.cmp(pat_bytes)
}

pub fn _std_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
    buf == pat_bytes
}

pub fn _std_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
    if dst.len() < src.len() {
        return Err(memx::RangeError);
    }
    let _ = &(dst[0..src.len()]).copy_from_slice(src);
    Ok(())
}

#[rustversion::since(1.50)]
pub fn _std_memset(dst: &mut [u8], by1: u8) {
    dst.fill(by1);
}

#[rustversion::before(1.50)]
pub fn _std_memset(dst: &mut [u8], by1: u8) {
    for i in 0..dst.len() {
        dst[i] = by1;
    }
}

pub fn _std_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
    if buf.len() < pat_bytes.len() {
        return None;
    }
    (0..=(buf.len() - pat_bytes.len())).find(|&i| &buf[i..(i + pat_bytes.len())] == pat_bytes)
}

pub fn _std_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
    let buf_len = buf.len();
    let pat_len = pat_bytes.len();
    if buf_len < pat_len {
        return None;
    }
    let max_i = buf_len - pat_len;
    for i in 0..=max_i {
        let j = max_i - i;
        if &buf[j..(j + pat_len)] == pat_bytes {
            return Some(j);
        }
    }
    None
}

pub fn _std_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    buf.iter().position(|&x| x == by1 || x == by2)
}

pub fn _std_memrchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x == by1 || x == by2)
}

pub fn _std_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    buf.iter().position(|&x| x == by1 || x == by2 || x == by3)
}

pub fn _std_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x == by1 || x == by2 || x == by3)
}

pub fn _std_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    buf.iter()
        .position(|&x| x == by1 || x == by2 || x == by3 || x == by4)
}

pub fn _std_memrchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    buf.iter()
        .rposition(|&x| x == by1 || x == by2 || x == by3 || x == by4)
}

pub fn _std_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    buf.iter().position(|&x| x != by1 && x != by2)
}

pub fn _std_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x != by1 && x != by2)
}

pub fn _std_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    buf.iter().position(|&x| x != by1 && x != by2 && x != by3)
}

pub fn _std_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x != by1 && x != by2 && x != by3)
}

pub fn _std_memnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    buf.iter()
        .position(|&x| x != by1 && x != by2 && x != by3 && x != by4)
}

pub fn _std_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    buf.iter()
        .rposition(|&x| x != by1 && x != by2 && x != by3 && x != by4)
}

pub fn _std_memchr_iter(buf: &[u8], by1: u8) -> StdMemchrSglIter {
    StdMemchrSglIter::new(buf, by1)
}

pub fn _std_memrchr_iter(buf: &[u8], by1: u8) -> StdMemrchrSglIter {
    StdMemrchrSglIter::new(buf, by1)
}

pub fn _std_memchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> StdMemchrDblIter {
    StdMemchrDblIter::new(buf, by1, by2)
}

pub fn _std_memrchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> StdMemrchrDblIter {
    StdMemrchrDblIter::new(buf, by1, by2)
}

pub fn _std_memchr_tpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8) -> StdMemchrTplIter {
    StdMemchrTplIter::new(buf, by1, by2, by3)
}

pub fn _std_memrchr_tpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8) -> StdMemrchrTplIter {
    StdMemrchrTplIter::new(buf, by1, by2, by3)
}

pub fn _std_memchr_qpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> StdMemchrQplIter {
    StdMemchrQplIter::new(buf, by1, by2, by3, by4)
}

pub fn _std_memrchr_qpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> StdMemrchrQplIter {
    StdMemrchrQplIter::new(buf, by1, by2, by3, by4)
}

pub fn _std_memmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> StdMemmemIter<'a> {
    StdMemmemIter::new(buf, pat)
}

pub fn _std_memrmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> StdMemrmemIter<'a> {
    StdMemrmemIter::new(buf, pat)
}

pub struct StdMemchrSglIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemchrSglIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8) -> StdMemchrSglIter {
        StdMemchrSglIter {
            needle1,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for StdMemchrSglIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr(&self.haystack[self.position..], self.needle1) {
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
impl<'a> DoubleEndedIterator for StdMemchrSglIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr(&self.haystack[..(self.position - 1)], self.needle1) {
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

pub struct StdMemrchrSglIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemrchrSglIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8) -> StdMemrchrSglIter {
        StdMemrchrSglIter {
            needle1,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for StdMemrchrSglIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr(&self.haystack[..(self.position - 1)], self.needle1) {
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
impl<'a> DoubleEndedIterator for StdMemrchrSglIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr(&self.haystack[self.position..], self.needle1) {
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

pub struct StdMemchrDblIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemchrDblIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> StdMemchrDblIter {
        StdMemchrDblIter {
            needle1,
            needle2,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for StdMemchrDblIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_dbl(&self.haystack[self.position..], self.needle1, self.needle2) {
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
impl<'a> DoubleEndedIterator for StdMemchrDblIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_dbl(
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

pub struct StdMemrchrDblIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemrchrDblIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> StdMemrchrDblIter {
        StdMemrchrDblIter {
            needle1,
            needle2,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for StdMemrchrDblIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_dbl(
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
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}
impl<'a> DoubleEndedIterator for StdMemrchrDblIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_dbl(&self.haystack[self.position..], self.needle1, self.needle2) {
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

pub struct StdMemchrTplIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemchrTplIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> StdMemchrTplIter {
        StdMemchrTplIter {
            needle1,
            needle2,
            needle3,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for StdMemchrTplIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_tpl(
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
impl<'a> DoubleEndedIterator for StdMemchrTplIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_tpl(
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

pub struct StdMemrchrTplIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemrchrTplIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> StdMemrchrTplIter {
        StdMemrchrTplIter {
            needle1,
            needle2,
            needle3,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for StdMemrchrTplIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_tpl(
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
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}
impl<'a> DoubleEndedIterator for StdMemrchrTplIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_tpl(
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
}

pub struct StdMemchrQplIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
    needle4: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemchrQplIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(
        haystack: &[u8],
        needle1: u8,
        needle2: u8,
        needle3: u8,
        needle4: u8,
    ) -> StdMemchrQplIter {
        StdMemchrQplIter {
            needle1,
            needle2,
            needle3,
            needle4,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for StdMemchrQplIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_qpl(
            &self.haystack[self.position..],
            self.needle1,
            self.needle2,
            self.needle3,
            self.needle4,
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
impl<'a> DoubleEndedIterator for StdMemchrQplIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_qpl(
            &self.haystack[..(self.position - 1)],
            self.needle1,
            self.needle2,
            self.needle3,
            self.needle4,
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

pub struct StdMemrchrQplIter<'a> {
    haystack: &'a [u8],
    needle1: u8,
    needle2: u8,
    needle3: u8,
    needle4: u8,
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemrchrQplIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(
        haystack: &[u8],
        needle1: u8,
        needle2: u8,
        needle3: u8,
        needle4: u8,
    ) -> StdMemrchrQplIter {
        StdMemrchrQplIter {
            needle1,
            needle2,
            needle3,
            needle4,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for StdMemrchrQplIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match _std_memrchr_qpl(
            &self.haystack[..(self.position - 1)],
            self.needle1,
            self.needle2,
            self.needle3,
            self.needle4,
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
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}
impl<'a> DoubleEndedIterator for StdMemrchrQplIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memchr_qpl(
            &self.haystack[self.position..],
            self.needle1,
            self.needle2,
            self.needle3,
            self.needle4,
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
}

pub struct StdMemmemIter<'a> {
    haystack: &'a [u8],
    needle: &'a [u8],
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemmemIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &'a [u8], needle: &'a [u8]) -> StdMemmemIter<'a> {
        StdMemmemIter {
            needle,
            haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for StdMemmemIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memmem(&self.haystack[self.position..], self.needle) {
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
impl<'a> DoubleEndedIterator for StdMemmemIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position == 0 {
            return None;
        }
        match _std_memrmem(&self.haystack[..(self.position - 1)], self.needle) {
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

pub struct StdMemrmemIter<'a> {
    haystack: &'a [u8],
    needle: &'a [u8],
    position: usize, // 0: idx is -1, 1: idx is 0, 2: idx is 1
}
impl<'a> StdMemrmemIter<'a> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &'a [u8], needle: &'a [u8]) -> StdMemrmemIter<'a> {
        StdMemrmemIter {
            needle,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl<'a> Iterator for StdMemrmemIter<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.position == 0 {
            return None;
        }
        match _std_memrmem(&self.haystack[..(self.position - 1)], self.needle) {
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
impl<'a> DoubleEndedIterator for StdMemrmemIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.position > self.haystack.len() {
            return None;
        }
        match _std_memmem(&self.haystack[self.position..], self.needle) {
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
