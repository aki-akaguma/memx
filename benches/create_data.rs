#![allow(dead_code)]

pub fn create_data_mem() -> (Vec<String>, &'static str, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_mem_ja_1(),
        _ => create_data_mem_en_1(),
    }
}

pub fn create_data_chr() -> (Vec<String>, u8, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_chr_ja_1(),
        _ => create_data_chr_en_1(),
    }
}

pub fn create_data_cmp() -> (Vec<String>, &'static str, usize, usize, usize) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_cmp_ja_1(),
        _ => create_data_cmp_en_1(),
    }
}

pub fn create_data_cpy() -> (Vec<Vec<u8>>, &'static [u8]) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_cpy_ja_1(),
        _ => create_data_cpy_en_1(),
    }
}

pub fn create_data_set() -> Vec<Vec<u8>> {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_set_ja_1(),
        _ => create_data_set_en_1(),
    }
}

pub fn create_data_mem_en_1() -> (Vec<String>, &'static str, usize) {
    let a1 = "You could not possibl"; // 21 bytes
    let a2 = "my dear Wats"; // 12 bytes
    let a3 = "This gentleman,"; // 15 bytes
    let a4 = "Your cases have indee"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = v.len() / 2;
    (v, "my dear WatsThis gentleman,", match_cnt)
}

pub fn create_data_mem_ja_1() -> (Vec<String>, &'static str, usize) {
    let a1 = "吾輩は猫である"; // 21 bytes
    let a2 = "夏目漱石"; // 12 bytes
    let a3 = "坊っちゃん"; // 15 bytes
    let a4 = "名前はまだない"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = v.len() / 2;
    (v, "夏目漱石坊っちゃん", match_cnt)
}

pub fn create_data_chr_en_1() -> (Vec<String>, u8, usize) {
    let a1 = "You could not possibl"; // 21 bytes
    let a2 = "my dear Wats"; // 12 bytes
    let a3 = "This gentleman,"; // 15 bytes
    let a4 = "Your cases have indee"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 20250;
    (v, b'e', match_cnt)
}

pub fn create_data_chr_ja_1() -> (Vec<String>, u8, usize) {
    let a1 = "吾輩は猫である"; // 21 bytes
    let a2 = "夏目漱石"; // 12 bytes
    let a3 = "坊っちゃん"; // 15 bytes
    let a4 = "名前はまだない"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 5000;
    let c = {
        let bytes = a3.as_bytes();
        bytes[bytes.len() - 1]
    };
    (v, c, match_cnt)
}

pub fn create_data_cmp_en_1() -> (Vec<String>, &'static str, usize, usize, usize) {
    let a1 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let a2 = "aaaaaaaaaaaa"; // 12 bytes
    let a3 = "aaaaaaaaaaaaaa,"; // 15 bytes
    let a4 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 50 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10200;
    let less_cnt = 6750;
    let greater_cnt = 0;
    (
        v,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaa",
        match_cnt,
        less_cnt,
        greater_cnt,
    )
}

pub fn create_data_cmp_ja_1() -> (Vec<String>, &'static str, usize, usize, usize) {
    let a1 = "あああああああ"; // 21 bytes
    let a2 = "ああああ"; // 12 bytes
    let a3 = "あああああ"; // 15 bytes
    let a4 = "あああああああ"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 90 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = 10170;
    let less_cnt = 20340;
    let greater_cnt = 0;
    (v, "あああああああああ", match_cnt, less_cnt, greater_cnt)
}

pub fn create_data_cpy_en_1() -> (Vec<Vec<u8>>, &'static [u8]) {
    let a1 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let a2 = "aaaaaaaaaaaa"; // 12 bytes
    let a3 = "aaaaaaaaaaaaaa,"; // 15 bytes
    let a4 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 50 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    //let match_cnt = 10200;
    (
        v,
        "EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE".as_bytes(),
    )
}

pub fn create_data_cpy_ja_1() -> (Vec<Vec<u8>>, &'static [u8]) {
    let a1 = "あああああああ"; // 21 bytes
    let a2 = "ああああ"; // 12 bytes
    let a3 = "あああああ"; // 15 bytes
    let a4 = "あああああああ"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 90 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    //let match_cnt = 10170;
    (v, "いいいいいいいいいいいいいいい".as_bytes())
}

pub fn create_data_set_en_1() -> Vec<Vec<u8>> {
    let a1 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let a2 = "aaaaaaaaaaaa"; // 12 bytes
    let a3 = "aaaaaaaaaaaaaa,"; // 15 bytes
    let a4 = "aaaaaaaaaaaaaaaaaaaaa"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 50 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    //let match_cnt = 10200;
    v
}

pub fn create_data_set_ja_1() -> Vec<Vec<u8>> {
    let a1 = "あああああああ"; // 21 bytes
    let a2 = "ああああ"; // 12 bytes
    let a3 = "あああああ"; // 15 bytes
    let a4 = "あああああああ"; // 21 bytes
    let s1 = a1.repeat(10) + a2 + a3.repeat(10).as_str(); // 372 = 21*10 + 12 + 15*10
    let s2 = a4.repeat(10) + a3.repeat(10).as_str(); // 360 = 21*10 + 15*10

    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 90 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.as_bytes().to_vec());
        } else {
            v.push(s2.as_bytes().to_vec());
        }
    }
    //let match_cnt = 10170;
    v
}
