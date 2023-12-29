use crate::expr::{Expr, Visitor};
use crate::expr;
use crate::Value;

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    pub fn print(&mut self, expr: Box<dyn Expr>) {
        if let Value::String(str) = expr.accept(self) {
            println!("{str}")
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&dyn Expr]) -> Option<String> {
        let mut result = format!("({name}");

        for expr in exprs {
            match expr.accept(self) {
                Value::String(inner) => {
                    result.push_str(&inner);
                    result.push(' ');
                }
                _ => return None,
            }
        }
        result.push(')');
        Some(result)
    }
}

impl Visitor for AstPrinter {
    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Value {
        Value::String(
            self.parenthesize(&expr.operator.lexeme, &expr.children())
                .unwrap(),
        )
    }
    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Value {
        Value::String(
            self.parenthesize(&expr.operator.lexeme, &expr.children())
                .unwrap(),
        )
    }
    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> Value {
        match &expr.value {
            Value::None => Value::String("nil".to_string()),
            Value::String(a) => Value::String(a.to_string()),
            Value::Number(a) => Value::String(a.to_string()),
            Value::Boolean(a) => Value::String(a.to_string()),
        }
    }
    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Value {
        Value::String(self.parenthesize("group", &expr.children()).unwrap())
    }
}
