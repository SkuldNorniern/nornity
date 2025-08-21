use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct HTMLLexer;

impl LanguageLexer for HTMLLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "html",
            "head",
            "body",
            "title",
            "meta",
            "link",
            "script",
            "style",
            "div",
            "span",
            "p",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "ul",
            "ol",
            "li",
            "table",
            "tr",
            "td",
            "th",
            "form",
            "input",
            "button",
            "textarea",
            "select",
            "img",
            "a",
            "br",
            "hr",
            "pre",
            "code",
            "blockquote",
            "article",
            "section",
            "header",
            "footer",
            "nav",
            "aside",
            "main",
            "figure",
            "figcaption",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let mut lexer = BaseLexer::new(input);
        let keywords = self.get_keywords();

        while lexer.position < lexer.input.len() {
            let ch = lexer.current_char().unwrap_or('\0');

            match ch {
                ' ' | '\t' => {
                    lexer.consume_whitespace();
                }
                '\n' | '\r' => {
                    lexer.consume_newlines();
                }
                '<' => {
                    let start = lexer.position;
                    let mut value = String::from("<");
                    lexer.advance();

                    // Check for DOCTYPE
                    if lexer.peek_char(0) == Some('!') {
                        value.push('!');
                        lexer.advance();

                        while let Some(c) = lexer.current_char() {
                            if c == '>' {
                                value.push(c);
                                lexer.advance();
                                break;
                            }
                            value.push(c);
                            lexer.advance();
                        }
                        lexer.add_token(TokenType::Comment, value, start, lexer.position);
                    } else {
                        // Regular tag
                        while let Some(c) = lexer.current_char() {
                            if c == '>' || c == ' ' || c == '\t' || c == '\n' {
                                break;
                            }
                            value.push(c);
                            lexer.advance();
                        }

                        // Check if it's a closing tag
                        if value.starts_with("</") {
                            lexer.add_token(TokenType::Operator, value, start, lexer.position);
                        } else {
                            lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                        }
                    }
                }
                '>' => {
                    lexer.add_operator(ch.to_string());
                }
                '/' => {
                    lexer.add_operator(ch.to_string());
                }
                '=' => {
                    lexer.add_operator(ch.to_string());
                }
                '"' | '\'' => {
                    lexer.consume_string(ch);
                }
                '0'..='9' => {
                    lexer.consume_number();
                }
                'a'..='z' | 'A'..='Z' | '_' | '-' => {
                    lexer.consume_identifier_with_hyphens(keywords);
                }
                _ => {
                    lexer.add_other(ch.to_string());
                }
            }
        }

        lexer.get_tokens()
    }
}