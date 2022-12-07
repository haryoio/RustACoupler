use itertools::Itertools;

pub fn encode_u8(data: &str) -> Vec<u8> {
    let mut buf: Vec<[u8; 8]> = Vec::with_capacity(data.len());
    for (i, c) in data.chars().enumerate() {
        buf.push([0; 8]);
        char_to_bin_u8(&mut buf[i], c as u8);
    }
    let res: Vec<u8> = buf.iter().flatten().map(|e| e.to_owned()).collect_vec();
    res
}

pub fn decode_u8(bin: Vec<u8>) -> String {
    if bin.len() % 8 != 0 {
        return format!("Invalid binary data len: {}", bin.len());
    }
    let mut buf: Vec<u8> = Vec::with_capacity(bin.len() / 8);
    for c in bin.chunks(8) {
        let mut a: u8 = 0;
        for (b, j) in c.iter().enumerate() {
            a |= j << (7 - b);
        }
        buf.push(a);
    }

    let res: String = buf.iter().map(|e| *e as char).collect();

    res
}

pub fn char_to_bin_u8(buf: &mut [u8; 8], dec: u8) {
    let mut ctr: u8 = 7;

    loop {
        let k = dec >> ctr;
        let b = k & 1u8;
        let idx = (8 - ctr - 1) as usize;
        if b == 1 {
            buf[idx] = 1
        } else {
            buf[idx] = 0
        }
        if ctr == 0 {
            break;
        }
        ctr -= 1;
    }
}

pub fn u32_from_bytes(vec: &[u8]) -> u32 {
    let mut bit: u32 = 0;
    for (i, _) in vec.iter().enumerate() {
        bit |= (vec[i] as u32) << i;
    }
    bit
}

pub fn u16_from_bytes(vec: &[u8]) -> u16 {
    let mut bit: u16 = 0;
    for (i, _) in vec.iter().enumerate() {
        bit |= (vec[i] as u16) << i;
    }
    bit
}

pub fn u8_from_bytes(vec: &[u8]) -> u8 {
    let mut bit: u8 = 0;
    for (i, _) in vec.iter().enumerate() {
        bit |= (vec[i]) << i;
    }
    bit
}

pub fn u32_to_bytes(num: u32) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = ((num >> i) & 1) as u8;
    }
    bytes
}
pub fn u16_to_bytes(num: u16) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = ((num >> i) & 1) as u8;
    }
    bytes
}
pub fn u8_to_bytes(num: u8) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = (num >> i) & 1;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use crate::bytes::{char_to_bin_u8, decode_u8, encode_u8};

    #[test]
    fn test_str_to_binary() {
        let binary = encode_u8("hello super man");
        let strr = decode_u8(binary);
        println!("{:?}", strr);
    }

    #[test]
    fn test_dec_to_bin() {
        let dec = b'o';
        println!("{:08b}", b'a');
        let mut buf: [u8; 8] = [0; 8];
        char_to_bin_u8(&mut buf, dec);
        println!("{:?}", buf);
    }
    #[test]
    fn test_vec_to_u8() {
        let v = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let mut a: u8 = 0;
        for (b, i) in v.iter().enumerate() {
            a = a | (i << (7 - b));
        }
        println!("{:08b}", a);
    }
}
