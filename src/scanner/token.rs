#[derive(Debug, Clone)]
pub enum Token{
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    EuqalEqual,
    NotEqual,
    Number(u32),
    StringLiteral(String),
    Invalid(String),
}

pub struct Scanner {
    input: String,
    pos: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        let tokens = Vec::new();
        Scanner {
            input: input,
            pos: 0,
            tokens: tokens,
        }
    }
    pub fn scan_tokens(&mut self) -> bool {
        let mut char_indices = self.input.char_indices().peekable();

        while let Some((pos, ch)) = char_indices.next() {
            let token = match ch {
                '+' => Some(Token::Plus),
                '-' =>  Some(Token::Minus),
                '*' =>  Some(Token::Multiply),
                '/' =>  Some(Token::Divide),
                '=' => {
                    match char_indices.next_if_eq(&(pos+1, '=')) {
                        Some(_) =>  Some(Token::EuqalEqual),
                        None =>  Some(Token::Equal),
                    }
                },
                '0'..='9' => {
                    let start = pos;
                    let mut end = pos;
                    while char_indices.next_if(|&(_pos, ch)| ch.is_numeric()).is_some(){
                        end+=1;
                    }
                    if end < self.input.len() {
                        end += 1;
                    }

                    let nstring = self.input[start..end].to_owned();
                    Some(Token::Number(nstring.parse::<u32>().unwrap()))
                },
                // lookahead
                '"' => {
                    let s = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| {*c != '"'})
                    .map(|(_pos, c)| {c})
                    .collect();
                    Some(Token::StringLiteral(s))
                },
                ' ' => None,
                _ => Some(Token::Invalid(format!("invalid"))),
            };

            if let Some(t) = token {
                self.tokens.push(t);
            }
        }

        return true;
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.pos;
        if cur < self.tokens.len() {
            self.pos += 1;
            return Some(self.tokens[cur].clone());
        }
        return None;
    }
}

// cargo test -- --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut scanner = Scanner::new(String::from("1 + 1 = 2"));
        scanner.scan_tokens();
        
        for t in scanner {
            println!("{:?}",t);
        }
    }
}