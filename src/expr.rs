pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> crate::Value;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> crate::Value;
    fn visit_literal_expr(&mut self, expr: &Literal) -> crate::Value;
    fn visit_unary_expr(&mut self, expr: &Unary) -> crate::Value;
}

pub trait Expr {
    fn children(&self) -> Vec<&dyn Expr>;
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value;
}

pub struct Binary<'a> {
    pub left: Box<dyn Expr>,
    pub operator: crate::Token<'a>,
    pub right: Box<dyn Expr>,
}
impl<'a> Binary<'a> {
    pub fn new(left: Box<dyn Expr>, operator: crate::Token<'a>, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<'a> Expr for Binary<'a> {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value {
        visitor.visit_binary_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![&*self.left, &*self.right]
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value {
        visitor.visit_grouping_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![&*self.expression]
    }
}

pub struct Literal {
    pub value: crate::Value,
}
impl Literal {
    pub fn new(value: crate::Value) -> Self {
        Self { value }
    }
}

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value {
        visitor.visit_literal_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![]
    }
}

pub struct Unary<'a> {
    pub operator: crate::Token<'a>,
    pub right: Box<dyn Expr>,
}
impl<'a> Unary<'a> {
    pub fn new(operator: crate::Token<'a>, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}

impl<'a> Expr for Unary<'a> {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value {
        visitor.visit_unary_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![&*self.right]
    }
}
