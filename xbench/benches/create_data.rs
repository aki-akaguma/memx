#![allow(dead_code)]

pub fn create_data_chr() -> (Vec<String>, u8, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_chr_ja_1(),
        _ => create_data_chr_en_1(),
    }
}

pub fn create_data_chr_dbl() -> (Vec<String>, (u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_chr_dbl_ja_1(),
        _ => create_data_chr_dbl_en_1(),
    }
}

pub fn create_data_chr_tpl() -> (Vec<String>, (u8, u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_chr_tpl_ja_1(),
        _ => create_data_chr_tpl_en_1(),
    }
}

pub fn create_data_chr_qpl() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_chr_qpl_ja_1(),
        _ => create_data_chr_qpl_en_1(),
    }
}

pub fn create_data_nechr() -> (Vec<String>, u8, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_nechr_ja_1(),
        _ => create_data_nechr_en_1(),
    }
}

pub fn create_data_nechr_dbl() -> (Vec<String>, (u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_nechr_dbl_ja_1(),
        _ => create_data_nechr_dbl_en_1(),
    }
}

pub fn create_data_nechr_tpl() -> (Vec<String>, (u8, u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_nechr_tpl_ja_1(),
        _ => create_data_nechr_tpl_en_1(),
    }
}

pub fn create_data_nechr_qpl() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_nechr_qpl_ja_1(),
        _ => create_data_nechr_qpl_en_1(),
    }
}

pub fn create_data_mem() -> (Vec<String>, &'static str, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_mem_ja_1(),
        _ => create_data_mem_en_1(),
    }
}

pub fn create_data_cmp() -> (Vec<String>, &'static str, usize, usize, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_cmp_ja_1(),
        _ => create_data_cmp_en_1(),
    }
}

pub fn create_data_cpy() -> (Vec<Vec<u8>>, Vec<u8>) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_cpy_ja_1(),
        _ => create_data_cpy_en_1(),
    }
}

pub fn create_data_set() -> (Vec<Vec<u8>>, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_set_ja_1(),
        _ => create_data_set_en_1(),
    }
}

//{{{ chr
pub fn create_data_chr_en_1() -> (Vec<String>, u8, usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 768;
    (v, b'r', match_cnt)
}

pub fn create_data_chr_ja_1() -> (Vec<String>, u8, usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 834;
    let by1 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    (v, by1, match_cnt)
}

pub fn create_data_chr_dbl_en_1() -> (Vec<String>, (u8, u8), usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 144 + 768;
    (v, (b'v', b'r'), match_cnt)
}

pub fn create_data_chr_dbl_ja_1() -> (Vec<String>, (u8, u8), usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 134 + 834;
    let by1 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by2 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    (v, (by1, by2), match_cnt)
}

pub fn create_data_chr_tpl_en_1() -> (Vec<String>, (u8, u8, u8), usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 63 + 144 + 768;
    (v, (b'x', b'v', b'r'), match_cnt)
}

pub fn create_data_chr_tpl_ja_1() -> (Vec<String>, (u8, u8, u8), usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 123 + 134 + 834;
    let by1 = {
        let bytes = r"ど".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by2 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by3 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    (v, (by1, by2, by3), match_cnt)
}

pub fn create_data_chr_qpl_en_1() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10 + 63 + 144 + 768;
    (v, (b'z', b'x', b'v', b'r'), match_cnt)
}

pub fn create_data_chr_qpl_ja_1() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 30 + 123 + 134 + 834;
    let by1 = {
        let bytes = r"ぷ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by2 = {
        let bytes = r"ど".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by3 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let by4 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    (v, (by1, by2, by3, by4), match_cnt)
}
//}}}
//{{{ nechr
pub fn create_data_nechr_en_1() -> (Vec<String>, u8, usize) {
    let by0 = b' ';
    let by1 = b'r';
    let v0 = vec![by1];
    let v1: Vec<u8> = EN_DAT_S1
        .to_string()
        .bytes()
        .map(|c| if c == by1 { by0 } else { v0[0] })
        .collect();
    let v2: Vec<u8> = EN_DAT_S2
        .to_string()
        .bytes()
        .map(|c| if c == by1 { by0 } else { v0[0] })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 768;
    (v, by1, match_cnt)
}

pub fn create_data_nechr_ja_1() -> (Vec<String>, u8, usize) {
    let by0 = b' ';
    let by1 = b'r';
    let bbb1 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    let v0 = vec![by1];
    let v1: Vec<u8> = (JA_DAT_S1.to_string() + JA_DAT_S2)
        .bytes()
        .map(|c| if c == bbb1 { by0 } else { v0[0] })
        .collect();
    let v2: Vec<u8> = JA_DAT_S2
        .to_string()
        .bytes()
        .map(|c| if c == bbb1 { by0 } else { v0[0] })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 834;
    (v, by1, match_cnt)
}

pub fn create_data_nechr_dbl_en_1() -> (Vec<String>, (u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'v';
    let by2 = b'r';
    let mut v0 = vec![by1, by2];
    let v1: Vec<u8> = EN_DAT_S1
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = EN_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 144 + 768;
    (v, (by1, by2), match_cnt)
}

pub fn create_data_nechr_dbl_ja_1() -> (Vec<String>, (u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'v';
    let by2 = b'r';
    let bbb1 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb2 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    let mut v0 = vec![by1, by2];
    let v1: Vec<u8> = (JA_DAT_S1.to_string() + JA_DAT_S2)
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = JA_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 134 + 834;
    (v, (by1, by2), match_cnt)
}

pub fn create_data_nechr_tpl_en_1() -> (Vec<String>, (u8, u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'x';
    let by2 = b'v';
    let by3 = b'r';
    let mut v0 = vec![by1, by2, by3];
    let v1: Vec<u8> = EN_DAT_S1
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 || c == by3 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = EN_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 || c == by3 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 63 + 144 + 768;
    (v, (by1, by2, by3), match_cnt)
}

pub fn create_data_nechr_tpl_ja_1() -> (Vec<String>, (u8, u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'x';
    let by2 = b'v';
    let by3 = b'r';
    let bbb1 = {
        let bytes = r"ど".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb2 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb3 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    let mut v0 = vec![by1, by2, by3];
    let v1: Vec<u8> = (JA_DAT_S1.to_string() + JA_DAT_S2)
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 || c == bbb3 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = JA_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 || c == bbb3 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 123 + 134 + 834;
    (v, (by1, by2, by3), match_cnt)
}

pub fn create_data_nechr_qpl_en_1() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'z';
    let by2 = b'x';
    let by3 = b'v';
    let by4 = b'r';
    let mut v0 = vec![by1, by2, by3, by4];
    let v1: Vec<u8> = EN_DAT_S1
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 || c == by3 || c == by4 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = EN_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == by1 || c == by2 || c == by3 || c == by4 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10 + 63 + 144 + 768;
    (v, (by1, by2, by3, by4), match_cnt)
}

pub fn create_data_nechr_qpl_ja_1() -> (Vec<String>, (u8, u8, u8, u8), usize) {
    let by0 = b' ';
    let by1 = b'z';
    let by2 = b'x';
    let by3 = b'v';
    let by4 = b'r';
    let bbb1 = {
        let bytes = r"ぷ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb2 = {
        let bytes = r"ど".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb3 = {
        let bytes = r"じ".as_bytes();
        bytes[bytes.len() - 1]
    };
    let bbb4 = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    let mut v0 = vec![by1, by2, by3, by4];
    let v1: Vec<u8> = (JA_DAT_S1.to_string() + JA_DAT_S2)
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 || c == bbb3 || c == bbb4 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let v2: Vec<u8> = JA_DAT_S2
        .to_string()
        .bytes()
        .map(|c| {
            if c == bbb1 || c == bbb2 || c == bbb3 || c == bbb4 {
                by0
            } else {
                v0.rotate_left(1);
                v0[0]
            }
        })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 30 + 123 + 134 + 834;
    (v, (by1, by2, by3, by4), match_cnt)
}
//}}}
//{{{ etc
pub fn create_data_mem_en_1() -> (Vec<String>, &'static str, usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10;
    let s = r#"just before we went into the very simple problem"#;
    (v, s, match_cnt)
}

pub fn create_data_mem_ja_1() -> (Vec<String>, &'static str, usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10;
    let s = "顔がつるつるしてまるで薬缶";
    (v, s, match_cnt)
}

pub fn create_data_cmp_en_1() -> (Vec<String>, &'static str, usize, usize, usize) {
    let cc = 'r';
    let s1: String = EN_DAT_S1
        .to_string()
        .chars()
        .map(|c| if c == cc { 'a' } else { cc })
        .collect();
    let s2: String = EN_DAT_S2
        .to_string()
        .chars()
        .map(|c| if c == cc { 'z' } else { cc })
        .collect();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 7 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 144;
    let less_cnt = 138;
    let greater_cnt = 60;
    let match_s = "rrrrrrrrrrrrrrr"; // 15 bytes
    (v, match_s, match_cnt, less_cnt, greater_cnt)
}

pub fn create_data_cmp_ja_1() -> (Vec<String>, &'static str, usize, usize, usize) {
    let by1 = b'r';
    let bbb = {
        let bytes = r"る".as_bytes();
        bytes[bytes.len() - 1]
    };
    let v1: Vec<u8> = JA_DAT_S1
        .to_string()
        .bytes()
        .map(|c| if c == bbb { b'a' } else { by1 })
        .collect();
    let v2: Vec<u8> = JA_DAT_S2
        .to_string()
        .bytes()
        .map(|c| if c == bbb { b'z' } else { by1 })
        .collect();
    let s1: String = String::from_utf8_lossy(&v1).to_string();
    let s2: String = String::from_utf8_lossy(&v2).to_string();

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 41 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 20;
    let less_cnt = 160;
    let greater_cnt = 126;
    #[rustfmt::skip]
    let match_s = concat!(
        "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr",
        "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr", "rrrrrrrrrrrrrrrrrr",
    ); // 144 bytes
    (v, match_s, match_cnt, less_cnt, greater_cnt)
}

pub fn create_data_cpy_en_1() -> (Vec<Vec<u8>>, Vec<u8>) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 21 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    let src_s = "E1234567890123E".as_bytes(); // 15 bytes
    (v, src_s.to_vec())
}

pub fn create_data_cpy_ja_1() -> (Vec<Vec<u8>>, Vec<u8>) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 41 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    let b1 = "いいいいいいいいいいいいいいいい"; // 48 bytes
    let m1 = b1.repeat(3);
    let _src_s = m1.as_bytes(); // 144 bytes
    (v, _src_s.to_vec())
}

pub fn create_data_set_en_1() -> (Vec<Vec<u8>>, usize) {
    let s1 = EN_DAT_S1.to_string();
    let s2 = EN_DAT_S2.to_string();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 11 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    (v, 15)
}

pub fn create_data_set_ja_1() -> (Vec<Vec<u8>>, usize) {
    let s1 = JA_DAT_S1.to_string() + JA_DAT_S2;
    let s2 = JA_DAT_S2.to_string();
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 41 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    (v, 144)
}
//}}}

// data for test
const EN_DAT_S1: &str = r#"You could not possibly have come at a better time, my dear Watson, he said cordially. I was afraid that you were engaged. So I am. Very much so. Then I can wait in the next room. Not at all. This gentleman, Mr. Wilson, has been my partner and helper in many of my most successful cases, and I have no doubt that he will be of the utmost use to me in yours also.
Try the settee, said Holmes, relapsing into his armchair and putting his finger-tips together, as was his custom when in judicial moods. I know, my dear Watson, that you share my love of all that is bizarre and outside the conventions and humdrum routine of everyday life.
You have shown your relish for it by the enthusiasm which has prompted you to chronicle, and, if you will excuse my saying so, somewhat to embellish so many of my own little adventures.

"Your cases have indeed been of the greatest interest to me," I observed.

You will remember that I remarked the other day, just before we went into the very simple problem presented by Miss Mary Sutherland, that for strange effects and extraordinary combinations we must go to life itself, which is always far more daring than any effort of the imagination.
A proposition which I took the liberty of doubting.
"#; // 1233 byte

const EN_DAT_S2: &str = r#"For the Doctor Watsons of this world, as opposed to the Sherlock Holmeses, success in the province of detective work must always be, to a very large extent, the result of luck. Sherlock Holmes can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted, and exhibited clearly, with a label attached.
"#; // 367 byte

const JA_DAT_S1: &str = r#"吾輩は猫である。名前はまだ無い。
どこで生れたかとんと見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。吾輩はここで始めて人間というものを見た。しかもあとで聞くとそれは書生という人間中で一番獰悪な種族であったそうだ。この書生というのは時々我々を捕えて煮て食うという話である。しかしその当時は何という考もなかったから別段恐しいとも思わなかった。ただ彼の掌に載せられてスーと持ち上げられた時何だかフワフワした感じがあったばかりである。掌の上で少し落ちついて書生の顔を見たのがいわゆる人間というものの見始であろう。この時妙なものだと思った感じが今でも残っている。第一毛をもって装飾されべきはずの顔がつるつるしてまるで薬缶だ。その後猫にもだいぶ逢ったがこんな片輪には一度も出会わした事がない。のみならず顔の真中があまりに突起している。そうしてその穴の中から時々ぷうぷうと煙を吹く。どうも咽せぽくて実に弱った。これが人間の飲む煙草というものである事はようやくこの頃知った。
"#; // 1346 byte

const JA_DAT_S2: &str = r#"
この書生の掌の裏でしばらくはよい心持に坐っておったが、しばらくすると非常な速力で運転し始めた。書生が動くのか自分だけが動くのか分らないが無暗に眼が廻る。胸が悪くなる。到底助からないと思っていると、どさりと音がして眼から火が出た。それまでは記憶しているがあとは何の事やらいくら考え出そうとしても分らない。
ふと気が付いて見ると書生はいない。たくさんおった兄弟が一疋も見えぬ。肝心の母親さえ姿を隠してしまった。その上今までの所とは違って無暗に明るい。眼を明いていられぬくらいだ。はてな何でも容子がおかしいと、のそのそ這い出して見ると非常に痛い。吾輩は藁の上から急に笹原の中へ棄てられたのである。
"#; // 884 byte
