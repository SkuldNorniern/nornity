use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct PythonLexer;

impl LanguageLexer for PythonLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "def", "class", "if", "elif", "else", "for", "while", "try", "except", "finally",
            "with", "as", "import", "from", "return", "break", "continue", "pass", "raise",
            "yield", "lambda", "in", "is", "not", "and", "or", "global", "nonlocal", "del",
            "assert",
        ]
    }
    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let types = [
            "str",
            "int",
            "float",
            "bool",
            "list",
            "dict",
            "tuple",
            "set",
            "bytes",
            "bytearray",
            "complex",
            "NoneType",
        ];
        let literals = ["True", "False", "None"];
        let builtins = [
            "print", "len", "range", "open", "close", "read", "write", "append", "extend", "pop",
            "self", "super",
        ];
        let mut lexer = BaseLexer::new(input);

        while lexer.position < lexer.input.len() {
            let ch = lexer.current_char().unwrap_or('\0');
            match ch {
                ' ' | '\t' => {
                    lexer.consume_whitespace();
                }
                '\n' | '\r' => {
                    lexer.consume_newlines();
                }
                '#' => {
                    lexer.consume_single_line_comment("#");
                }
                '"' | '\'' => {
                    // Triple-quoted string
                    if lexer.peek_char(1) == Some(ch) && lexer.peek_char(2) == Some(ch) {
                        let start = lexer.position;
                        let mut value = String::new();
                        value.push(ch);
                        value.push(ch);
                        value.push(ch);
                        lexer.advance();
                        lexer.advance();
                        lexer.advance();
                        while lexer.position < lexer.input.len() {
                            let c = lexer.current_char().unwrap();
                            value.push(c);
                            if c == ch
                                && lexer.peek_char(1) == Some(ch)
                                && lexer.peek_char(2) == Some(ch)
                            {
                                value.push(ch);
                                value.push(ch);
                                lexer.advance();
                                lexer.advance();
                                lexer.advance();
                                break;
                            } else {
                                lexer.advance();
                            }
                        }
                        lexer.add_token(TokenType::String, value, start, lexer.position);
                    } else {
                        lexer.consume_string(ch);
                    }
                }
                '0'..='9' => {
                    lexer.consume_number();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = lexer.position;
                    let mut value = String::new();
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    if keywords.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                    } else if types.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Type, value, start, lexer.position);
                    } else if literals.contains(&value.as_str()) {
                        lexer.add_token(TokenType::String, value, start, lexer.position);
                    } else if builtins.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Macro, value, start, lexer.position);
                    } else {
                        lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                    }
                }
                '<' | '>' | '=' | '+' | '-' | '*' | '/' | '&' | '|' | '!' | '?' | ':' | ';'
                | ',' | '.' | '(' | ')' | '[' | ']' | '{' | '}' => {
                    lexer.add_operator(ch.to_string());
                }
                _ => {
                    lexer.add_other(ch.to_string());
                }
            }
        }
        lexer.get_tokens()
    }
}