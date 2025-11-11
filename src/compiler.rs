use crate::lexer::Lexer;
use crate::parser::{Parser, SyntaxAnalyzer};
use crate::semantic::SemanticAnalyzer;
use std::fs;
use std::process::{Command, exit};

/// Compiler trait as specified in project requirements
#[allow(dead_code)]
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

/// Main Compiler implementation
pub struct LolCompiler {
    #[allow(dead_code)]
    pub current_token: String,
    semantic: SemanticAnalyzer,
}

impl LolCompiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
            semantic: SemanticAnalyzer::new(),
        }
    }

    pub fn compile_file(&mut self, source: &str, input_file: &str) {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer, &mut self.semantic);
        
        parser.parse_lolcode();
        
        let html_output = self.semantic.get_output();
        let output_file = input_file.replace(".lol", ".html");
        
        if let Err(e) = fs::write(&output_file, html_output) {
            eprintln!("Error writing output file '{}': {}", output_file, e);
            exit(1);
        }

        self.launch_browser(&output_file);
    }

    fn launch_browser(&self, html_file: &str) {
        let abs_path = std::env::current_dir()
            .unwrap()
            .join(html_file)
            .to_string_lossy()
            .to_string();

        let chrome_paths = vec![
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ];

        let mut launched = false;
        for chrome_path in chrome_paths {
            if std::path::Path::new(chrome_path).exists() {
                match Command::new(chrome_path).arg(&abs_path).spawn() {
                    Ok(_) => {
                        launched = true;
                        break;
                    }
                    Err(_) => continue,
                }
            }
        }

        if !launched {
            println!("Note: Could not automatically launch Chrome. Please open {} manually.", html_file);
        }
    }
}

impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer, &mut self.semantic);
        parser.parse_lolcode();
    }

    fn next_token(&mut self) -> String {
        self.current_token.clone()
    }

    fn parse(&mut self) {
        // Implemented via compile method
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}
