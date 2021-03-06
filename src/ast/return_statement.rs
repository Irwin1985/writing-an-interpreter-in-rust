use token::Token;
use ast::{Node, Statement, Expression};

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<Expression>>,
}

impl Statement for ReturnStatement {
    fn node(&self) -> String {
        "".to_owned()
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s = s + self.token_literal().as_str();
        if let Some(ref expression) = self.return_value {
            s = s + expression.to_string().as_ref();
        }
        s += ";";
        s
    }
}
