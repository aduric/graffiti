use std::collections::HashMap;
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
        let tag_map = suffix_map.into_iter()
            .map(|s| (utils::get_hash_val(&s.0), s.1))
            .collect();
        
        PosTagger {
            tag_map: tag_map,
        }
    }

    pub fn tag(&self, token: &[u8]) -> Option<&Tag> {
        (0..token.len())
            .rev()
            .filter_map(|i| {
                self.tag_map.get(&utils::get_hash_val(&token[i+1..]))
            })
            .last()
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
