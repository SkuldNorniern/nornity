use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct CppLexer;

impl LanguageLexer for CppLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            // C++ specific keywords
            "class", "namespace", "template", "typename", "using", "typedef", "auto", "decltype",
            "nullptr", "override", "final", "virtual", "explicit", "mutable", "friend", "inline",
            "constexpr", "consteval", "constinit", "static_assert", "noexcept", "alignas", "alignof",
            "thread_local", "export", "extern", "register", "volatile", "wchar_t", "char16_t", "char32_t",
            
            // Control flow (inherited from C)
            "if", "else", "for", "while", "do", "switch", "case", "default", "break", "continue", "return", "goto",
            
            // Data types (inherited from C)
            "int", "char", "float", "double", "long", "short", "unsigned", "signed", "void", "const", "bool",
            
            // Storage classes (inherited from C)
            "static", "extern", "typedef",
            
            // Other keywords
            "sizeof", "struct", "union", "enum", "restrict", "_Bool", "_Complex", "_Imaginary",
            "public", "private", "protected", "operator", "new", "delete", "this", "super",
            "try", "catch", "throw", "typeid", "dynamic_cast", "static_cast", "reinterpret_cast", "const_cast",
            "and", "and_eq", "bitand", "bitor", "compl", "not", "not_eq", "or", "or_eq", "xor", "xor_eq",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let keywords = self.get_keywords();
        let types = [
            // C++ specific types
            "string", "vector", "map", "set", "unordered_map", "unordered_set", "list", "deque", "queue", "stack",
            "unique_ptr", "shared_ptr", "weak_ptr", "function", "tuple", "pair", "optional", "variant", "any",
            "chrono", "filesystem", "regex", "random", "thread", "mutex", "condition_variable", "future", "promise",
            "array", "bitset", "multimap", "multiset", "priority_queue", "forward_list", "unordered_multimap",
            "unordered_multiset", "valarray", "complex", "ratio", "atomic", "memory_order", "launch",
            
            // C types
            "int", "char", "float", "double", "long", "short", "unsigned", "signed", "void", "size_t", "ptrdiff_t",
            "uint8_t", "uint16_t", "uint32_t", "uint64_t", "int8_t", "int16_t", "int32_t", "int64_t",
            "wchar_t", "char16_t", "char32_t",
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
                '\'' => {
                    lexer.consume_string('\'');
                }
                '#' => {
                    // Preprocessor directives
                    let start = lexer.position;
                    let mut value = String::from("#");
                    lexer.advance();
                    
                    // Skip whitespace after #
                    while let Some(c) = lexer.current_char() {
                        if c.is_whitespace() {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    
                    // Read the directive (include, define, ifdef, etc.)
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    
                    lexer.add_token(TokenType::Macro, value, start, lexer.position);
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
                    
                    // Check if it's a type (including user-defined types)
                    if types.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Type, value, start, lexer.position);
                    } else if keywords.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                    } else {
                        lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                    }
                }
                '+' | '-' | '*' | '%' | '=' | '!' | '&' | '|' | '^' | '~' | '?' => {
                    // Handle compound operators
                    let start = lexer.position;
                    let mut value = String::from(ch);
                    lexer.advance();
                    
                    // Check for compound operators
                    if let Some(next_ch) = lexer.current_char() {
                        let compound = format!("{}{}", ch, next_ch);
                        match compound.as_str() {
                            "++" | "--" | "==" | "!=" | "&&" | "||" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" => {
                                value.push(next_ch);
                                lexer.advance();
                            }
                            _ => {}
                        }
                    }
                    
                    lexer.add_token(TokenType::Operator, value, start, lexer.position);
                }
                '<' => {
                    // Handle template syntax and stream operators
                    let start = lexer.position;
                    let mut value = String::from(ch);
                    lexer.advance();
                    
                    // Check for stream operators and compound operators
                    if let Some(next_ch) = lexer.current_char() {
                        if next_ch == '<' {
                            value.push(next_ch);
                            lexer.advance();
                        } else if next_ch == '=' {
                            value.push(next_ch);
                            lexer.advance();
                        }
                    }
                    
                    lexer.add_token(TokenType::Operator, value, start, lexer.position);
                }
                '>' => {
                    // Handle template syntax and stream operators
                    let start = lexer.position;
                    let mut value = String::from(ch);
                    lexer.advance();
                    
                    // Check for stream operators and compound operators
                    if let Some(next_ch) = lexer.current_char() {
                        if next_ch == '>' {
                            value.push(next_ch);
                            lexer.advance();
                        } else if next_ch == '=' {
                            value.push(next_ch);
                            lexer.advance();
                        }
                    }
                    
                    lexer.add_token(TokenType::Operator, value, start, lexer.position);
                }
                ':' => {
                    // Handle scope resolution operator
                    let start = lexer.position;
                    let mut value = String::from(ch);
                    lexer.advance();
                    
                    if let Some(next_ch) = lexer.current_char() {
                        if next_ch == ':' {
                            value.push(next_ch);
                            lexer.advance();
                        }
                    }
                    
                    lexer.add_token(TokenType::Operator, value, start, lexer.position);
                }
                '(' | ')' | '[' | ']' | '{' | '}' | ';' | ',' | '.' => {
                    lexer.add_token(TokenType::Other, ch.to_string(), lexer.position, lexer.position + 1);
                    lexer.advance();
                }
                _ => {
                    lexer.add_token(TokenType::Other, ch.to_string(), lexer.position, lexer.position + 1);
                    lexer.advance();
                }
            }
        }
        
        lexer.get_tokens()
    }
}
