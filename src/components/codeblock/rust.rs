use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct RustLexer;

impl LanguageLexer for RustLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "fn", "let", "mut", "pub", "impl", "struct", "enum", "use", "mod", "async", "await",
            "match", "if", "else", "for", "while", "loop", "return", "break", "continue", "const",
            "static", "trait", "where", "as", "in", "ref", "move", "unsafe", "extern", "Self",
            "super", "crate", "self",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let types = [
            "String", "Vec", "Result", "Option", "Box", "DateTime", "Utc", "str", "u8", "u16",
            "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
            "f64", "bool", "char", "Ok", "Err", "Some", "None", "Arc", "Mutex", "RwLock",
            "HashMap", "BTreeMap", "HashSet", "BTreeSet", "LinkedList", "VecDeque", "BinaryHeap",
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
                '/' => {
                    if lexer.peek_char(1) == Some('/') {
                        lexer.consume_single_line_comment("//");
                    } else if lexer.peek_char(1) == Some('*') {
                        lexer.consume_multi_line_comment("/*", "*/");
                    } else {
                        lexer.add_operator(ch.to_string());
                    }
                }
                '"' => {
                    lexer.consume_string('"');
                }
                '#' => {
                    let start = lexer.position;
                    let mut value = String::from("#");
                    lexer.advance();
                    if lexer.current_char() == Some('[') {
                        value.push('[');
                        lexer.advance();
                    }
                    lexer.add_token(TokenType::Operator, value, start, lexer.position);
                }
                // Macro: identifier followed by !
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
                    // Macro
                    if lexer.current_char() == Some('!') {
                        value.push('!');
                        lexer.advance();
                        lexer.add_token(TokenType::Macro, value, start, lexer.position);
                    // Lifetime
                    } else if value.starts_with("'") && value.len() > 1 {
                        lexer.add_token(TokenType::Lifetime, value, start, lexer.position);
                    // Type
                    } else if types.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Type, value, start, lexer.position);
                    // Keyword
                    } else if keywords.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                    // Identifier
                    } else {
                        lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                    }
                }
                // Lifetime: 'a, 'static, etc.
                '\'' => {
                    let start = lexer.position;
                    let mut value = String::new();
                    value.push('\'');
                    lexer.advance();
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    lexer.add_token(TokenType::Lifetime, value, start, lexer.position);
                }
                '0'..='9' => {
                    lexer.consume_number();
                }
                '<' | '>' | '=' | '+' | '-' | '*' | '&' | '|' | '!' | '?' | ':' | ';' | ','
                | '.' | '(' | ')' | '[' | ']' | '{' | '}' => {
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