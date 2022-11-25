use std::{cell::RefCell, rc::Rc};

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

pub fn decode_u8(bin: Vec<u8>) -> Vec<char> {
    // 整数値に変換された値を保存する配列
    let mut decs = vec![];
    // 8文字ずつバイナリを変換していくため使用する配列
    let tmp = RefCell::new(vec![]);

    for b in bin {
        tmp.borrow_mut().push(b);

        if tmp.borrow_mut().len() == 8 {
            let mut tmp_str = "".to_string();
            for c in tmp.borrow().iter() {
                tmp_str.push_str(&format!("{}", c));
            }
            let dec = u8::from_str_radix(&tmp_str, 2).unwrap();
            decs.push(dec as char);
            tmp.borrow_mut().clear();
        }
    }

    decs
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
        ctr = ctr - 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::ascii::{char_to_bin_u8, decode_u8, encode_u8};

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
}
