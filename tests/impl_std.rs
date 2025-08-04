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

pub fn _std_memchr_iter(buf: &[u8], by1: u8) -> StdMemchrSglIter<'_> {
    StdMemchrSglIter::new(buf, by1)
}

pub fn _std_memrchr_iter(buf: &[u8], by1: u8) -> StdMemrchrSglIter<'_> {
    StdMemrchrSglIter::new(buf, by1)
}

pub fn _std_memchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> StdMemchrDblIter<'_> {
    StdMemchrDblIter::new(buf, by1, by2)
}

pub fn _std_memrchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> StdMemrchrDblIter<'_> {
    StdMemrchrDblIter::new(buf, by1, by2)
}

pub fn _std_memchr_tpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8) -> StdMemchrTplIter<'_> {
    StdMemchrTplIter::new(buf, by1, by2, by3)
}

pub fn _std_memrchr_tpl_iter(buf: &[u8], by1: u8, by2: u8, by3: u8) -> StdMemrchrTplIter<'_> {
    StdMemrchrTplIter::new(buf, by1, by2, by3)
}

pub fn _std_memchr_qpl_iter(
    buf: &[u8],
    by1: u8,
    by2: u8,
    by3: u8,
    by4: u8,
) -> StdMemchrQplIter<'_> {
    StdMemchrQplIter::new(buf, by1, by2, by3, by4)
}

pub fn _std_memrchr_qpl_iter(
    buf: &[u8],
    by1: u8,
    by2: u8,
    by3: u8,
    by4: u8,
) -> StdMemrchrQplIter<'_> {
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
impl StdMemchrSglIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8) -> StdMemchrSglIter<'_> {
        StdMemchrSglIter {
            needle1,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for StdMemchrSglIter<'_> {
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
impl DoubleEndedIterator for StdMemchrSglIter<'_> {
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
impl StdMemrchrSglIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8) -> StdMemrchrSglIter<'_> {
        StdMemrchrSglIter {
            needle1,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl Iterator for StdMemrchrSglIter<'_> {
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
impl DoubleEndedIterator for StdMemrchrSglIter<'_> {
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
impl StdMemchrDblIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> StdMemchrDblIter<'_> {
        StdMemchrDblIter {
            needle1,
            needle2,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for StdMemchrDblIter<'_> {
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
impl DoubleEndedIterator for StdMemchrDblIter<'_> {
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
impl StdMemrchrDblIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8) -> StdMemrchrDblIter<'_> {
        StdMemrchrDblIter {
            needle1,
            needle2,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl Iterator for StdMemrchrDblIter<'_> {
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
impl DoubleEndedIterator for StdMemrchrDblIter<'_> {
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
impl StdMemchrTplIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> StdMemchrTplIter<'_> {
        StdMemchrTplIter {
            needle1,
            needle2,
            needle3,
            haystack,
            position: 0,
        }
    }
}
impl Iterator for StdMemchrTplIter<'_> {
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
impl DoubleEndedIterator for StdMemchrTplIter<'_> {
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
impl StdMemrchrTplIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(haystack: &[u8], needle1: u8, needle2: u8, needle3: u8) -> StdMemrchrTplIter<'_> {
        StdMemrchrTplIter {
            needle1,
            needle2,
            needle3,
            haystack,
            position: haystack.len() + 1,
        }
    }
}
impl Iterator for StdMemrchrTplIter<'_> {
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
impl DoubleEndedIterator for StdMemrchrTplIter<'_> {
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
impl StdMemchrQplIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(
        haystack: &[u8],
        needle1: u8,
        needle2: u8,
        needle3: u8,
        needle4: u8,
    ) -> StdMemchrQplIter<'_> {
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
impl Iterator for StdMemchrQplIter<'_> {
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
impl DoubleEndedIterator for StdMemchrQplIter<'_> {
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
impl StdMemrchrQplIter<'_> {
    #[allow(dead_code)]
    #[inline]
    pub fn new(
        haystack: &[u8],
        needle1: u8,
        needle2: u8,
        needle3: u8,
        needle4: u8,
    ) -> StdMemrchrQplIter<'_> {
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
impl Iterator for StdMemrchrQplIter<'_> {
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
impl DoubleEndedIterator for StdMemrchrQplIter<'_> {
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
impl Iterator for StdMemmemIter<'_> {
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
impl DoubleEndedIterator for StdMemmemIter<'_> {
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
impl Iterator for StdMemrmemIter<'_> {
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
impl DoubleEndedIterator for StdMemrmemIter<'_> {
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
