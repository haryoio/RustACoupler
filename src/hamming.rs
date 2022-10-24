pub(crate) mod Hamming {
    use std::ops::Div;

    use nalgebra::{DMatrix, Dynamic};

    /// 8x4
    static GENERATE_MATRIX: [u8; 32] = [
        1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1,
        1, 0,
    ];

    /// 4x8
    static CHECK_MATRIX: [u8; 32] = [
        1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
        1, 1,
    ];
    /// 4x8
    static EXTRACT_MATRIX: [u8; 32] = [
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0,
    ];

    static SYNDROME_TO_INDEX_MATRIX: [u8; 4] = [1, 2, 4, 0];

    /// input は常に4bitの倍数でなければならない。
    pub fn calc_parity(data: Vec<u8>) -> Vec<u8> {
        let len = data.len();
        if len % 4 != 0 {
            panic!()
        }

        let gen_mat = DMatrix::from_row_slice(8, 4, &GENERATE_MATRIX);
        let data_mat = DMatrix::from_vec(1, len, data);

        let mut p = vec![0; 8];
        for (i, e) in gen_mat.row_iter().enumerate() {
            p[i] = e.dot(&data_mat) % 2;
        }

        p.push(parity(&p));

        return p.to_vec();
    }
    fn parity(bits: &[u8]) -> u8 {
        let mut buf = 0;
        for b in bits {
            buf = buf ^ b;
        }
        buf
    }
}

#[cfg(test)]
mod tesd {
    use super::Hamming::calc_parity;

    #[test]
    fn test_calc_parity() {
        let res = calc_parity(vec![0, 1, 0, 1]);
        println!("{:?}", res);
    }
}
