use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum AstNodeType {
    Root,
    Eof,
    Separator,
    ResourceName(String),
    Parameter(String),
    Invalid(String),
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub children: Vec<AstNode>,
    pub value: AstNodeType,
}

impl AstNode {
    pub fn new() -> AstNode {
        AstNode {
            children: Vec::new(),
            value: AstNodeType::Root,
        }
    }
}

pub fn parse(tokens: &Vec<Token>, root: &mut AstNode, pos: usize) -> Result<AstNode, String> {
    let curr = tokens.get(pos);
    match curr {
        Some(t) => {
            match t {
                Token::Separator => {
                    
                }
                Token::WildChar => {}
                Token::OpenBracket => {}
                Token::CloseBracket => {}
                Token::Numeric(_) => {}
                Token::Identifier(_) => {}
            }

            Ok(root.clone())
        }
        _ => {
            root.children.push(AstNode { value: AstNodeType::Eof, children: vec![] });
            return Ok(root.clone());
        }
    }
}