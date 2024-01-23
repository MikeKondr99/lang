
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

pub fn tokens(mut input: &str) -> Vec<Token> {
    let mut res = vec![];

    while !input.is_empty() {
        let token = if input.starts_with("if") {
            Some(Token::If)
        } else if input.starts_with(' ') {
            Some(Token::Space)
        } else if input.starts_with('\n') {
            Some(Token::Endline)
        } else if input.starts_with('=') {
            Some(Token::Assign)
        } else if input.starts_with('+') {
            Some(Token::Plus)
        } else if input.starts_with('-') {
            Some(Token::Minus)
        } else if input.starts_with('*') {
            Some(Token::Multiplication)
        } else if input.starts_with('/') {
            Some(Token::Division)
        } else if input
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
        {
            // Name
            let index = input.find(|x: char| !x.is_ascii_alphanumeric());
            let index = index.unwrap_or(input.len());
            Some(Token::Name(input[..index].into()))
        } else if input.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            // Number
            let index = input.find(|x: char| !x.is_ascii_digit());
            let index = index.unwrap_or(input.len());
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


pub enum Instruction {
    Assign {
        left: Box<str>
    }

}

pub fn instructions(tokens:&[Token]) {
    let mut tokens: Vec<Token> = tokens.to_vec();
    tokens.retain(|t| !matches!(t, Token::Endline | Token::Space));



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
