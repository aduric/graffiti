use std::collections::HashMap;
use std::cmp::{Eq,PartialEq};
use std::str;
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TokenType(u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State(u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
        value: Vec<u8>,
        t: TokenType
}

pub struct Tokenizer {
        tokens: Vec<Token>,
}

impl Tokenizer {
        pub fn new() -> Self {
                Tokenizer {
                        tokens: Vec::new(),
                }
        }
        pub fn compile_states(transition_map: &str) -> HashMap<State, HashMap<TokenType, State>> {
                let mut parsed_trans = HashMap::new();
                for t in transition_map.lines() {
                        let v: Vec<&str> = t.split("=>").map(|s| s.trim()).collect();
			let v_bytes = v[0].bytes().collect();
                        let start_state: State = State(utils::get_hash_val(&v_bytes));
                        let mut tokens: Vec<&str> = v[1].split("|").collect();
                        if tokens.is_empty() {
                                tokens.push(v[1]);
                        }
			let v2_bytes = v[2].bytes().collect();
                        let end_state: State = State(utils::get_hash_val(&v2_bytes));
                        let x = parsed_trans.entry(start_state).or_insert(HashMap::new());
                        for t in tokens {
				let mut t_bytes = t.bytes().collect();
                                (*x).entry(TokenType(utils::get_hash_val(&t_bytes))).or_insert(end_state.to_owned());
                        }
                };
                parsed_trans
        }
        pub fn compile_tokens_ascii(tokens: &str) -> HashMap<u8, TokenType> {
                let mut token_map = HashMap::new();
                for t in tokens.lines() {
                        let v: Vec<&str> = t.split("=>").map(|s| s.trim()).collect();
                        let mut vals: Vec<u8> = Vec::new();
                        if v[1].contains("..") {
                                let mut bounds: Vec<u8> = v[1].split("..").map(|c| c.parse::<u8>().unwrap()).collect();
                                vals.append(&mut (bounds[0]..bounds[1]).collect());
                        } else if v[1].contains(',') {
				let mut nums: Vec<u8> = v[1].split(',').map(|i| i.parse::<u8>().unwrap()).collect();
                                vals.append(&mut nums);
                        } else {
                                let single_num = v[1].parse::<u8>().unwrap(); 
                                vals.push(single_num);
                        }
                        let t_bytes = v[0].bytes().collect();
                        for u in vals {
                                token_map.insert(u, TokenType(utils::get_hash_val(&t_bytes)));
                        }
                };
                token_map
        }
        pub fn get_tokens(&self) -> &Vec<Token> {
                &self.tokens
        }
        fn _emit_token(&mut self, token: Token) {
                println!("Adding: {:?}", token);
                self.tokens.push(token);
        }
        pub fn tokenize(&mut self, token_str: &Vec<u8>, state_map: &str, token_map: &str) -> () {
                let mut curr_token: &mut Vec<u8> = &mut Vec::new();
                let mut raw_bytes = token_str.clone();
                let mut curr_state = State(0);

                let s_map: HashMap<State, HashMap<TokenType, State>> = Tokenizer::compile_states(state_map);
                let t_map: HashMap<u8, TokenType> = Tokenizer::compile_tokens_ascii(token_map);

                while !raw_bytes.is_empty() {
                        println!("Byte: {:?}", raw_bytes[0]);
                        println!("curr_token: {:?}", curr_token);
                        println!("tokens: {:?}", self.tokens);    

                        let curr_byte = raw_bytes[0];
                        let curr_byte_t = match t_map.get(&curr_byte) {
                                None => TokenType(0),
                                Some(v) => v.to_owned()
                        };

                        let new_state = match s_map.get(&curr_state).unwrap().get(&curr_byte_t) {
                                None => State(0),
                                Some(v) => {
                                        curr_token.push(raw_bytes.remove(0));
                                        if v.to_owned() != curr_state {
                                                self._emit_token(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned()} );
                                                curr_token.clear();
                                        };
                                        v.to_owned()
                                } 
                        };

                        if raw_bytes.is_empty() {
                                self._emit_token(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned() } );
                        }

                        curr_state = new_state;
                }

        }
}

#[cfg(test)]
mod tests {

        use super::*;
        use utils::*;

        #[test]
        fn tokenize_test_1() {
                let tokens =
                "
                Alpha => 65..123
                Number => 48..57
                Whitespace => 9,10,13,32
                Punctuation => 33..46
                Punctuation => 58..65
                Slash => 47
                ";

                let transitions =
                "
                Alpha => Alpha | Number => Alpha
                Slash => Slash => Pos
                Pos => Alpha => Pos
                Number => Number => Number
                Number => Alpha => Alpha
                Whitespace => Whitespace => Whitespace
                Punctuation => Punctuation => Punctuation
                ";

                let mut tokenizer = Tokenizer::new();

                let bs: Vec<u8> = vec![97, 98, 99];

                tokenizer.tokenize(&bs, &tokens, &transitions);

                println!("{:?}", bs);
                println!("{:?}", tokenizer.get_tokens());

                assert_eq!(tokenizer.get_tokens()[0], Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: bs } );
        }
}
