use std::collections::HashMap;
use std::cmp::{Eq,PartialEq};
use std::str;
use utils::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TokenType(pub u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State(pub u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
        pub value: Vec<u8>,
        pub t: TokenType
}

pub struct Tokenizer {
        token_map: HashMap<u8, TokenType>,
        transition_map: HashMap<State, HashMap<TokenType, State>>
}

impl Tokenizer {
        pub fn new(token_map: &str, transition_map: &str) -> Self {
                Tokenizer {
                        token_map: Tokenizer::compile_tokens_ascii(token_map),
                        transition_map: Tokenizer::compile_states(transition_map)
                }
        }
        pub fn compile_states(transition_map: &str) -> HashMap<State, HashMap<TokenType, State>> {
                let mut parsed_trans = HashMap::new();
                for t in transition_map.lines() {
                        let v: Vec<&str> = t.split("=>").map(|s| s.trim()).collect();
                        if v[0].is_empty() {
                                continue;
                        }
			let v_bytes = v[0].bytes().collect();
                        let start_state: State = State(utils::get_hash_val(&v_bytes));
                        let mut tokens: Vec<&str> = v[1].split("|").map(|t| t.trim()).collect();
			let v2_bytes = v[2].bytes().collect();
                        let end_state: State = State(utils::get_hash_val(&v2_bytes));

                        let x = parsed_trans.entry(start_state).or_insert(HashMap::new());
                        for t in tokens {
				let mut t_bytes = t.bytes().collect();
                                let token_type = TokenType(utils::get_hash_val(&t_bytes));
                                (*x).entry(token_type).or_insert(end_state.to_owned());
                        }
                };
                parsed_trans
        }
        pub fn compile_tokens_ascii(tokens: &str) -> HashMap<u8, TokenType> {
                let mut token_map = HashMap::new();
                for t in tokens.lines() {
                        let v: Vec<&str> = t.split("=>").map(|s| s.trim()).collect();
                        if v[0].is_empty() {
                                continue;
                        }
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
        pub fn tokenize(&mut self, token_str: &Vec<u8>) -> (Vec<Token>, Vec<State>) {
                let mut tokens: Vec<Token> = Vec::new();
                let mut states: Vec<State> = Vec::new();

                let mut curr_token: Vec<u8> = Vec::new();
                let mut raw_bytes = token_str.clone();
                let start_state = State(utils::get_hash_val(&String::from("Start").into_bytes()));
                let mut curr_state = State(utils::get_hash_val(&String::from("Start").into_bytes()));
                let mut last_byte_t = TokenType(0);

                while !raw_bytes.is_empty() {
                        println!("Byte: {:?}", raw_bytes[0]);
                        println!("curr_token: {:?}", curr_token);
                        println!("curr_state: {:?}", curr_state);
                        println!("tokens: {:?}", tokens);    

                        let curr_byte = raw_bytes[0];
                        let curr_byte_t = match self.token_map.get(&curr_byte) {
                                None => TokenType(0),
                                Some(v) => v.to_owned()
                        };

                        let new_state = match self.transition_map.get(&curr_state).unwrap().get(&curr_byte_t) {
                                None => {
                                        let start_state = State(utils::get_hash_val(&String::from("Start").into_bytes()));
                                        let mut s = &curr_state;
                                        if curr_byte_t != TokenType(0) {
                                                tokens.push(Token { t: last_byte_t.to_owned(), value: curr_token.to_owned()});
                                                states.push(curr_state.to_owned());
                                                curr_token.clear();                                          
                                                s = &start_state;
                                        } else {
                                                raw_bytes.remove(0);
                                        };
                                        s.to_owned()
                                },
                                Some(v) => {
                                        if v == &curr_state && curr_state != State(utils::get_hash_val(&String::from("Start").into_bytes())) {
                                                curr_token.push(raw_bytes.remove(0));
                                        }
                                        if v != &curr_state && curr_state != State(utils::get_hash_val(&String::from("Start").into_bytes())) {
                                                tokens.push(Token { t: last_byte_t.to_owned(), value: curr_token.to_owned()});
                                                states.push(curr_state.to_owned());
                                                curr_token.clear();
                                        };
                                        v.to_owned()
                                } 
                        };

                        if raw_bytes.is_empty() {
                                tokens.push(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned() });
                                states.push(new_state.to_owned());
                        }

                        last_byte_t = curr_byte_t;
                        curr_state = new_state;
                };
                (tokens, states)
        }
}

#[cfg(test)]
mod tests {

        use super::*;
        use utils::*;
        use std::collections::HashMap;


        static tokens: &'static str = 
                "
                Alpha => 65..91
                Alpha => 97..123
                Number => 48..57
                Whitespace => 9,10,13,32
                Punctuation => 33..47
                Punctuation => 58..65
                Slash => 47
                ";

        static transitions: &'static str = 
                "
                Start => Alpha => Alpha
                Start => Number => Number
                Start => Whitespace => Whitespace
                Start => Punctuation => Punctuation
                Start => Slash => Slash
                Slash => Slash => Slash
                Slash => Whitespace => Whitespace
                Slash => Alpha => Pos
                Alpha => Alpha | Number => Alpha
                Pos => Alpha => Pos
                Number => Number => Number
                Number => Alpha => Alpha
                Whitespace => Whitespace => Whitespace
                Punctuation => Punctuation => Punctuation
                ";

        #[test]
        fn test_compile_states() {

                let transition_map: HashMap<State, HashMap<TokenType, State>> = Tokenizer::compile_states(&transitions);

                let test_state_alpha = State(utils::get_hash_val(&String::from("Alpha").into_bytes()));
                let test_token_type_alpha = TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes()));

                assert_eq!(transition_map[&test_state_alpha][&test_token_type_alpha], test_state_alpha);
        }


        #[test]
        fn test_compile_tokens() {

                let token_map: HashMap<u8, TokenType> = Tokenizer::compile_tokens_ascii(&tokens);
                let test_token_type_alpha = TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes()));

                assert_eq!(token_map[&65], test_token_type_alpha);
                assert_eq!(token_map[&122], test_token_type_alpha);
        }

        #[test]
        fn tokenize_test_1() {

                let mut tokenizer = Tokenizer::new(&tokens, &transitions);

                let bs: Vec<u8> = vec![97, 98, 99];

                let tokenized = tokenizer.tokenize(&bs);
                let test_token = Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: bs };

                println!("{:?}", tokenized.0);
                println!("{:?}", &test_token);

                assert_eq!(tokenized.0[0], test_token);
        }

        #[test]
        fn tokenize_test_2() {

                let bs: Vec<u8> = String::from("foo/bar").into_bytes();

                let mut hashed_dict: HashMap<u32, &str> = HashMap::new();
                hashed_dict.insert(utils::get_hash_val(&String::from("Alpha").into_bytes()), "Alpha");
                hashed_dict.insert(utils::get_hash_val(&String::from("Whitespace").into_bytes()), "Whitepace");
                hashed_dict.insert(utils::get_hash_val(&String::from("Slash").into_bytes()), "Slash");
                hashed_dict.insert(utils::get_hash_val(&String::from("Pos").into_bytes()), "Pos");

                println!("{:?}", hashed_dict);

                let test_alpha = Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: String::from("foo").into_bytes() };
                let test_white = Token { t: TokenType(utils::get_hash_val(&String::from("Whitespace").into_bytes())), value: String::from(" ").into_bytes() };
                let test_slash = Token { t: TokenType(utils::get_hash_val(&String::from("Slash").into_bytes())), value: String::from("/").into_bytes() };
                let test_pos = Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: String::from("bar").into_bytes() };

                let test_state_alpha = State(utils::get_hash_val(&String::from("Alpha").into_bytes()));
                let test_state_pos = State(utils::get_hash_val(&String::from("Pos").into_bytes()));

                let mut tokenizer = Tokenizer::new(&tokens, &transitions);

                let tokenized: (Vec<Token>, Vec<State>) = tokenizer.tokenize(&bs);
                let test_tokens = vec![&test_alpha, &test_slash, &test_pos];

                println!("{:?}", &test_tokens);

                assert_eq!(tokenized.0.len(), 3);
                assert_eq!(tokenized.0[0], test_alpha);
                assert_eq!(tokenized.0[1], test_slash);
                assert_eq!(tokenized.0[2], test_pos);

                assert_eq!(tokenized.1.len(), 3);
                assert_eq!(tokenized.1[0], test_state_alpha);
                assert_eq!(tokenized.1[2], test_state_pos);

        }

        #[test]
        fn tokenize_test_3() {

                let bs: Vec<u8> = String::from("foo ^ foo").into_bytes();

                let test_alpha = Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: String::from("foo").into_bytes() };
                let test_white = Token { t: TokenType(utils::get_hash_val(&String::from("Whitespace").into_bytes())), value: String::from("  ").into_bytes() };
                //let test_unknown = Token { t: TokenType(utils::get_hash_val(&String::from("Unknown").into_bytes())), value: String::from("^").into_bytes() };

                let test_state_alpha = State(utils::get_hash_val(&String::from("Alpha").into_bytes()));
                let test_state_white = State(utils::get_hash_val(&String::from("Whitespace").into_bytes()));
                //let test_state_unkown = State(utils::get_hash_val(&String::from("Unknown").into_bytes()));

                let mut tokenizer = Tokenizer::new(&tokens, &transitions);

                let tokenized: (Vec<Token>, Vec<State>) = tokenizer.tokenize(&bs);

                assert_eq!(tokenized.0.len(), 3);
                assert_eq!(tokenized.0[0], test_alpha);
                assert_eq!(tokenized.0[1], test_white);
                assert_eq!(tokenized.0[2], test_alpha);

                assert_eq!(tokenized.1.len(), 3);
                assert_eq!(tokenized.1[0], test_state_alpha);
                assert_eq!(tokenized.1[1], test_state_white);
                assert_eq!(tokenized.1[2], test_state_alpha);

        }
}

