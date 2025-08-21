use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct CSSLexer;

impl LanguageLexer for CSSLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "var",
            "calc",
            "min",
            "max",
            "clamp",
            "rgb",
            "hsl",
            "hsla",
            "linear-gradient",
            "inherit",
            "initial",
            "unset",
            "revert",
            "important",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let properties = [
            "color",
            "background",
            "font-size",
            "margin",
            "padding",
            "border",
            "width",
            "height",
            "display",
            "position",
            "top",
            "left",
            "right",
            "bottom",
            "text-align",
            "font-weight",
            "line-height",
            "opacity",
            "font-family",
            "font-style",
            "text-decoration",
            "text-transform",
            "box-shadow",
            "border-radius",
            "z-index",
            "overflow",
            "cursor",
            "transition",
            "animation",
            "transform",
            "filter",
            "backdrop-filter",
        ];
        let property_prefixes = [
            "overflow",
            "border",
            "margin",
            "padding",
            "font",
            "text",
            "background",
            "flex",
            "grid",
            "animation",
            "transition",
            "transform",
            "box",
        ];
        let values = [
            "solid",
            "dashed",
            "dotted",
            "auto",
            "relative",
            "absolute",
            "fixed",
            "block",
            "inline",
            "flex",
            "grid",
            "none",
            "hidden",
            "visible",
            "transparent",
            "deg",
            "px",
            "rem",
            "em",
            "vh",
            "vw",
            "rgba",
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
                    if lexer.peek_char(1) == Some('*') {
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
                'a'..='z' | 'A'..='Z' | '_' | '-' => {
                    let start = lexer.position;
                    let mut value = String::new();
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    let token_type = if keywords.contains(&value.as_str()) {
                        TokenType::Keyword
                    } else if properties.contains(&value.as_str())
                        || property_prefixes
                            .iter()
                            .any(|prefix| value.starts_with(prefix))
                    {
                        TokenType::Type
                    } else if values.contains(&value.as_str()) {
                        TokenType::String
                    } else {
                        TokenType::Identifier
                    };
                    lexer.add_token(token_type, value, start, lexer.position);
                }
                ':' | ';' | ',' | '.' | '(' | ')' | '{' | '}' | '[' | ']' | '=' | '+' | '*'
                | '>' | '<' | '!' => {
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