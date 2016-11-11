use std::collections::HashMap;
use std::hash::{Hash, SipHasher, Hasher};
use std::cmp::{Eq,PartialEq};
use std::str;
use utils::*;

/*
pub enum State {
        Start,
        Alphanumeric,
        Number,
        Punctuation,
        Whitespace
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
        Alpha,
        Whitespace,
        Punctuation,
        Number,
        Unknown
}
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TokenType(u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State(u32);

#[derive(Debug)]
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
                        let start_state: State = State(utils::get_hash_val(v[0].bytes().collect().to_vec()));
                        let tokens: Vec<&str> = v[1].split("|").collect();
                        if tokens.is_empty() {
                                tokens.push(v[1]);
                        }
                        let end_state: State = State(utils::get_hash_val(v[2].bytes().collect().to_vec()));
                        let x = parsed_trans.entry(start_state).or_insert(HashMap::new());
                        for t in tokens {
                                (*x).entry(TokenType(utils::get_hash_val(t.bytes().collect().to_vec()))).or_insert(end_state);
                        }
                };
                parsed_trans
        }
        pub fn compile_tokens_ascii(tokens: &str) -> HashMap<u8, TokenType> {
                let mut token_map = HashMap::new();
                for t in tokens.lines() {
                        let v: Vec<&str> = t.split("=>").map(|s| s.trim()).collect();
                        let mut vals = Vec::new();
                        if v[1].contains("..") {
                                let mut bounds: Vec<u8> = v[1].split("..").collect().to_vec();
                                vals.append((bounds[0] as u8..bounds[1] as u8).collect());
                        } else if v[1].contains(',') {
                                vals = v[1].split(',').collect();
                        } else {
                                vals.push(v[1] as u8);
                        }
                        for u in vals {
                                token_map[u] = TokenType(utils::get_hash_val(v[0].bytes().collect().to_vec()));
                        }
                };
                token_map
        }
        pub fn get_tokens(&self) -> Vec<Token> {
                self.tokens
        }
        fn _emit_token(&mut self, token: Token) {
                println!("Adding: {:?}", token);
                self.tokens.push(token);
        }
        /*
        fn _match(&mut self, t: u8) -> Token {
                match t {
                        t if t >= 65 && t <= 122 => Token::Alpha,
                        t if t >= 48 && t <= 57 => Token::Number,
                        t if t == 9 || t == 10 || t == 13 || t == 32 => Token::Whitespace,
                        t if (t >= 33 && t <= 47) || (t <= 58 && t >= 64) => Token::Punctuation,
                        _ => Token::Unknown
                }
        }
        */
        pub fn tokenize(&mut self, token_str: &Vec<u8>, state_map: &str, token_map: &str) -> () {
                let mut curr_token: &mut Vec<u8> = &mut Vec::new();
                let mut raw_bytes = token_str.clone();
                let mut curr_state = &State(0);

                let s_map: HashMap<State, HashMap<TokenType, State>> = Tokenizer::compile_states(state_map);
                let t_map: HashMap<u8, TokenType> = Tokenizer::compile_tokens_ascii(token_map);

                while !raw_bytes.is_empty() {
                        //let t = self._match(raw_bytes[0]);
                        println!("Byte: {:?}", raw_bytes[0]);
                        println!("curr_token: {:?}", curr_token);
                        println!("tokens: {:?}", self.tokens);    

                        let curr_byte = raw_bytes[0];
                        let curr_byte_t = match t_map.get(&curr_byte) {
                                None => &TokenType(0),
                                Some(v) => v
                        };

                        let new_state = match s_map.get(&curr_state).unwrap().get(&curr_byte_t) {
                                None => &State(0),
                                Some(v) => {
                                        curr_token.push(raw_bytes.remove(0));
                                        if v != curr_state {
                                                self._emit_token(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned()} );
                                                curr_token.clear();
                                        };
                                        v
                                } 
                        };

                        if raw_bytes.is_empty() {
                                self._emit_token(Token { t: curr_byte_t.to_owned(), value: curr_token.to_owned() } );
                        }

                        curr_state = new_state;
                }

                
                /*
                        match curr_state {
                        
                               State::Start => {
                                        match t {
                                                Token::Alpha => curr_state = State::Alphanumeric,
                                                Token::Number => curr_state = State::Number,
                                                Token::Whitespace => curr_state = State::Whitespace,
                                                Token::Punctuation => curr_state = State::Punctuation,
                                                Token::Unknown => {
                                                        curr_token.push(raw_bytes.remove(0));
                                                }                                              
                                        }
                                }
                                State::Alphanumeric => {
                                        match t {
                                                Token::Alpha | Token::Number => {
                                                        curr_token.push(raw_bytes.remove(0));
                                                        curr_state = State::Alphanumeric;
                                                },
                                                _ => {
                                                        self._emit_token(Token::Alpha, &curr_token);
                                                        curr_token.clear();
                                                        curr_state = State::Start;
                                                }
                                        }
                                },
                                State::Number => {
                                        match t {
                                                Token::Number => {
                                                        curr_token.push(raw_bytes.remove(0));
                                                },
                                                Token::Alpha => {
                                                        curr_state = State::Alphanumeric;
                                                }
                                                _ => {
                                                        self._emit_token(Token::Number, &curr_token);
                                                        curr_token.clear();
                                                        curr_state = State::Start;
                                                }
                                        }
                                },
                                State::Whitespace => {
                                        match t {
                                                Token::Whitespace => {
                                                        raw_bytes.remove(0);
                                                },
                                                _ => curr_state = State::Start
                                        }
                                },
                                State::Punctuation => {
                                        match t {
                                                Token::Punctuation => {
                                                        curr_token.push(raw_bytes.remove(0));
                                                },
                                                _ => {
                                                        self._emit_token(Token::Punctuation, &curr_token);
                                                        curr_token.clear();
                                                        curr_state = State::Start;
                                                }
                                        }
                                }
                        }
                        if raw_bytes.is_empty() {
                                self._emit_token(t, &curr_token);
                                curr_token.clear();
                                curr_state = State::Start;                                                                
                        }
                }
                */
        }
}

#[cfg(test)]
mod tests {

        use super::*;

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

                assert_eq!(tokenizer.get_tokens()[0], (bs, "Alpha"));
        }
}
