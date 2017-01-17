use std::collections::HashMap;

pub struct InvertedIndex {
    dictionary: HashMap<u32,Vec<u32>>,
    tfs: HashMap<u32,Vec<u32>>,
    idfs: HashMap<u32,u32>,
    tws: HashMap<u32,f32>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            dictionary: HashMap::new(),
            tfs: HashMap::new(),
            idfs: HashMap::new(),
            tws: HashMap::new()
        }
    }
    pub fn get_hash_val(&self, token: &[u8]) -> u32 {
        let mut hash_val: u32 = 0;
        for i in 0..token.len() {
            hash_val += i as u32 * token[i] as u32;
        };
        hash_val
    }
    pub fn get_docs(&self, token: &[u8]) -> &Vec<u32> {
        let s: u32 = self.get_hash_val(token);
        &self.dictionary[&s]
    }
    pub fn get_tfs(&self, token: &[u8]) -> &Vec<u32> {
        let s: u32 = self.get_hash_val(token);
        &self.tfs[&s]
    }
    pub fn get_idf(&self, token: &[u8]) -> u32 {
        let s: u32 = self.get_hash_val(token);
        self.idfs[&s]
    }
    pub fn add_doc(&mut self, tokens: &[Vec<u8>], doc: u32) {
        let mut token_freqs = HashMap::new();
        for token in tokens {
            let t = token_freqs.entry(token).or_insert(0);
            *t += 1;
        }
        for (token, freq) in token_freqs {
            let s: u32 = self.get_hash_val(token);
            self.dictionary.entry(s).or_insert(Vec::new()).push(doc);
            self.tfs.entry(s).or_insert(Vec::new()).push(freq);
            let x = self.idfs.entry(s).or_insert(0);
            *x += 1;
            let w = self.tws.entry(doc).or_insert(0.0);
            *w += (freq as f32 * *x as f32).powi(2);
        }
    }
    pub fn get_ranking(&self, query: &[Vec<u8>]) -> HashMap<u32,f64> {
        let mut weights: HashMap<u32,u64> = HashMap::new();
        let mut rankings: HashMap<u32,f64> = HashMap::new();
        let mut token_freqs: HashMap<&[u8],u32> = HashMap::new();
        let mut query_weight = 0.0;
        for token in query {
            let t = token_freqs.entry(token).or_insert(0);
            *t += 1;
        }
        for (token, freq) in token_freqs {
            let idf = self.get_idf(&token);
            query_weight += (freq as f32).powi(2);
            for (d, tf) in self.get_docs(token).iter().zip(self.get_tfs(token).iter()) {
                let w = weights.entry(*d).or_insert(0);
                *w += (tf * idf) as u64;
            }
        };
        query_weight = query_weight.sqrt();
        for (d, w) in weights {
            rankings.insert(d, w as f64 / (query_weight as f64 * self.tws[&d].sqrt() as f64) as f64);
        };
        rankings
    }
    pub fn print_internal(&self) {
        println!("{:?}", self.dictionary);
        println!("{:?}", self.tfs);
        println!("{:?}", self.idfs);
        println!("{:?}", self.tws);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_docs_at() {
        let mut ii = InvertedIndex::new();

        let doc = vec![
            vec![97, 98, 99]
        ];

        ii.add_doc(&doc, 42);

        let v_test = vec![97, 98, 99];

        assert!((&ii.get_docs(&v_test)).len() == 1);
        assert!((&ii.get_tfs(&v_test)).len() == 1);
    }

    #[test]
    fn search() {
        let mut ii = InvertedIndex::new();
        let doc1 = vec![
            vec![97, 98, 99],
            vec![99, 100, 101]
        ];

        let doc2 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6]
        ];

        ii.add_doc(&doc1, 42);
        ii.add_doc(&doc2, 43);

        let query = vec![
            vec![97, 98, 99]
        ];
        
        ii.print_internal();

        assert!((&ii.get_ranking(&query)).len() == 1);
        assert!(&ii.get_ranking(&query)[&42] > &0.0f64);

    }
}
