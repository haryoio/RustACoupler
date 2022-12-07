pub fn encode_u8(data: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(data.len() * 8);
    for c in data.chars() {
        buf.extend(u8_to_bytes(c as u8));
    }
    buf
}

pub fn decode_u8(bin: Vec<u8>) -> String {
    if bin.len() % 8 != 0 {
        return format!("Invalid binary data len: {}", bin.len());
    }
    let mut buf = Vec::with_capacity(bin.len() / 8);
    for c in bin.chunks(8) {
        let mut a: u8 = 0;
        for (i, &b) in c.iter().enumerate() {
            a |= b << i;
        }
        buf.push(a);
    }
    let buf = buf.iter().map(|&c| c as char).collect::<String>();

    buf
}

pub fn char_to_byte(c: char) -> [u8; 8] {
    let c = c as u8;
    let mut buf: [u8; 8] = [0; 8];
    let mut ctr: u8 = 7;

    loop {
        buf[(8 - ctr - 1) as usize] = c >> ctr & 1u8;
        if ctr == 0 {
            break;
        }
        ctr -= 1;
    }

    buf
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
// Rustのboolは1byteであり､u8などとサイズは同じ
// enum Bit {
//     Zero,
//     One,
// }

// impl Bit {
//     fn to_u8(&self) -> u8 {
//         match self {
//             Bit::Zero => 0,
//             Bit::One => 1,
//         }
//     }
// }

// struct BitVec {
//     bits: VecDeque<Bit>,
// }

// impl BitVec {
//     fn new() -> Self {
//         BitVec {
//             bits: VecDeque::with_capacity(8),
//         }
//     }
//     fn with_capacity(cap: usize) -> Self {
//         BitVec {
//             bits: VecDeque::with_capacity(cap),
//         }
//     }
// }

// impl Iterator for BitVec {
//     type Item = Bit;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.bits.pop_front().map(|b| Bit::from(b.to_u8()))
//     }
// }

// impl From<u8> for Bit {
//     fn from(num: u8) -> Self {
//         match num {
//             0 => Bit::Zero,
//             1 => Bit::One,
//             _ => panic!("Invalid bit value"),
//         }
//     }
// }
// impl From<u16> for Bit {
//     fn from(num: u16) -> Self {
//         match num {
//             0 => Bit::Zero,
//             1 => Bit::One,
//             _ => panic!("Invalid bit value"),
//         }
//     }
// }
// impl From<u32> for Bit {
//     fn from(num: u32) -> Self {
//         match num {
//             0 => Bit::Zero,
//             1 => Bit::One,
//             _ => panic!("Invalid bit value"),
//         }
//     }
// }
// impl From<u64> for Bit {
//     fn from(num: u64) -> Self {
//         match num {
//             0 => Bit::Zero,
//             1 => Bit::One,
//             _ => panic!("Invalid bit value"),
//         }
//     }
// }

// impl From<u8> for BitVec {
//     fn from(num: u8) -> Self {
//         let mut bits = BitVec::with_capacity(8);
//         for i in 0..8 {
//             bits.bits.push_front(Bit::from((num >> i) & 1));
//         }
//         bits
//     }
// }
// impl From<u16> for BitVec {
//     fn from(num: u16) -> Self {
//         let mut bits = BitVec::with_capacity(16);
//         for i in 0..16 {
//             bits.bits.push_front(Bit::from((num >> i) & 1));
//         }
//         bits
//     }
// }
// impl From<u32> for BitVec {
//     fn from(num: u32) -> Self {
//         let mut bits = BitVec::with_capacity(32);
//         for i in 0..32 {
//             bits.bits.push_front(Bit::from((num >> i) & 1));
//         }
//         bits
//     }
// }
// impl From<u64> for BitVec {
//     fn from(num: u64) -> Self {
//         let mut bits = BitVec::with_capacity(64);
//         for i in 0..64 {
//             bits.bits.push_front(Bit::from((num >> i) & 1));
//         }
//         bits
//     }
// }

// impl Display for BitVec {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for bit in self.bits.iter() {
//             write!(f, "{}", bit.to_u8())?;
//         }
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use std::time::Instant;

    // use super::BitVec;
    use crate::bytes::{
        char_to_byte,
        decode_u8,
        encode_u8,
        // u32_from_bytes,
        u32_to_bytes,
        u8_to_bytes,
    };

    #[test]
    fn test_str_to_binary() {
        let binary = encode_u8("hello super man");
        let strr = decode_u8(binary);
        println!("{:?}", strr);
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

    #[test]
    fn test_string_to_byte() {
        let line = "string";
        // let mut buf = vec![];
        for c in line.clone().chars() {
            println!("{:?}", char_to_byte(c));
            println!("{:08b}", c as u8);
        }
        // println!("{:?}", buf);
    }
    // 1000000回
    // 2.788305181s
    // 4.104479981s
    // 2.07504064s
    // 2.289092772s
    // #[test]
    // fn test_bitvec_u8() {
    //     let start = Instant::now();

    //     for num in 0..1000000 {
    //         let res = BitVec::from(num as u32);
    //     }
    //     let end = Instant::now();
    //     println!(
    //         "Time elapsed in BitVec from u32 is: {:?}",
    //         end.duration_since(start)
    //     );
    // }
    // 1.480609398s
    // 1.467942894s
    // 1.668451503s
    #[test]
    fn test_u32_to_bytes() {
        let start = Instant::now();

        for num in 0..1000000 {
            let _ = u32_to_bytes(num as u32);
        }
        let end = Instant::now();
        println!(
            "Time elapsed in u32 to bytes function is: {:?}",
            end.duration_since(start)
        );
    }

    // 524.013977ms
    // 433.325444ms
    #[test]
    fn test_u8_to_bytes() {
        let start = Instant::now();

        for num in 0..1000000 {
            let _ = u8_to_bytes(num as u8);
        }
        let end = Instant::now();
        println!(
            "Time elapsed in u8 to bytes function is: {:?}",
            end.duration_since(start)
        );
    }
}
