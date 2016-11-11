pub mod utils {
    pub fn get_hash_val(token: &Vec<u8>) -> u32 {
        let mut hash_val: u32 = 0;
        for i in 0..token.len() {
            hash_val += i as u32 * token[i] as u32;
        };
        hash_val
    }
}