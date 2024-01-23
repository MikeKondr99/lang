use std::collections::HashMap;


#[derive(PartialEq,Eq,Debug,Clone)]
pub enum Token {
    If,
    Space,
    Endline,
    Assign,
    Plus,
    Minus,
    Multiplication,
    Division,
    Name(Box<str>),
    Number(Box<str>),
}

impl Token {

    pub fn size(&self) -> usize {
        match self {
            Token::If => 2,
            Token::Space => 1,
            Token::Endline => 1,
            Token::Assign => 1,
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Multiplication => 1,
            Token::Division => 1,
            Token::Name(s) => s.len(),
            Token::Number(n) => n.len(),
        }
    }
}

pub fn create_token_map() -> HashMap<&'static str,Token> {
    [
        ("if",Token::If),
        (" ",Token::Space),
        ("\n",Token::Endline),
        ("=",Token::Assign),
        ("+",Token::Plus),
        ("-",Token::Minus),
        ("*",Token::Multiplication),
        ("/",Token::Division),
    ] .into_iter()
    .collect()
}

pub fn pattern(input: &str,start:impl Fn(char) -> bool,rest:impl Fn(char) -> bool) -> Option<usize> {
    if start(input.chars().next()?) {
        return Some(input[1..].find(|c| !rest(c)).unwrap_or(input.len() - 1) + 1);
    }
    None
}


#[allow(clippy::manual_map)]
pub fn tokens(mut input: &str) -> Vec<Token> {
    let mut res = vec![];
    let token_map = create_token_map();

    while !input.is_empty() {
        let token = if let Some(token) = token_map.iter()
            .filter(|&(k,_)| input.starts_with(*k))
            .map(|(_,v)| v).next() {
            Some(token.clone())
        } else if let Some(index) =  pattern(input,|c| c.is_ascii_alphabetic(),|c| c.is_ascii_alphanumeric()) {
            Some(Token::Name(input[..index].into()))
        } else if let Some(index) =  pattern(input,|c| c.is_ascii_digit(),|c| c.is_ascii_digit()) {
            Some(Token::Number(input[..index].into()))
        } else {
            None
        };
        if let Some(token) = token {
            input = &input[token.size()..];
            res.push(token);
        } else {
            panic!("Токен не распознан {}",input);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn simple_test() {
        let input = "+ -f";
        let a = tokens(input);
        assert_eq!(a,vec![ 
            Token::Plus,
            Token::Space,
            Token::Minus,
            Token::Name("f".into())
        ]);

    }

    #[test]
    fn should_not_find_if() {
        let input = "sniff";
        let a = tokens(input);
        assert_eq!(a,vec![ 
            Token::Name("sniff".into())
        ]);
    }

    #[test]
    fn should_find_minus_and_number() {
        let input = "-55";
        let a = tokens(input);
        assert_eq!(a,vec![ 
            Token::Minus,
            Token::Number("55".into())
        ]);
    }

    #[test]
    fn should_work_with_leading_zeros() {
        let input = "+05+";
        let a = tokens(input);
        assert_eq!(a,vec![ 
            Token::Plus,
            Token::Number("05".into()),
            Token::Plus,
        ]);
    }

}
