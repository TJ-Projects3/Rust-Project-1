use std::collections::HashMap;
use std::process;

/// Semantic Analyzer - handles variable scoping and HTML generation.
/// 
/// This analyzer performs two key functions:
/// 1. Static scope resolution using a symbol table stack
/// 2. HTML code generation (semantic actions)
/// 
/// Symbol Table Stack for Static Scoping:
/// - Each scope (global, paragraph, list) gets its own symbol table (HashMap)
/// - Symbol tables are organized in a stack structure
/// - When entering a new scope (e.g., paragraph), push a new table
/// - When exiting a scope, pop the table (variables go out of scope)
/// - Variable lookup searches from top of stack (innermost) to bottom (outermost)
/// - This implements static (lexical) scoping as required
/// 
/// HTML Generation:
/// - As we parse, we emit corresponding HTML tags and text
/// - The output string accumulates the complete HTML document
pub struct SemanticAnalyzer {
    /// Stack of symbol tables - each HashMap represents one scope level
    /// Bottom of stack = global scope, top = current innermost scope
    scope_stack: Vec<HashMap<String, String>>,
    
    /// Accumulated HTML output as we traverse the parse tree
    output: String,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![HashMap::new()], // Start with global scope
            output: String::new(),
        }
    }

    /// Push a new scope onto the stack
    pub fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    /// Pop the current scope from the stack
    pub fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }

    /// Define a variable in the current scope
    pub fn define_variable(&mut self, name: String, value: String) {
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.insert(name, value);
        }
    }

    /// Look up a variable value using static (lexical) scoping.
    /// 
    /// Static Scoping Algorithm:
    /// 1. Start at the top of the scope stack (current/innermost scope)
    /// 2. Check if variable exists in current scope's symbol table
    /// 3. If found, return its value
    /// 4. If not found, move to next outer scope (down the stack)
    /// 5. Repeat until variable found or all scopes exhausted
    /// 6. If never found, this is a static semantic error
    /// 
    /// Example with nested scopes:
    ///   Global scope: { myname: "Josh" }
    ///   Paragraph scope: { myname: "Jon" }
    ///   
    ///   Looking up "myname" from paragraph will find "Jon"
    ///   Looking up "myname" after paragraph finds "Josh"
    ///   
    /// This is "static" because scope is determined by program structure,
    /// not by runtime call stack (as in dynamic scoping).
    pub fn lookup_variable(&self, name: &str) -> String {
        // Search from innermost to outermost scope
        // .iter().rev() traverses stack from top to bottom
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return value.clone();  // Variable found - return its value
            }
        }

        // Variable not found in any scope - this is a static semantic error
        // In a real compiler, this would be caught during a separate semantic analysis pass
        // Here we catch it during parsing and immediately report the error
        eprintln!("Static semantic error: Variable '{}' used before definition", name);
        process::exit(1);
    }

    /// Emit HTML code to the output
    pub fn emit(&mut self, html: &str) {
        self.output.push_str(html);
    }

    /// Get the generated HTML output
    pub fn get_output(&self) -> &str {
        &self.output
    }

    /// Clear the output (for testing or reset)
    pub fn clear(&mut self) {
        self.output.clear();
        self.scope_stack.clear();
        self.scope_stack.push(HashMap::new());
    }
}
