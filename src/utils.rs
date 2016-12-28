pub mod utils {

    use std::cmp::min;

    pub fn get_hash_val(token: &Vec<u8>) -> u32 {
        let mut hash_val: u32 = 0;
        for i in 0..token.len() {
            hash_val += i as u32 * token[i] as u32;
        };
        hash_val
    }
    pub fn levenshtein(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        let a_len = a.len();
        let b_len = b.len();
        let cap = a_len * b_len;
        let mut lev: Vec<u32> = Vec::with_capacity(cap);

        for j in 0..b_len {
            lev[j] = j as u32;
        };

        for i in 0..a_len {
            lev[i*a_len] = i as u32;
            for j in 0..b_len {
                if a[i*a_len] == b[i*a_len+j] {
                    lev[i*a_len+j] = min(lev[i*a_len+j-1] + 1, min(lev[i*(a_len-1)+j] + 1, lev[i*(a_len-1)+j-1]));
                } else {
                    lev[i*a_len+j] = min(lev[i*a_len+j-1] + 1, min(lev[i*(a_len-1)+j] + 1, lev[i*(a_len-1)+j-1] + 1));
                }
            }
        };
        lev[(a_len-1)*(b_len-1)-1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn get_levenshtein_distance() {
        let a_vec: Vec<u8> = String::from("algorithm").into_bytes();
        let b_vec: Vec<u8> = String::from("altruistic").into_bytes();

        let lev = utils::levenshtein(&a_vec, &b_vec);

        assert!(lev == 6);
    }
}