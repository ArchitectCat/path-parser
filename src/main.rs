use crate::parser::{AstNode, AstNodeType};

mod lexer;
mod parser;

fn validate_path(path: &str) -> bool {
    let tokens = lexer::tokenize(path);
    return match tokens {
        Ok(t) => {
            println!("Tokens: {:?}", t);
            
            let mut root_node = AstNode::new();
            let parsed = parser::parse(&t, &mut root_node, 0);            
            match parsed {
                Ok (ast) => {
                    println!("AST: {:?}", ast);
                    true
                }
                Err(e) => {
                    println!("{:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

fn main() {    
    let test = "/test/{foo}/123/.api-secret_here/{**remainder}";
    println!("Path is valid: {}", validate_path(test));
}
