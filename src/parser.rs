use crate::lexer::Lexer;
use crate::semantic::SemanticAnalyzer;
use std::process;

/// Syntax Analyzer trait as specified in project requirements
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_audio(&mut self);
    fn parse_video(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

/// Parser implementation
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: String,
    semantic: &'a mut SemanticAnalyzer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, semantic: &'a mut SemanticAnalyzer) -> Self {
        Self {
            lexer,
            current_token: String::new(),
            semantic,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn matches(&self, expected: &str) -> bool {
        self.current_token.to_lowercase() == expected.to_lowercase()
    }

    fn expect(&mut self, expected: &str) {
        if !self.matches(expected) {
            eprintln!(
                "Syntax error: Expected '{}', found '{}'",
                expected, self.current_token
            );
            process::exit(1);
        }
        self.next_token();
    }
}

impl<'a> SyntaxAnalyzer for Parser<'a> {
    fn parse_lolcode(&mut self) {
        self.next_token();

        if !self.matches("#hai") {
            eprintln!("Syntax error: Program must start with #HAI");
            process::exit(1);
        }
        self.semantic.emit("<html>\n");
        self.next_token();

        self.parse_body();

        if !self.matches("#kthxbye") {
            eprintln!("Syntax error: Program must end with #KTHXBYE");
            process::exit(1);
        }
        self.semantic.emit("</html>");
    }

    fn parse_body(&mut self) {
        // Comments can appear before head
        while self.matches("#obtw") {
            self.parse_comment();
        }

        // Optional head
        if self.matches("#maek head") {
            self.parse_head();
        }

        // More comments after head
        while self.matches("#obtw") {
            self.parse_comment();
        }

        // Content list
        while !self.matches("#kthxbye") && !self.current_token.is_empty() {
            if self.matches("#maek paragraf") {
                self.parse_paragraph();
            } else if self.matches("#gimmeh bold") {
                self.parse_bold();
            } else if self.matches("#gimmeh italics") {
                self.parse_italics();
            } else if self.matches("#maek list") {
                self.parse_list();
            } else if self.matches("#gimmeh newline") {
                self.parse_newline();
            } else if self.matches("#gimmeh vidz") {
                self.parse_video();
            } else if self.matches("#gimmeh soundz") {
                self.parse_audio();
            } else if self.matches("#i haz") {
                self.parse_variable_define();
            } else if self.matches("#lemme see") {
                self.parse_variable_use();
            } else if !self.current_token.starts_with('#') {
                self.parse_text();
            } else {
                eprintln!("Syntax error: Unexpected token '{}'", self.current_token);
                process::exit(1);
            }
        }
    }

    fn parse_head(&mut self) {
        self.expect("#maek head");
        self.semantic.emit("<head>\n");
        self.parse_title();
        self.expect("#oic");
        self.semantic.emit("</head>\n");
    }

    fn parse_title(&mut self) {
        self.expect("#gimmeh title");
        self.semantic.emit("<title>");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Title cannot be empty");
            process::exit(1);
        }
        self.semantic.emit(&self.current_token);
        self.next_token();

        self.expect("#mkay");
        self.semantic.emit("</title>\n");
    }

    fn parse_comment(&mut self) {
        self.expect("#obtw");
        self.semantic.emit("<!-- ");

        if !self.current_token.is_empty() && !self.current_token.starts_with('#') {
            self.semantic.emit(&self.current_token);
            self.next_token();
        }

        self.expect("#tldr");
        self.semantic.emit(" -->\n");
    }

    fn parse_paragraph(&mut self) {
        self.expect("#maek paragraf");
        self.semantic.emit("<p>");
        self.semantic.push_scope();

        self.parse_inner_paragraph();

        if !self.matches("#oic") {
            eprintln!(
                "Syntax error: Expected #OIC to close paragraph, found '{}'",
                self.current_token
            );
            process::exit(1);
        }
        self.next_token();

        self.semantic.pop_scope();
        self.semantic.emit("</p>\n");
    }

    fn parse_inner_paragraph(&mut self) {
        while !self.matches("#oic") && !self.current_token.is_empty() {
            if self.matches("#gimmeh bold") {
                self.parse_bold();
            } else if self.matches("#gimmeh italics") {
                self.parse_italics();
            } else if self.matches("#gimmeh soundz") {
                self.parse_audio();
            } else if self.matches("#gimmeh vidz") {
                self.parse_video();
            } else if self.matches("#gimmeh newline") {
                self.parse_newline();
            } else if self.matches("#maek list") {
                self.parse_list();
            } else if self.matches("#i haz") {
                self.parse_variable_define();
            } else if self.matches("#lemme see") {
                self.parse_variable_use();
            } else if !self.current_token.starts_with('#') {
                self.parse_text();
            } else {
                eprintln!(
                    "Syntax error: Unexpected token in paragraph '{}'",
                    self.current_token
                );
                process::exit(1);
            }
        }
    }

    fn parse_inner_text(&mut self) {
        self.parse_text();
    }

    fn parse_variable_define(&mut self) {
        self.expect("#i haz");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Variable name cannot be empty");
            process::exit(1);
        }
        let var_name = self.current_token.clone();
        self.next_token();

        self.expect("#it iz");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Variable value cannot be empty");
            process::exit(1);
        }
        let var_value = self.current_token.clone();
        self.next_token();

        self.expect("#mkay");
        self.semantic.define_variable(var_name, var_value);
    }

    fn parse_variable_use(&mut self) {
        self.expect("#lemme see");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Variable name cannot be empty");
            process::exit(1);
        }
        let var_name = self.current_token.clone();
        self.next_token();

        self.expect("#mkay");
        let value = self.semantic.lookup_variable(&var_name);
        self.semantic.emit(&value);
    }

    fn parse_bold(&mut self) {
        self.expect("#gimmeh bold");
        self.semantic.emit("<b>");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Bold text cannot be empty");
            process::exit(1);
        }
        self.semantic.emit(&self.current_token);
        self.next_token();

        self.expect("#mkay");
        self.semantic.emit("</b>");
    }

    fn parse_italics(&mut self) {
        self.expect("#gimmeh italics");
        self.semantic.emit("<i>");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Italics text cannot be empty");
            process::exit(1);
        }
        self.semantic.emit(&self.current_token);
        self.next_token();

        self.expect("#mkay");
        self.semantic.emit("</i>");
    }

    fn parse_list(&mut self) {
        self.expect("#maek list");
        self.semantic.emit("<ul>\n");
        self.semantic.push_scope();

        self.parse_list_items();

        self.expect("#oic");
        self.semantic.pop_scope();
        self.semantic.emit("</ul>\n");
    }

    fn parse_list_items(&mut self) {
        if !self.matches("#gimmeh item") {
            eprintln!("Syntax error: List must contain at least one item");
            process::exit(1);
        }

        while self.matches("#gimmeh item") {
            self.parse_inner_list();
        }
    }

    fn parse_inner_list(&mut self) {
        self.expect("#gimmeh item");
        self.semantic.emit("<li>");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: List item cannot be empty");
            process::exit(1);
        }
        self.semantic.emit(&self.current_token);
        self.next_token();

        self.expect("#mkay");
        self.semantic.emit("</li>\n");
    }

    fn parse_audio(&mut self) {
        self.expect("#gimmeh soundz");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Audio address cannot be empty");
            process::exit(1);
        }

        self.semantic.emit(&format!(
            "<audio controls><source src=\"{}\"></audio>\n",
            self.current_token
        ));
        self.next_token();

        self.expect("#mkay");
    }

    fn parse_video(&mut self) {
        self.expect("#gimmeh vidz");

        if self.current_token.is_empty() || self.current_token.starts_with('#') {
            eprintln!("Syntax error: Video address cannot be empty");
            process::exit(1);
        }

        self.semantic.emit(&format!(
            "<iframe src=\"{}\"></iframe>\n",
            self.current_token
        ));
        self.next_token();

        self.expect("#mkay");
    }

    fn parse_newline(&mut self) {
        self.expect("#gimmeh newline");
        self.semantic.emit("<br>\n");
    }

    fn parse_text(&mut self) {
        if !self.current_token.is_empty() && !self.current_token.starts_with('#') {
            self.semantic.emit(&self.current_token);
            self.semantic.emit(" ");
            self.next_token();
        }
    }
}
