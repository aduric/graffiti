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
        states: Vec<State>
}

impl Tokenizer {
        pub fn new() -> Self {
                Tokenizer {
                        tokens: Vec::new(),
                        states: Vec::new()
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
        pub fn get_tokens(&self) -> &Vec<Token> {
                &self.tokens
        }
        pub fn get_states(&self) -> &Vec<State> {
                &self.states
        }
        fn _emit_token_state(&mut self, token: Token, state: State) {
                println!("Adding: {:?}", token);
                println!("Adding: {:?}", state);
                self.tokens.push(token);
                self.states.push(state);
        }
        pub fn tokenize(&mut self, token_str: &Vec<u8>, state_map: &str, token_map: &str) -> () {
                let mut curr_token: &mut Vec<u8> = &mut Vec::new();
                let mut raw_bytes = token_str.clone();
                let start_state = State(utils::get_hash_val(&String::from("Start").into_bytes()));
                let mut curr_state = State(utils::get_hash_val(&String::from("Start").into_bytes()));
                let mut last_byte_t = TokenType(0);

                let s_map: HashMap<State, HashMap<TokenType, State>> = Tokenizer::compile_states(state_map);
                let t_map: HashMap<u8, TokenType> = Tokenizer::compile_tokens_ascii(token_map);

                while !raw_bytes.is_empty() {
                        println!("Byte: {:?}", raw_bytes[0]);
                        println!("curr_token: {:?}", curr_token);
                        println!("curr_state: {:?}", curr_state);
                        println!("tokens: {:?}", self.tokens);    

                        let curr_byte = raw_bytes[0];
                        let curr_byte_t = match t_map.get(&curr_byte) {
                                None => TokenType(0),
                                Some(v) => v.to_owned()
                        };

                        let new_state = match s_map.get(&curr_state).unwrap().get(&curr_byte_t) {
                                None => {
                                        self._emit_token_state(Token { t: last_byte_t.to_owned(), value: curr_token.to_owned()}, curr_state.to_owned());
                                        curr_token.clear();                                         
                                        State(utils::get_hash_val(&String::from("Start").into_bytes()))
                                },
                                Some(v) => {
                                        if v == &curr_state && curr_state != State(utils::get_hash_val(&String::from("Start").into_bytes())) {
                                                curr_token.push(raw_bytes.remove(0));
                                        }
                                        if v != &curr_state && curr_state != State(utils::get_hash_val(&String::from("Start").into_bytes())) {
                                                self._emit_token_state(Token { t: last_byte_t.to_owned(), value: curr_token.to_owned()}, curr_state.to_owned() );
                                                curr_token.clear();
                                        };
                                        v.to_owned()
                                } 
                        };

                        if raw_bytes.is_empty() {
                                self._emit_token_state(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned() }, new_state.to_owned() );
                        }

                        last_byte_t = curr_byte_t;
                        curr_state = new_state;
                }

        }
}

#[cfg(test)]
mod tests {

        use super::*;
        use utils::*;
        use std::collections::HashMap;


        static tokens: &'static str = 
                "
                Alpha => 65..123
                Number => 48..57
                Whitespace => 9,10,13,32
                Punctuation => 33..46
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

                let mut tokenizer = Tokenizer::new();

                let bs: Vec<u8> = vec![97, 98, 99];

                tokenizer.tokenize(&bs, &transitions, &tokens);
                let test_token = Token { t: TokenType(utils::get_hash_val(&String::from("Alpha").into_bytes())), value: bs };

                println!("{:?}", tokenizer.get_tokens());
                println!("{:?}", &test_token);

                assert_eq!(tokenizer.get_tokens()[0], test_token);
        }

        #[test]
        fn tokenize_test_2() {

                let mut tokenizer = Tokenizer::new();

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

                tokenizer.tokenize(&bs, &transitions, &tokens);
                let test_tokens = vec![&test_alpha, &test_slash, &test_pos];

                println!("{:?}", &test_tokens);
                println!("{:?}", tokenizer.get_tokens());

                assert_eq!(tokenizer.get_tokens().len(), 3);
                assert_eq!(tokenizer.get_tokens()[0], test_alpha);
                assert_eq!(tokenizer.get_tokens()[1], test_slash);
                assert_eq!(tokenizer.get_tokens()[2], test_pos);

                assert_eq!(tokenizer.get_states().len(), 3);
                assert_eq!(tokenizer.get_states()[0], test_state_alpha);
                assert_eq!(tokenizer.get_states()[2], test_state_pos);

        }
}

