use crate::parser::lexer::Lexer;
use crate::parser::token::{ Token, Node, OperPrec };

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::UnexpectedToken("Unexpected end of input".into()))
        };

        Ok(Self {
            lexer,
            current_token: cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e)
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::UnexpectedEndOfInput)
        };
        self.current_token = next_token;
        Ok(())
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token == expected {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken("Unexpected token".into()))
        }
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Neg(Box::new(expr)))
            }
            Token::Number(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RParen)?;
                if self.current_token == Token::LParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Mul(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unexpected token".into()))
        }
    }

    fn generate_ast(&mut self, prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;
        while prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }

            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_exp = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_exp)))
            },
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Sub(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Mul(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Div(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Pow => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Pow)?;
                Ok(Node::Pow(Box::new(left_expr), Box::new(right_expr)))
            },
            _ => Err(ParseError::InvalidOperator(format!("Please enter valid operator {:?}", self.current_token)))
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnableToParse(String),
    UnexpectedEndOfInput,
    InvalidOperator(String),
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}
