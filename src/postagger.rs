use std::collections::HashMap;
use std::str;
use utils::*;

#[derive(PartialEq, Debug)]
pub enum Tag {
    N,
    VB,
    ADJ
}

pub struct PosTagger {
    tag_map: HashMap<u32,Tag>
}

impl PosTagger {
    pub fn new(suffix_map: HashMap<Vec<u8>,Tag>) -> Self {
        let mut _tag_map = HashMap::new();
        for (s, t) in suffix_map {
            _tag_map.insert(utils::get_hash_val(&s), t);
        };
        PosTagger {
            tag_map: _tag_map,
        }
    }
    pub fn tag(&self, token: &Vec<u8>) -> Option<&Tag> {
        let mut tag = None;
        for c in (0..token.len()).rev() {
            let suffix_split = token.clone().split_off(c+1);
            let suffix = self.tag_map.get(&utils::get_hash_val(&suffix_split));
            if suffix != None {
                tag = suffix;
            }
        };
        tag
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_tagging() {
        let test_tag = Tag::N;
        let mut tag_map = HashMap::new();
        tag_map.insert(vec!(99,100,101), Tag::N);

        let tagger = PosTagger::new(tag_map);
        let v = vec![97, 98, 99, 100, 101];

        let v_tag = tagger.tag(&v);

        assert!(v_tag == Some(&test_tag));
    }
}
