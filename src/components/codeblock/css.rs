use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct CSSLexer;

impl LanguageLexer for CSSLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "var", "calc", "min", "max", "clamp", "rgb", "hsl", "hsla", "linear-gradient",
            "radial-gradient", "conic-gradient", "repeating-linear-gradient", "repeating-radial-gradient",
            "inherit", "initial", "unset", "revert", "important", "not", "and", "or", "only",
            "screen", "print", "media", "supports", "keyframes", "from", "to", "animation-name",
            "animation-duration", "animation-timing-function", "animation-delay", "animation-iteration-count",
            "animation-direction", "animation-fill-mode", "animation-play-state",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let properties = [
            "color", "background", "font-size", "margin", "padding", "border", "width", "height",
            "display", "position", "top", "left", "right", "bottom", "text-align", "font-weight",
            "line-height", "opacity", "font-family", "font-style", "text-decoration", "text-transform",
            "box-shadow", "border-radius", "z-index", "overflow", "cursor", "transition", "animation",
            "transform", "filter", "backdrop-filter", "flex-direction", "justify-content", "align-items",
            "flex-wrap", "grid-template-columns", "grid-template-rows", "grid-gap", "gap", "flex-basis",
            "flex-grow", "flex-shrink", "order", "align-self", "justify-self", "grid-column",
            "grid-row", "min-width", "max-width", "min-height", "max-height", "box-sizing",
            "outline", "outline-offset", "resize", "vertical-align", "white-space", "word-break",
            "word-wrap", "overflow-wrap", "text-overflow", "user-select", "pointer-events",
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
            "solid", "dashed", "dotted", "auto", "relative", "absolute", "fixed", "block", "inline",
            "flex", "grid", "none", "hidden", "visible", "transparent", "deg", "px", "rem", "em",
            "vh", "vw", "rgba", "center", "left", "right", "top", "bottom", "middle", "bold",
            "normal", "italic", "underline", "uppercase", "lowercase", "capitalize", "nowrap",
            "pre-wrap", "pre-line", "break-word", "break-all", "keep-all", "break-spaces",
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