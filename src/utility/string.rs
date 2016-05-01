pub fn substring(s: &str, length: usize) -> String {
    let mut v = Vec::new();
    let mut c = 0;

    for x in s.chars() {
        if cjks_contains(x) {
            if c < length - 2 {
                v.push(x);
                c = c + 2;
            } else {
                break;
            }

        } else {
            if c < length - 1 {
                v.push(x);
                c = c + 1;
            } else {
                break;
            }
        }
    }

    let s: String = v.iter().cloned().collect();
    return s;
}

pub fn cjks_contains(c: char) -> bool {
    let cjks = vec![(0x4E00..0xA000),
                    (0x3400..0x4DC0),
                    (0x20000..0x2A6E0),
                    (0x2A700..0x2B740),
                    (0x2B740..0x2B820),
                    (0xF900..0xFB00),
                    (0x2F800..0x2FA20),
                    (0x9FA6..0x9FCC),

                    (0x3000..0x303F), // CJK Symbols and Punctuation
                    (0xff00..0xffef) /* Halfwidth and Fullwidth Forms */];

    for cjk in cjks {
        let h = c as u32;
        if cjk.start <= h && h < cjk.end {
            return true;
        }
    }
    return false;
}

pub fn jks_len(s: &str) -> usize {
    return s.chars()
            .map(|x| if cjks_contains(x) {
                2
            } else {
                1
            })
            .collect::<Vec<usize>>()
            .iter()
            .fold(0, |acc, &x| acc + x);
}
