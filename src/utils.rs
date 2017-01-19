pub mod utils {
    pub fn get_hash_val(token: &[u8]) -> u32 {
        let mut hash_val: u32 = 0;
        for i in 0..token.len() {
            hash_val += i as u32 * token[i] as u32;
        };
        hash_val
    }

    pub fn levenshtein(a: &[u8], b: &[u8]) -> u32 {
        use std::cmp::min;

        let a_len = a.len();
        let b_len = b.len();
        let mut lev = vec![0u32; a_len * b_len];

        for n in 1..b_len {
            lev[n] = n as u32;
        }

        for i in 1..a_len {
            lev[i*a_len] = i as u32;
            for j in 1..b_len {
                if a[i] == b[j] {
                    lev[i*a_len+j] = min(lev[(i-1)*a_len+j] + 1, min(lev[i*a_len+j-1] + 1, lev[(i-1)*a_len+j-1]));
                } else {
                    lev[i*a_len+j] = min(lev[(i-1)*a_len+j] + 1, min(lev[i*a_len+j-1] + 1, lev[(i-1)*a_len+j-1] + 1));
                }
            }
        };

        lev[(a_len-1)*(b_len)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_levenshtein_distance() {
        let a: &[u8] = b"algorithm";
        let b: &[u8] = b"altruistic";

        let lev = utils::levenshtein(&a, &b);

        println!("{:?}", lev);

        assert!(lev == 6);
    }
}