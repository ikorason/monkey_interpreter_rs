use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        l.read_char();

        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() { // if we reach the end of the input
            self.ch = None; // set ch to NUL
        } else {
            self.ch = self.input.chars().nth(self.read_position); // set ch to the character at read_position
        }
        self.position = self.read_position; // move position forward
        self.read_position += 1; // increment 'read_position' to track the next character
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch {
            if !Lexer::isLetter(ch) {
                break;
            }
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn isLetter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some('=') => Token { token_type: TokenType::ASSIGN, literal: self.ch.unwrap().to_string() },
            Some(';') => Token { token_type: TokenType::SEMICOLON, literal: self.ch.unwrap().to_string() },
            Some('(') => Token { token_type: TokenType::LPAREN, literal: self.ch.unwrap().to_string() },
            Some(')') => Token { token_type: TokenType::RPAREN, literal: self.ch.unwrap().to_string() },
            Some(',') => Token { token_type: TokenType::COMMA, literal: self.ch.unwrap().to_string() },
            Some('+') => Token { token_type: TokenType::PLUS, literal: self.ch.unwrap().to_string() },
            Some('{') => Token { token_type: TokenType::LBRACE, literal: self.ch.unwrap().to_string() },
            Some('}') => Token { token_type: TokenType::RBRACE, literal: self.ch.unwrap().to_string() },
            Some(_) => Token { token_type: TokenType::EOF, literal: "".to_string() },
            None => Token { token_type: TokenType::EOF, literal: "".to_string() },
        };

        self.read_char();

        tok
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let current_char = self.ch.take();
        self.read_char();
        current_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_1() {
        let input = "=+(){},;";

        let tests = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(tok.token_type, *expected_type, "tests[{}] - token type wrong. expected={:?}, got={:?}", i, expected_type, tok.token_type);
            assert_eq!(tok.literal, *expected_literal, "tests[{}] - literal wrong. expected={}, got={}", i, expected_literal, tok.literal);
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        ";

        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(tok.token_type, *expected_type, "tests[{}] - token type wrong. expected={:?}, got={:?}", i, expected_type, tok.token_type);
            assert_eq!(tok.literal, *expected_literal, "tests[{}] - literal wrong. expected={}, got={}", i, expected_literal, tok.literal);
        }
    }
}