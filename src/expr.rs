pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> crate::Literal;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> crate::Literal;
    fn visit_literal_expr(&mut self, expr: &Literal) -> crate::Literal;
    fn visit_unary_expr(&mut self, expr: &Unary) -> crate::Literal;
}

pub trait Expr {
    fn children(&self) -> Vec<&dyn Expr>;
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Literal;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: crate::Token,
    pub right: Box<dyn Expr>,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: crate::Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Literal {
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
        Grouping { expression }
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Literal {
        visitor.visit_grouping_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![&*self.expression]
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
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Literal {
        visitor.visit_literal_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![]
    }
}

pub struct Unary {
    pub operator: crate::Token,
    pub right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: crate::Token, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Literal {
        visitor.visit_unary_expr(self)
    }

    fn children(&self) -> Vec<&dyn Expr> {
        vec![&*self.right]
    }
}
