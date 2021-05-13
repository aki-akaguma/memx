#[allow(dead_code)]

pub fn create_data() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_ja_1(),
        _ => create_data_en_1(),
    }
}

pub fn create_data_eq() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_eq_ja_1(),
        _ => create_data_eq_en_1(),
    }
}

pub fn create_data_en_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
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
    (v, match_cnt, "my dear WatsThis gentleman,", "", "")
}

pub fn create_data_ja_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
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
    (v, match_cnt, "夏目漱石坊っちゃん", "", "")
}

pub fn create_data_eq_en_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
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
    (v, match_cnt, "aaaaaaaaaaaaaaaaaaaaaaaaaaa", "", "")
}

pub fn create_data_eq_ja_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
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
    (v, match_cnt, "あああああああああ", "", "")
}
