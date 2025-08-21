use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct JavaScriptLexer;

impl LanguageLexer for JavaScriptLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "function",
            "const",
            "let",
            "var",
            "if",
            "else",
            "for",
            "while",
            "do",
            "switch",
            "case",
            "default",
            "try",
            "catch",
            "finally",
            "throw",
            "return",
            "break",
            "continue",
            "class",
            "extends",
            "super",
            "new",
            "this",
            "typeof",
            "instanceof",
            "delete",
            "void",
            "with",
            "debugger",
            "export",
            "import",
            "static",
            "async",
            "await",
            "yield",
            "get",
            "set",
            "of",
            "in",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let types = [
            "String", "Number", "Boolean", "Array", "Object", "Function", "Promise", "Date",
            "RegExp", "Error", "Map", "Set", "WeakMap", "WeakSet", "Symbol", "BigInt",
        ];
        let literals = ["true", "false", "null", "undefined", "NaN", "Infinity"];
        let builtins = [
            "console",
            "document",
            "window",
            "Math",
            "JSON",
            "parseInt",
            "parseFloat",
            "isNaN",
            "isFinite",
            "encodeURI",
            "decodeURI",
            "setTimeout",
            "setInterval",
            "clearTimeout",
            "clearInterval",
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
                '"' | '\'' => {
                    lexer.consume_string(ch);
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