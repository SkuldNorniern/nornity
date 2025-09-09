use super::LanguageLexer;
use super::{TokenType, Token};
use super::BaseLexer;

pub struct BashLexer;

impl LanguageLexer for BashLexer {
    fn get_keywords(&self) -> &[&str] {
        &[
            // Shell syntax keywords
            "while", "do", "done", "if", "then", "fi", "for", "in", "case", "esac", "elif",
            "echo", // kept for backward compatibility but will be classified as command when appropriate
            "grep", "ls", "cd", "mkdir", "rm", "cp", "mv", "cat", "head", "tail", "cut", "local",
            "read", "function", "return", "exit",
        ]
    }

    fn lex(&self, input: &str) -> Vec<Token> {
        let mut lexer = BaseLexer::new(input);
        let keywords = self.get_keywords();

        // Common builtins to highlight as commands when in command position
        let builtins: &[&str] = &[
            "echo", "printf", "read", "exit", "return", "cd", "pwd", "test", "[", "alias",
            "unalias", "type", "hash", "help", "let", "mapfile", "readonly", "shift", "trap",
            "umask", "unset", "export", "local", "declare", "typeset", "jobs", "bg", "fg", "kill",
            "wait", "dirs", "pushd", "popd", ".", "source", "set", "unset", "env", "getopts",
            "shift", "eval", "exec", "trap", "wait", "suspend", "caller", "command", "builtin",
        ];

        let mut expecting_command = true; // start of file acts like start of line

        while lexer.position < lexer.input.len() {
            let ch = lexer.current_char().unwrap_or('\0');

            // Shebang at file start: #!...
            if lexer.position == 0 && ch == '#' && lexer.peek_char(1) == Some('!') {
                let start = lexer.position;
                let mut value = String::from("#!");
                lexer.advance(); // '#'
                lexer.advance(); // '!'
                while let Some(c) = lexer.current_char() {
                    if c == '\n' || c == '\r' {
                        break;
                    }
                    value.push(c);
                    lexer.advance();
                }
                lexer.add_token(TokenType::Shebang, value, start, lexer.position);
                expecting_command = true;
                continue;
            }

            match ch {
                ' ' | '\t' => {
                    lexer.consume_whitespace();
                }
                '\n' | '\r' => {
                    lexer.consume_newlines();
                    expecting_command = true;
                }
                '#' => {
                    // Regular comment
                    lexer.consume_single_line_comment("#");
                    expecting_command = true;
                }
                '"' | '\'' => {
                    lexer.consume_string(ch);
                    // inside quotes does not change command expectation
                }
                // Backticks for command substitution
                '`' => {
                    let start = lexer.position;
                    let mut value = String::new();
                    value.push('`');
                    lexer.advance();
                    while let Some(c) = lexer.current_char() {
                        value.push(c);
                        lexer.advance();
                        if c == '`' {
                            break;
                        }
                    }
                    lexer.add_token(TokenType::String, value, start, lexer.position);
                }
                // Variables: $var, ${var}, $1, $@, $#, $?, $$, $!, $-, $(...) , $((...))
                '$' => {
                    let start = lexer.position;
                    let mut value = String::from("$");
                    lexer.advance();

                    match lexer.current_char() {
                        Some('{') => {
                            value.push('{');
                            lexer.advance();
                            while let Some(c) = lexer.current_char() {
                                value.push(c);
                                lexer.advance();
                                if c == '}' {
                                    break;
                                }
                            }
                        }
                        Some('(') => {
                            // $(...) or $((...))
                            value.push('(');
                            lexer.advance();
                            let mut depth: i32 = 1;
                            while let Some(c) = lexer.current_char() {
                                value.push(c);
                                lexer.advance();
                                if c == '(' {
                                    depth += 1;
                                }
                                if c == ')' {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        Some(c) => {
                            // simple variable name or special params
                            if c.is_alphanumeric()
                                || matches!(c, '_' | '@' | '*' | '#' | '?' | '!' | '-' | '$')
                            {
                                value.push(c);
                                lexer.advance();
                                while let Some(nc) = lexer.current_char() {
                                    if nc.is_alphanumeric()
                                        || matches!(nc, '_' | '@' | '*' | '#' | '?' | '!' | '-')
                                    {
                                        value.push(nc);
                                        lexer.advance();
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                        None => {}
                    }

                    lexer.add_token(TokenType::Variable, value, start, lexer.position);
                }
                // Numbers
                '0'..='9' => {
                    lexer.consume_number();
                }
                // Operators and separators
                '|' | '&' | ';' | '(' | ')' | '{' | '}' | '=' | '>' | '<' => {
                    // Single-character operators for simplicity
                    lexer.add_operator(ch.to_string());
                    // After pipe/and/or/semicolon and sub-shell delimiters, expect a new command
                    if matches!(ch, '|' | '&' | ';' | '(' | '{') {
                        expecting_command = true;
                    }
                }
                // Identifiers and commands (allow -, ., / in names)
                'a'..='z' | 'A'..='Z' | '_' | '/' | '.' | '-' => {
                    let start = lexer.position;
                    let mut value = String::new();
                    while let Some(c) = lexer.current_char() {
                        if c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | '/') {
                            value.push(c);
                            lexer.advance();
                        } else {
                            break;
                        }
                    }

                    // Function definition detection: name() pattern without space
                    let is_function_name =
                        lexer.current_char() == Some('(') && lexer.peek_char(1) == Some(')');

                    if expecting_command || is_function_name || builtins.contains(&value.as_str()) {
                        let token_type = if is_function_name {
                            TokenType::Function
                        } else {
                            TokenType::Command
                        };
                        lexer.add_token(token_type, value, start, lexer.position);
                        expecting_command = false;
                    } else if keywords.contains(&value.as_str()) {
                        lexer.add_token(TokenType::Keyword, value, start, lexer.position);
                    } else {
                        lexer.add_token(TokenType::Identifier, value, start, lexer.position);
                    }
                }
                _ => {
                    // Fallback: treat as other
                    lexer.add_other(ch.to_string());
                }
            }
        }

        lexer.get_tokens()
    }
}