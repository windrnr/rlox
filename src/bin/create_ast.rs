use ::std::{
    io::{self},
    process,
    path::Path,
    fs::write,
};
use std::fs;

fn define_type(file_path: &Path, struct_name: &str, fields: &str) -> io::Result<()> {
    let fields_binding = fields.split(',').collect::<Vec<&str>>();

    let struct_body = fields_binding
        .iter()
        .filter_map(|field| {
            if let Some((var_name, tok_name)) = field.to_string().split_once(':') {
                let left_side = format!("pub {}", var_name.trim());
                let rigth_side = match tok_name.trim() {
                    "Token" => "crate::Token".to_string(),
                    "Literal" => "crate::Value".to_string(),
                    _ => tok_name.trim().to_string(),
                };
                Some(format!("{left_side}:{rigth_side}, \n\t"))
            } else {
                None
            }
        })
        .collect::<String>();

    let fn_arguments = fields_binding
        .iter()
        .filter_map(|field| {
            if let Some((left_name, right_name)) = field.to_string().split_once(':') {
                let rigth_side = match right_name.trim() {
                    "Token" => "crate::Token".to_string(),
                    "Literal" => "crate::Value".to_string(),
                    _ => right_name.trim().to_string(),
                };
                Some(format!("{left_name}: {rigth_side}, "))
            } else {
                None
            }
        })
        .collect::<String>();

    let new_arguments = fields_binding
        .iter()
        .filter_map(|field| {
            if let Some((name, _)) = field.split_once(':') {
                Some(format!("{name}, "))
            } else {
                None
            }
        })
        .collect::<String>();

    let result = format!(
        r#"
pub struct {struct_name} {{
    {struct_body}
}}

impl {struct_name} {{
    pub fn new({fn_arguments}) -> Self {{
        Self {{ {new_arguments} }}
    }}
}}

impl Expr for {struct_name} {{
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value {{
        visitor.visit_{struct_name_lowercase}_expr(self)
    }}
    fn children(&self) -> Vec<&dyn Expr> {{
        {children_return_statement}
    }}
}}

"#,
        struct_name_lowercase = struct_name.to_lowercase(),
        children_return_statement = match struct_name.trim() {
            "Binary" => "vec![&*self.left, &*self.right]".to_string(),
            "Grouping" => "vec![&*self.expression]".to_string(),
            "Literal" => "vec![]".to_string(),
            "Unary" => "vec![&*self.right]".to_string(),
            _ => unreachable!(),
        },
    );
    append_to_file(file_path, &result)?;
    Ok(())
}

fn visitor_trait_definition(file_path: &Path, types: &[&str]) -> io::Result<()> {
    let body = types
        .iter()
        .filter_map(|t| {
            if let Some((type_name, _)) = t.split_once('=') {
                let fn_name = format!("visit_{}_expr", type_name.trim());
                Some(format!(
                    "\tfn {} (&mut self, expr: &{}) -> crate::Value; \n",
                    fn_name.to_lowercase(),
                    type_name
                ))
            } else {
                None
            }
        })
        .collect::<String>();

    let result = format!(
r#"pub trait Visitor {{
    {body}
}}
"#
    );

    append_to_file(file_path, &result)?;
    Ok(())
}

fn append_to_file(file_path: &Path, content_to_append: &str) -> io::Result<()> {
    let existing_content = fs::read_to_string(file_path).unwrap_or_default();
    let combined = format!("{}{}", existing_content, content_to_append);
    write(file_path, combined)?;
    Ok(())
}

fn define_ast(file_path: &Path, types: Vec<&str>) -> io::Result<()> {
    visitor_trait_definition(file_path, &types)?;

    let fn_definition = 
        r#"
pub trait Expr {
    fn children(&self) -> Vec<&dyn Expr>;
    fn accept(&self, visitor: &mut dyn Visitor) -> crate::Value;
}


"#.to_string();

    append_to_file(file_path, &fn_definition)?;

    for t in types {
        if let Some((name, fields)) = t.split_once('=') {
            define_type(file_path, name.trim(), fields.trim())?
        }
    }
    Ok(())
}

fn generate_ast() -> io::Result<()> {
    let dir_file_path = Path::new("./src/bin/rlox/expr.rs");

    if fs::metadata(dir_file_path)?.is_file() {
        fs::remove_file(dir_file_path)?
    }

    define_ast(
        dir_file_path,
        vec![
            "Binary   = left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>",
            "Grouping = expression: Box<dyn Expr>",
            "Literal  = value: Literal",
            "Unary    = operator: Token, right: Box<dyn Expr>",
        ],
    )?;
    Ok(())
}

fn main() {
    if let Err(error) = generate_ast() {
        eprintln!("{}", error);
        process::exit(64);
    }
}
