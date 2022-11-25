pub mod Hamming {

    use std::mem;

    use nalgebra::DMatrix;

    /// 8x4
    /// hamming code generator matrix
    #[rustfmt::skip]
    static GENERATE_MATRIX: [u8; 32] = [
        1, 1, 0, 1,
        1, 0, 1, 1,
        1, 0, 0, 0,
        0, 1, 1, 1,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
        1, 1, 1, 0,
    ];

    /// 4x8
    /// parity check matrix
    #[rustfmt::skip]
    static CHECK_MATRIX: [u8; 32] = [
        1, 0, 1, 0, 1, 0, 1, 0,
        0, 1, 1, 0, 0, 1, 1, 0,
        0, 0, 0, 1, 1, 1, 1, 0,
        1, 1, 1, 1, 1, 1, 1, 1,
    ];

    /// 4x8
    /// hamming code extract matrix
    #[rustfmt::skip]
    static EXTRACT_MATRIX: [u8; 32] = [
        0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0,
    ];

    static SYNDROME_TO_INDEX_MATRIX: [u8; 4] = [1, 2, 4, 0];

    pub fn get_hamming_code(bin: Vec<u8>) -> Vec<u8> {
        let origin = bin.clone();

        if bin.len() % 4 != 0 {
            panic!("データの長さが4の倍数値ではありません。");
        }

        if bin.len() > 4 {
            let mut buf = vec![];
            let target = &origin[..4];
            let res = calc_parity(target.to_vec());
            let res1 = get_hamming_code(origin[4..].to_vec());
            buf.extend(res);
            buf.extend(res1);
            return buf;
        } else {
            return calc_parity(origin);
        }
    }

    pub fn correct_hamming_code(bin: Vec<u8>) -> Vec<u8> {
        let mut origin = bin.clone();
        let mut output = vec![];

        if bin.len() % 8 != 0 {
            panic!("データの長さが8の倍数値ではありません。");
        }

        loop {
            if origin.len() == 0 {
                break;
            }

            let target = &origin.clone()[..8];
            origin = (&origin[8..]).to_vec();
            let res = correct(&target);
            output.extend(res);
        }
        return output;
    }

    fn calc_parity(data: Vec<u8>) -> Vec<u8> {
        if data.len() % 4 != 0 {
            panic!()
        }
        let gen_mat = DMatrix::from_row_slice(8, 4, &GENERATE_MATRIX);
        let data_mat = DMatrix::from_vec(1, 4, data);

        let mut p = vec![0; 8];
        for (i, e) in gen_mat.row_iter().enumerate() {
            p[i] = e.dot(&data_mat) % 2;
        }

        return p.to_vec();
    }

    fn correct(data: &[u8]) -> Vec<u8> {
        let mut data = data.to_vec();

        let check_mat = DMatrix::from_row_slice(4, 8, &CHECK_MATRIX);
        let extract_mat = DMatrix::from_row_slice(4, 8, &EXTRACT_MATRIX);
        let data_mat = DMatrix::from_vec(1, 8, data.clone());

        let mut syndrome = vec![0; 4];
        for (i, e) in check_mat.row_iter().enumerate() {
            syndrome[i] = e.dot(&data_mat) % 2;
        }

        if syndrome[syndrome.len() - 1] == 0 && any(&syndrome[..syndrome.len() - 1]) {
            panic!("detected double error");
        }

        if any(&syndrome) {
            let index = syndrome_to_index(&syndrome);
            data[index] = data[index] ^ 1;
        }

        let mut result = vec![0; 4];
        for (i, e) in extract_mat.row_iter().enumerate() {
            result[i] = e.dot(&data_mat);
        }

        result
    }

    fn any(data: &[u8]) -> bool {
        for e in data {
            if e == &1 {
                return true;
            }
        }
        return false;
    }

    fn syndrome_to_index(syndrome: &[u8]) -> usize {
        let mut buf = 0;
        for (i, e) in syndrome.iter().enumerate() {
            buf = buf + e * SYNDROME_TO_INDEX_MATRIX[i];
        }
        buf as usize
    }
}

#[cfg(test)]
mod tesd {
    use crate::hamming::Hamming::{correct_hamming_code, get_hamming_code};

    #[test]
    fn test_calc_parity() {
        let res = get_hamming_code(vec![0, 1, 0, 1]);
        println!("{:?}", res);
    }

    #[test]
    fn test_x1_error() {
        let data = vec![0, 1, 0, 0, 1, 0, 1, 1];
        let res = correct_hamming_code(data);
        println!("{:?}", res);
    }
}
