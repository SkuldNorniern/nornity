use log::debug;

/// Code block component for syntax highlighting and HTML generation
pub struct CodeBlock {
    pub language: String,
    pub content: String,
}

/// Token types for lexing
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    String,
    Comment,
    Number,
    Operator,
    Identifier,
    Type,
    Macro,
    Lifetime,
    Whitespace,
    Newline,
    Other,
}

/// Token structure
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

/// Language-specific lexer trait for extensibility
pub trait LanguageLexer {
    fn lex(&self, input: &str) -> Vec<Token>;
    fn get_keywords(&self) -> &[&str];
}

/// Base lexer with common functionality
pub struct BaseLexer {
    input: Vec<char>,
    position: usize,
    tokens: Vec<Token>,
}

impl BaseLexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            tokens: Vec::new(),
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn peek_char(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn add_token(&mut self, token_type: TokenType, value: String, start: usize, end: usize) {
        self.tokens.push(Token {
            token_type,
            value,
            start,
            end,
        });
    }

    pub fn consume_whitespace(&mut self) -> Option<String> {
        let start = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c == ' ' || c == '\t' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if !value.is_empty() {
            self.add_token(TokenType::Whitespace, value.clone(), start, self.position);
            Some(value)
        } else {
            None
        }
    }

    pub fn consume_newlines(&mut self) -> Option<String> {
        let start = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c == '\n' || c == '\r' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if !value.is_empty() {
            self.add_token(TokenType::Newline, value.clone(), start, self.position);
            Some(value)
        } else {
            None
        }
    }

    pub fn consume_identifier(&mut self, keywords: &[&str]) -> Option<String> {
        let start = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if !value.is_empty() {
            let token_type = if keywords.contains(&value.as_str()) {
                TokenType::Keyword
            } else {
                TokenType::Identifier
            };

            self.add_token(token_type, value.clone(), start, self.position);
            Some(value)
        } else {
            None
        }
    }

    pub fn consume_identifier_with_hyphens(&mut self, keywords: &[&str]) -> Option<String> {
        let start = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if !value.is_empty() {
            let token_type = if keywords.contains(&value.as_str()) {
                TokenType::Keyword
            } else {
                TokenType::Identifier
            };

            self.add_token(token_type, value.clone(), start, self.position);
            Some(value)
        } else {
            None
        }
    }

    pub fn consume_number(&mut self) -> Option<String> {
        let start = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() || c == '.' || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if !value.is_empty() {
            self.add_token(TokenType::Number, value.clone(), start, self.position);
            Some(value)
        } else {
            None
        }
    }

    pub fn consume_string(&mut self, quote: char) -> Option<String> {
        let start = self.position;
        let mut value = String::from(quote);
        self.advance();

        while let Some(c) = self.current_char() {
            value.push(c);
            self.advance();
            if c == quote {
                break;
            }
        }

        self.add_token(TokenType::String, value.clone(), start, self.position);
        Some(value)
    }

    pub fn consume_single_line_comment(&mut self, prefix: &str) -> Option<String> {
        let start = self.position;
        let mut value = String::from(prefix);

        for _ in 0..prefix.len() {
            self.advance();
        }

        while let Some(c) = self.current_char() {
            if c == '\n' || c == '\r' {
                break;
            }
            value.push(c);
            self.advance();
        }

        self.add_token(TokenType::Comment, value.clone(), start, self.position);
        Some(value)
    }

    pub fn consume_multi_line_comment(
        &mut self,
        start_prefix: &str,
        end_suffix: &str,
    ) -> Option<String> {
        let start = self.position;
        let mut value = String::from(start_prefix);

        for _ in 0..start_prefix.len() {
            self.advance();
        }

        while let Some(c) = self.current_char() {
            value.push(c);
            self.advance();

            // Check for end suffix
            if value.ends_with(end_suffix) {
                break;
            }
        }

        self.add_token(TokenType::Comment, value.clone(), start, self.position);
        Some(value)
    }

    pub fn add_operator(&mut self, value: String) {
        let start = self.position;
        self.add_token(TokenType::Operator, value, start, self.position + 1);
        self.advance();
    }

    pub fn add_other(&mut self, value: String) {
        let start = self.position;
        self.add_token(TokenType::Other, value, start, self.position + 1);
        self.advance();
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

/// Rust language lexer
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
            "f64", "bool", "char", "Ok", "Err", "Some", "None",
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

/// JavaScript language lexer
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

/// Python language lexer
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

/// CSS language lexer
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
                    } else if properties.contains(&value.as_str()) || property_prefixes
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

/// Bash language lexer
pub struct BashLexer;

impl LanguageLexer for BashLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            "while", "do", "done", "if", "then", "fi", "for", "in", "case", "esac", "echo", "find",
            "grep", "ls", "cd", "mkdir", "rm", "cp", "mv", "cat", "head", "tail", "cut", "local",
            "read", "function", "return", "exit",
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
                '#' => {
                    lexer.consume_single_line_comment("#");
                }
                '$' => {
                    let start = lexer.position;
                    let mut value = String::from("$");
                    lexer.advance();

                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || c == '_' || c == '{' || c == '}' {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                }
                '"' | '\'' => {
                    lexer.consume_string(ch);
                }
                '0'..='9' => {
                    lexer.consume_number();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    lexer.consume_identifier(keywords);
                }
                _ => {
                    lexer.add_other(ch.to_string());
                }
            }
        }

        lexer.get_tokens()
    }
}

/// HTML language lexer
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

/// Lexer factory for creating language-specific lexers
pub struct LexerFactory;

impl LexerFactory {
    pub fn create_lexer(language: &str) -> Box<dyn LanguageLexer> {
        match language.to_lowercase().as_str() {
            "rust" | "rs" => Box::new(RustLexer),
            "javascript" | "js" => Box::new(JavaScriptLexer),
            "python" | "py" => Box::new(PythonLexer),
            "css" => Box::new(CSSLexer),
            "bash" | "shell" | "sh" => Box::new(BashLexer),
            "html" => Box::new(HTMLLexer),
            _ => Box::new(JavaScriptLexer), // Default fallback
        }
    }
}

impl CodeBlock {
    /// Create a new code block
    pub fn new(language: String, content: String) -> Self {
        Self { language, content }
    }

    /// Apply syntax highlighting to the code content using lexing
    pub fn highlight(&self) -> String {
        debug!(
            "Starting highlight for language: '{}', content length: {}",
            self.language,
            self.content.len()
        );

        // Decode HTML entities first, but be careful not to double-decode
        let decoded_content = self
            .content
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&nbsp;", " ");
        // Note: We don't decode &quot; and &#39; here as they might be intentional in code examples

        // Create language-specific lexer
        let lexer = LexerFactory::create_lexer(&self.language);
        let tokens = lexer.lex(&decoded_content);

        // Convert tokens to HTML with proper escaping
        let mut result = String::new();
        for token in tokens {
            let html_class = match token.token_type {
                TokenType::Keyword => "keyword",
                TokenType::String => "string",
                TokenType::Comment => "comment",
                TokenType::Number => "number",
                TokenType::Operator => "operator",
                TokenType::Identifier => "identifier",
                TokenType::Type => "type",
                TokenType::Macro => "macro",
                TokenType::Lifetime => "lifetime",
                TokenType::Whitespace => "",
                TokenType::Newline => "",
                TokenType::Other => "",
            };

            if html_class.is_empty() {
                // Escape HTML entities in the output, but preserve existing entities
                let escaped_value = escape_html_entities(&token.value);
                result.push_str(&escaped_value);
            } else {
                // Escape HTML entities in the output, but preserve existing entities
                let escaped_value = escape_html_entities(&token.value);
                result.push_str(&format!(
                    "<span class=\"{}\">{}</span>",
                    html_class, escaped_value
                ));
            }
        }

        debug!("After highlighting: {:?}", result);
        result
    }
}

/// Escape HTML entities while preserving existing ones
fn escape_html_entities(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        let ch = chars[i];
        match ch {
            '&' => {
                // Check if this is already an HTML entity
                if i + 1 < chars.len() && chars[i + 1] == '#' {
                    // Already an HTML entity, preserve it
                    result.push(ch);
                } else if i + 3 < chars.len()
                    && chars[i + 1] == 'l'
                    && chars[i + 2] == 't'
                    && chars[i + 3] == ';'
                {
                    // Already &lt;
                    result.push(ch);
                } else if i + 3 < chars.len()
                    && chars[i + 1] == 'g'
                    && chars[i + 2] == 't'
                    && chars[i + 3] == ';'
                {
                    // Already &gt;
                    result.push(ch);
                } else if i + 4 < chars.len()
                    && chars[i + 1] == 'a'
                    && chars[i + 2] == 'm'
                    && chars[i + 3] == 'p'
                    && chars[i + 4] == ';'
                {
                    // Already &amp;
                    result.push(ch);
                } else if i + 5 < chars.len()
                    && chars[i + 1] == 'q'
                    && chars[i + 2] == 'u'
                    && chars[i + 3] == 'o'
                    && chars[i + 4] == 't'
                    && chars[i + 5] == ';'
                {
                    // Already &quot;
                    result.push(ch);
                } else if i + 4 < chars.len()
                    && chars[i + 1] == '#'
                    && chars[i + 2] == '3'
                    && chars[i + 3] == '9'
                    && chars[i + 4] == ';'
                {
                    // Already &#39;
                    result.push(ch);
                } else {
                    // Regular &, escape it
                    result.push_str("&amp;");
                }
            }
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            _ => result.push(ch),
        }
        i += 1;
    }

    result
}

/// Simple HTML parser for code block extraction without regex
pub struct HTMLParser {
    content: Vec<char>,
    position: usize,
}

impl HTMLParser {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.chars().collect(),
            position: 0,
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.content.get(self.position).copied()
    }

    pub fn peek_char(&self, offset: usize) -> Option<char> {
        self.content.get(self.position + offset).copied()
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn extract_code_blocks(&mut self) -> Vec<(String, String)> {
    let mut code_blocks = Vec::new();

        while self.position < self.content.len() {
            // Look for <pre><code
            if self.current_char() == Some('<')
                && self.peek_char(1) == Some('p')
                && self.peek_char(2) == Some('r')
                && self.peek_char(3) == Some('e')
                && self.peek_char(4) == Some('>')
            {
                // Skip <pre>
                for _ in 0..5 {
                    self.advance();
                }

                // Look for <code
                if self.current_char() == Some('<')
                    && self.peek_char(1) == Some('c')
                    && self.peek_char(2) == Some('o')
                    && self.peek_char(3) == Some('d')
                    && self.peek_char(4) == Some('e')
                {
                    // Skip <code
                    for _ in 0..5 {
                        self.advance();
                    }

                    // Extract language from class attribute
                    let mut language = "text".to_string();
                    if self.current_char() == Some(' ')
                        && self.peek_char(1) == Some('c')
                        && self.peek_char(2) == Some('l')
                        && self.peek_char(3) == Some('a')
                        && self.peek_char(4) == Some('s')
                        && self.peek_char(5) == Some('s')
                        && self.peek_char(6) == Some('=')
                        && self.peek_char(7) == Some('"')
                    {
                        // Skip class="
                        for _ in 0..8 {
                            self.advance();
                        }

                        // Extract language
                        let mut lang = String::new();
                        while let Some(c) = self.current_char() {
                            if c == '"' {
                                self.advance();
                                break;
                            }
                            lang.push(c);
                            self.advance();
                        }

                        if let Some(stripped) = lang.strip_prefix("language-") {
                            language = stripped.to_string();
                        }
                    }

                    // Skip to >
                    while let Some(c) = self.current_char() {
                        if c == '>' {
                            self.advance();
                            break;
                        }
                        self.advance();
                    }

                    // Extract code content until </code></pre>
                    let mut code_content = String::new();

                    while self.position < self.content.len() {
                        if self.current_char() == Some('<')
                            && self.peek_char(1) == Some('/')
                            && self.peek_char(2) == Some('c')
                            && self.peek_char(3) == Some('o')
                            && self.peek_char(4) == Some('d')
                            && self.peek_char(5) == Some('e')
                            && self.peek_char(6) == Some('>')
                        {
                            // Found </code>
                            for _ in 0..7 {
                                self.advance();
                            }

                            // Look for </pre>
                            if self.current_char() == Some('<')
                                && self.peek_char(1) == Some('/')
                                && self.peek_char(2) == Some('p')
                                && self.peek_char(3) == Some('r')
                                && self.peek_char(4) == Some('e')
                                && self.peek_char(5) == Some('>')
                            {
                                // Found </pre>
                                for _ in 0..6 {
                                    self.advance();
                                }
                                break;
                            }
                        } else {
                            if let Some(c) = self.current_char() {
                                code_content.push(c);
                            }
                            self.advance();
                        }
                    }

                    code_blocks.push((language, code_content));
                }
            } else {
                self.advance();
            }
        }

        code_blocks
    }
}

/// Process markdown content and enhance code blocks
pub fn process_markdown_content(content: &str) -> String {
    debug!(
        "Starting markdown processing, content length: {}",
        content.len()
    );

    // First, process the markdown content with pulldown-cmark
    let parser = pulldown_cmark::Parser::new_ext(content, pulldown_cmark::Options::all());
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    
    debug!("Markdown processed, HTML length: {}", html_output.len());
    
    // Now enhance the HTML code blocks with syntax highlighting
    let enhanced_html = enhance_html_code_blocks(&html_output);

    debug!(
        "HTML code blocks enhanced, final length: {}",
        enhanced_html.len()
    );
    enhanced_html
}

/// Enhance HTML code blocks with syntax highlighting
fn enhance_html_code_blocks(html: &str) -> String {
    debug!("Enhancing HTML code blocks, input length: {}", html.len());

    let mut parser = HTMLParser::new(html);
    let code_blocks = parser.extract_code_blocks();

    debug!("Extracted {} HTML code blocks", code_blocks.len());

    // Replace code blocks with enhanced versions
    let mut result = html.to_string();
    let mut offset = 0;

    for (language, code_content) in code_blocks {
        debug!(
            "Found HTML code block - Language: '{}', Content length: {}",
            language,
            code_content.len()
        );
        debug!(
            "Code block content preview: {}",
            &code_content[..code_content.len().min(100)]
        );

        // Try different search patterns for different language scenarios
        let search_patterns = vec![
            format!(
                "<pre><code class=\"language-{}\">{}</code></pre>",
                language, code_content
            ),
            format!("<pre><code>{}</code></pre>", code_content), // For plain text without class
            format!(
                "<pre><code class=\"{}\">{}</code></pre>",
                language, code_content
            ), // Without language- prefix
        ];

        let mut found_pattern = None;
        let mut actual_start = 0;
        let mut end_pos = 0;

        for pattern in &search_patterns {
            if let Some(start_pos) = result[offset..].find(pattern) {
                actual_start = offset + start_pos;
                end_pos = actual_start + pattern.len();
                found_pattern = Some(pattern);
                debug!("Found pattern: {}", pattern);
                break;
            }
        }

        if let Some(_pattern) = found_pattern {
            // Use fallback text for unspecified languages
            debug!("Language: {}", language);
            let display_language = if language.is_empty() || language == "text" {
                "TEXT"
            } else {
                &language
            };

            // For text language, don't apply syntax highlighting
            let final_content = if language == "text" {
                // Just escape HTML entities for plain text
                escape_html_entities(&code_content)
            } else {
                // Apply syntax highlighting for other languages
                let code_block = CodeBlock::new(language.clone(), code_content.clone());
                code_block.highlight()
            };
        
        let enhanced_html = format!(
                r#"<pre data-language="{}" class="code-block-container"><code class="language-{}">{}</code></pre>"#,
                display_language, language, final_content
        );
        
            // Replace the original with enhanced version
            result.replace_range(actual_start..end_pos, &enhanced_html);
            offset = actual_start + enhanced_html.len();
        }
    }
    
    result
}
 