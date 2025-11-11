use std::process;

/// Trait for a simple lexical analyzer
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// Lexical Analyzer implementation
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    current_char: char,
    buffer: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();

        Self {
            source: chars,
            position: 0,
            current_char: '\0',
            buffer: String::new(),
        }
    }

    pub fn next_token(&mut self) -> String {
        self.buffer.clear();

        // Initialize current_char on first call
        if self.position == 0 && self.current_char == '\0' {
            self.get_char();
        }

        while self.current_char.is_whitespace() && self.current_char != '\0' {
            self.get_char();
        }

        if self.current_char == '\0' {
            return String::new();
        }

        if self.current_char == '#' {
            return self.read_keyword();
        }

        self.read_text()
    }

    fn read_keyword(&mut self) -> String {
        self.add_char(self.current_char);
        self.get_char();

        while self.current_char.is_alphabetic() {
            self.add_char(self.current_char);
            self.get_char();
        }

        let current_buffer = self.buffer.clone().to_lowercase();

        if matches!(
            current_buffer.as_str(),
            "#maek" | "#gimmeh" | "#i" | "#it" | "#lemme"
        ) {
            while self.current_char == ' ' || self.current_char == '\t' {
                self.add_char(self.current_char);
                self.get_char();
            }

            while self.current_char.is_alphabetic() {
                self.add_char(self.current_char);
                self.get_char();
            }
        }

        let token = self.buffer.clone();

        if !self.lookup(&token) {
            eprintln!("Lexical error: '{}' is not a valid keyword", token);
            process::exit(1);
        }

        token
    }

    fn read_text(&mut self) -> String {
        while self.current_char != '\0' && self.current_char != '#' {
            self.add_char(self.current_char);
            self.get_char();
        }

        self.buffer.trim().to_string()
    }
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        if self.position >= self.source.len() {
            self.current_char = '\0';
            return '\0';
        }

        self.current_char = self.source[self.position];
        self.position += 1;
        self.current_char
    }

    fn add_char(&mut self, c: char) {
        self.buffer.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        let lower = s.to_lowercase();
        matches!(
            lower.as_str(),
            "#hai"
                | "#kthxbye"
                | "#obtw"
                | "#tldr"
                | "#maek head"
                | "#maek paragraf"
                | "#maek list"
                | "#gimmeh italics"
                | "#gimmeh title"
                | "#gimmeh item"
                | "#gimmeh newline"
                | "#gimmeh soundz"
                | "#gimmeh vidz"
                | "#gimmeh bold"
                | "#mkay"
                | "#oic"
                | "#i haz"
                | "#it iz"
                | "#lemme see"
        )
    }
}
