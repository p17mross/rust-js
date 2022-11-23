use super::Lexer;

#[derive(Debug)]
pub struct Engine {
    lexer: Lexer,
    //parser: super::Parser,
    //runtime state
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            lexer: Default::default(),
        }
    }

    pub fn parse(&mut self, s: &str) {
        let tokens = self.lexer.lex(s);
        
        let Ok(tokens) = tokens else {
            println!("Error: {tokens:?}");
            return;
        };

        for token in tokens {
            println!("{token:?}")
        }

        // TODO: parse to AST
    }
}