use std::collections::HashMap;
use std::str;
use utils::*;

pub struct Stemmer {
    suffix_map: HashMap<u32,Vec<u8>>,
}


impl Stemmer {
    pub fn new(suffixes: &Vec<Vec<u8>>) -> Self {
        let mut _suffix_map = HashMap::new();
        for s in suffixes {
            _suffix_map.insert(utils::get_hash_val(s), s.clone());
        };
        Stemmer {
            suffix_map: _suffix_map,
        }
    }
    pub fn stem(&self, token: &Vec<u8>) -> Vec<u8> {
        let mut suffix_end = token.len();
        for c in (0..token.len()).rev() {
            let suffix_split = token.clone().split_off(c+1);
            if self.suffix_map.get(&utils::get_hash_val(&suffix_split)) != None {
                suffix_end -= c;
                break;
            }
        };
        token[0..suffix_end].to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stem() {
        let stemmer = Stemmer::new(&vec![vec![100,101]]);
        let v = vec![97, 98, 99, 100, 101];

        let v_stemmed = stemmer.stem(&v);

        assert!(v_stemmed == vec![97,98,99]);
    }
}
