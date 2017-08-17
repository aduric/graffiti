use std::collections::HashMap;
use utils::*;

pub struct Stemmer {
    suffix_map: HashMap<u32,Vec<u8>>,
}

impl Stemmer {
    pub fn new(suffixes: &[Vec<u8>]) -> Self {
        let suffix_map = suffixes.iter()
            .map(|s| (utils::get_hash_val(s), s.clone()))
            .collect();

        Stemmer {
            suffix_map: suffix_map,
        }
    }

    pub fn stem<'a>(&self, token: &'a [u8]) -> &'a [u8] {
        let mut suffix_end = token.len();
        for c in (0..token.len()).rev() {
            let suffix_split = &token[c+1..];
            if let Some(_) = self.suffix_map.get(&utils::get_hash_val(&suffix_split)) {
                suffix_end -= c;
                break;
            }
        };
        &token[0..suffix_end]
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

        assert!(v_stemmed == &[97,98,99]);
    }
}
