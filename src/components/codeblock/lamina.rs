use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct LaminaLexer;

impl LanguageLexer for LaminaLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            // Function and control flow keywords
            "fn", "ret", "br", "call", "if", "else", "loop", "while", "for",
            
            // Memory operations
            "alloc", "dealloc", "load", "store", "stack", "heap",
            
            // Type operations
            "getfield", "getelem", "ptr",
            
            // Binary operators
            "add", "sub", "mul", "div", "mod", "and", "or", "xor", "shl", "shr",
            "eq", "ne", "lt", "le", "gt", "ge",
            
            // Unary operators
            "not", "neg",
            
            // Type keywords
            "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128",
            "f32", "f64", "bool", "ptr", "struct", "array",
            
            // Annotations
            "export", "import",
            
            // Special operations
            "print", "global", "type",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
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
                    // Comments in Lamina IR
                    lexer.consume_single_line_comment("#");
                }
                '"' => {
                    // String literals
                    lexer.consume_string('"');
                }
                '@' => {
                    // Global symbols and function names
                    let start = lexer.position;
                    let mut value = String::from("@");
                    lexer.advance();
                    
                    // Consume identifier after @
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' || c == '-' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    
                    lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                }
                '%' => {
                    // Local variables and SSA values
                    let start = lexer.position;
                    let mut value = String::from("%");
                    lexer.advance();
                    
                    // Consume identifier after %
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    
                    lexer.add_token(TokenType::Variable, value, start, lexer.position);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Identifiers, keywords, and operators
                    let start = lexer.position;
                    let mut value = String::new();
                    
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' || c == '.' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    
                    // Check if it's a keyword
                    if keywords.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                    } else {
                        lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                    }
                }
                '0'..='9' => {
                    // Numbers
                    lexer.consume_number();
                }
                '<' | '>' | '=' | '+' | '-' | '*' | '&' | '|' | '!' | '?' | ':' | ';' | ','
                | '.' | '(' | ')' | '[' | ']' | '{' | '}' => {
                    // Operators and punctuation
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamina_lexer_basic() {
        let lexer = LaminaLexer;
        let input = r#"# Type declaration
type @Vec2 = struct { x: f32, y: f32 }

# Global value
global @message: [5 x i8] = "hello"

# Function with annotations
@export
fn @add(i32 %a, i32 %b) -> i32 {
  entry:
    %sum = add.i32 %a, %b
    ret.i32 %sum
}"#;

        let tokens = lexer.lex(input);
        
        // Verify we have tokens
        assert!(!tokens.is_empty());
        
        // Check for specific token types
        let has_comment = tokens.iter().any(|t| t.token_type == TokenType::Comment);
        let has_keyword = tokens.iter().any(|t| t.token_type == TokenType::Keyword);
        let has_identifier = tokens.iter().any(|t| t.token_type == TokenType::Identifier);
        let has_variable = tokens.iter().any(|t| t.token_type == TokenType::Variable);
        let has_string = tokens.iter().any(|t| t.token_type == TokenType::String);
        
        assert!(has_comment, "Should have comments");
        assert!(has_keyword, "Should have keywords");
        assert!(has_identifier, "Should have identifiers");
        assert!(has_variable, "Should have variables");
        assert!(has_string, "Should have strings");
    }

    #[test]
    fn test_lamina_lexer_keywords() {
        let lexer = LaminaLexer;
        let input = "fn ret br call if else loop while for alloc dealloc load store";
        
        let tokens = lexer.lex(input);
        let keywords: Vec<&str> = tokens.iter()
            .filter(|t| t.token_type == TokenType::Keyword)
            .map(|t| t.value.as_str())
            .collect();
        
        assert!(keywords.contains(&"fn"));
        assert!(keywords.contains(&"ret"));
        assert!(keywords.contains(&"br"));
        assert!(keywords.contains(&"call"));
        assert!(keywords.contains(&"alloc"));
        assert!(keywords.contains(&"dealloc"));
    }

    #[test]
    fn test_lamina_lexer_symbols() {
        let lexer = LaminaLexer;
        let input = "@global_function %local_var";
        
        let tokens = lexer.lex(input);
        let identifiers: Vec<&str> = tokens.iter()
            .filter(|t| t.token_type == TokenType::Identifier)
            .map(|t| t.value.as_str())
            .collect();
        let variables: Vec<&str> = tokens.iter()
            .filter(|t| t.token_type == TokenType::Variable)
            .map(|t| t.value.as_str())
            .collect();
        
        assert!(identifiers.contains(&"@global_function"));
        assert!(variables.contains(&"%local_var"));
    }
}
