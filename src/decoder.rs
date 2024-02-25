use std::mem;

// pub fn create_pattern_mask(pattern: &'static str) -> u32 {
//     let mut index = 0;
//     let mut mask = 0;
//     while let Some(ch) = pattern.chars().nth(index) {

//     }

//     mask
// }



struct Decoder {

}

const fn create_pattern_mask<T>(pattern: &'static str) -> u32 {
    let mut mask = 0;
    let bytes = pattern.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        let ch = bytes[i] as char;
        if ch == '0' || ch == '1' {
            mask |= 1 << (len - i - 1);
        }
        
        i += 1;
    }

    mask
}

const fn create_pattern_value<T>(pattern: &'static str) -> u32 {
    let mut mask = 0;
    let bytes = pattern.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        let ch = bytes[i] as char;
        if ch == '1' {
            mask |= 1 << (len - i - 1);
        }
        
        i += 1;
    }

    mask
}