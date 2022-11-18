pub mod Hamming {

    use nalgebra::DMatrix;

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

    pub fn get_hamming_code(bin: Vec<u8>) -> Vec<u8> {
        let origin = bin.clone();

        if bin.len() % 4 != 0 {
            panic!("4の倍数値の必要があります。");
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

    pub fn correct(data: Vec<u8>) {
        let back = data.clone();
        let len = data.len();
        let check_mat = DMatrix::from_row_slice(4, 8, &CHECK_MATRIX);
        let data_mat = DMatrix::from_vec(1, 8, data);

        let mut syndrome = vec![0; 4];
        for (i, e) in check_mat.row_iter().enumerate() {
            syndrome[i] = e.dot(&data_mat);
        }
        println!("{:?}", syndrome);
        if syndrome[syndrome.len() - 1] == 0 {
            panic!("");
        }

        if any(&syndrome) {}
    }

    fn any(data: &Vec<u8>) -> bool {
        for e in data {
            if e == &1 {
                return true;
            }
        }
        return false;
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
