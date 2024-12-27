
    #[derive(Debug, PartialEq, Clone)]
    pub enum Node {
        Add(Box<Node>, Box<Node>),
        Sub(Box<Node>, Box<Node>),
        Mul(Box<Node>, Box<Node>),
        Div(Box<Node>, Box<Node>),
        Pow(Box<Node>, Box<Node>),
        Neg(Box<Node>),
        Number(f64),
    }
    
    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum OperPrec {
        DefaultZero,
        AddSub,
        MulDiv,
        Pow,
        Negative
    }
    
    
    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        Number(f64),
        Add,
        Subtract,
        Multiply,
        Divide,
        LParen,
        RParen,
        Pow,
        EOF,
    }
    
    impl Token {
        pub fn get_oper_prec(&self) -> OperPrec {
            match self {
                Token::Add | Token::Subtract => OperPrec::AddSub,
                Token::Multiply | Token::Divide => OperPrec::MulDiv,
                Token::Pow => OperPrec::Pow,
                _ => OperPrec::DefaultZero,
            }
        }
    }