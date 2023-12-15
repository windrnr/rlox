use crate::Token;

#[derive(Debug)]
pub enum VisitorReturnValues {
    String(String),
}

pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> VisitorReturnValues;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> VisitorReturnValues;
    fn visit_literal_expr(&mut self, expr: &Literal) -> VisitorReturnValues;
    fn visit_unary_expr(&mut self, expr: &Unary) -> VisitorReturnValues;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnValues;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnValues {
        visitor.visit_binary_expr(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Grouping { expression }
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnValues {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    pub value: crate::Literal,
}
impl Literal {
    pub fn new(value: crate::Literal) -> Self {
        Literal { value }
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnValues {
        visitor.visit_literal_expr(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnValues {
        visitor.visit_unary_expr(self)
    }
}
