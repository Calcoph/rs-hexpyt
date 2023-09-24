use std::{path::PathBuf, fmt::format};

use hexparser::{Expr, Value, m_parser::{HexTypeDef, HexType, BinaryOp, Assignment, UnaryOp}, token::ValueType};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rs_hexpyt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(translate_file, m)?)?;
    py.run(&include_str!("../hexpyt/src/primitives.py"), None, None).unwrap();
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (
    input_file_path,
    output_file_path,
    indentation="    ",
    extra_paths=Vec::new()
))]
fn translate_file(input_file_path: PathBuf, output_file_path: PathBuf, indentation: &str, extra_paths: Vec<String>) -> PyResult<()> {
    let input_file = std::fs::read_to_string(input_file_path)?;

    let (ast, errors, _) = hexparser::parse(&input_file, &extra_paths);

    let mut output_file = get_header();

    output_file += &translate_expr(ast.0);

    std::fs::write(output_file_path, output_file)?;

    Ok(())
}

fn get_header() -> String {
    let mut final_string = "from primitives import Dollar, Struct, BitField, IntStruct, ".to_string();
    final_string += "u8, u16, u24, u32, u48, u64, u96, u128, ";
    final_string += "s8, s16, s24, s32, s48, s64, s96, s128, ";
    final_string += "Float, double, char, char16, Bool, ";
    final_string += "Padding, Array, Enum, sizeof, addressof\n";
    final_string += r#"
# Template to read from a file. follow the instructions.
# _dollar___offset has this name so it doesn't clash with others. Feel free to rename it. 
if True: # Change this from "if True" to "if False", then put the file path below.
    byts = b''
else:
    file_path = "" # Put the file path here and change the above "if True" to "if False".
    with open(file_path, "rb") as f:
        byts = f.read()
_dollar___offset = Dollar(0x00, byts)
# End of template
"#;

    return final_string
}

fn translate_expr(expr: Expr) -> String {
    match expr {
        Expr::Error => "raise Error".to_string(),
        Expr::Value { val } => {
            match val {
                Value::Null => "None".to_string(),
                Value::Bool(b) => if b {
                    "True".to_string()
                } else {
                    "False".to_string()
                },
                Value::Num(n) => n.to_string(),
                Value::Str(s) => s,
                Value::Char(c) => c.to_string(),
                Value::Func(f) => f,
            }
        },
        Expr::ExprList { list } => {
            list.into_iter()
                .map(|(expr, _)| translate_expr(expr))
                .fold(String::new(), |old_str, expr| old_str + "\n" + &expr)
        },
        Expr::UnnamedParameter { type_ } => translate_hextype(type_.0),
        Expr::Local { name } => name.0,
        Expr::Unary { operation, operand } => {
            match operation {
                UnaryOp::Add => format!("+{}", translate_expr(operand.0)),
                UnaryOp::Sub => format!("-{}", translate_expr(operand.0)),
                UnaryOp::LNot => format!("not {}", translate_expr(operand.0)),
                UnaryOp::BNot => format!("!{}", translate_expr(operand.0)),
            }
        },
        Expr::Binary { loperand, operator, roperand } => {
            let operator = match operator {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Eq => "==",
                BinaryOp::NotEq => "!=",
                BinaryOp::Mod => "%",
                BinaryOp::LShift => "<<",
                BinaryOp::RShift => ">>",
                BinaryOp::BAnd => "&",
                BinaryOp::BXor => "^",
                BinaryOp::BOr => "|",
                BinaryOp::GreaterEqual => ">=",
                BinaryOp::LessEqual => "<=",
                BinaryOp::Greater => ">",
                BinaryOp::Less => "<",
                BinaryOp::LAnd => "and",
                BinaryOp::LXor => "xor",
                BinaryOp::LOr => "or",
                BinaryOp::Assign(a) => match a {
                    Assignment::Just => "=",
                    Assignment::Add => "+=",
                    Assignment::Sub => "-=",
                    Assignment::Mul => "*=",
                    Assignment::Div => "/=",
                    Assignment::Mod => "%=",
                    Assignment::RShift => ">>=",
                    Assignment::LShift => "<<=",
                    Assignment::BOr => "|=",
                    Assignment::BAnd => "&=",
                    Assignment::BXor => "^=",
                },
            };
            let loperand = translate_expr(loperand.0);
            let roperand = translate_expr(roperand.0);

            format!("{loperand} {operator} {roperand}")
        },
        Expr::Ternary { loperand, moperand, roperand } => {
            let loperand = translate_expr(loperand.0);
            let moperand = translate_expr(moperand.0);
            let roperand = translate_expr(roperand.0);

            format!("{loperand} ? {moperand} : {roperand}")
        },
        Expr::Call { func_name, arguments } => {
            let func_name = translate_expr(func_name.0);
            let arguments = arguments.0.into_iter()
                .map(|(expr, _)| translate_expr(expr))
                .fold(String::new(), |old_str, expr| old_str + ", " + &expr);

            format!("{func_name}({arguments})")
        },
        Expr::If { test, consequent } => {
            let test = translate_expr(test.0);
            let consequent = translate_expr(consequent.0);

            format!("if {test}:\n{consequent}")
        },
        Expr::IfBlock { ifs, alternative } => {
            let ifs = translate_expr(ifs.0);
            let alternative = translate_expr(alternative.0);

            format!("{ifs} {alternative}")
        },
        Expr::Definition { value_type, name, body } => {
            let value_type = translate_hextypedef(value_type.0);
            let name = translate_expr(name.0);
            let body = translate_expr(body.0);

            format!("{value_type} {name} {body}")
        },
        Expr::ArrayDefinition { value_type, array_name, size, body } => {
            let value_type = translate_hextypedef(value_type.0);
            let array_name = translate_expr(array_name.0);
            let body = translate_expr(body.0);

            format!("{value_type} {array_name} {body}")
        },
        Expr::BitFieldEntry { name, length } => {
            let name = name.0;
            let length = translate_expr(length.0);

            format!("{name} {length}")
        },
        Expr::EnumEntry { name, value } => {
            let name = name.0;
            let value = translate_expr(value.0);

            format!("{name} {value}")
        },
        Expr::NamespaceAccess { previous, name } => {
            let name = name.0;
            let previous = translate_expr(previous.0);

            format!("{name} {previous}")
        },
        Expr::Using { new_name, template_parameters, old_name } => {
            let new_name = new_name.0;
            let old_name = translate_hextypedef(old_name.0);
            
            if let Some(template_parameters) = template_parameters {
                let template_parameters = translate_expr(template_parameters.0);

                format!("{new_name} {template_parameters} {old_name}")
            } else {
                format!("{new_name} {old_name}")
            }
        },
        Expr::Return { value } => {
            let value = translate_expr(value.0);


            format!("return {value}")
        },
        Expr::Continue => {
            "continue".to_string()
        },
        Expr::Break => {
            "break".to_string()
        },
        Expr::Func { name, args, body } => {
            let name = name.0;
            let args = "TODO".to_string(); // TODO
            let body = translate_expr(body.0);

            format!("{name} {args} {body}")
        },
        Expr::Struct { name, body, template_parameters } => {
            let name = name.0;
            let body = translate_expr(body.0);

            if let Some(template_parameters) = template_parameters {
                let template_parameters = translate_expr(template_parameters.0);

                format!("{name} {template_parameters} {body}")
            } else {
                format!("{name} {body}")
            }
        },
        Expr::Namespace { name, body } => {
            let name = translate_expr(name.0);
            let body = translate_expr(body.0);

            format!("{name} {body}")
        },
        Expr::Enum { name, value_type, body } => {
            let name = name.0;
            let value_type = translate_hextypedef(value_type.0);
            let body = translate_expr(body.0);

            format!("{name} {value_type} {body}")
        },
        Expr::Bitfield { name, body } => {
            let name = name.0;
            let body = translate_expr(body.0);

            format!("{name} {body}")
        },
        Expr::Access { item, member } => {
            let item = translate_expr(item.0);
            let member = translate_expr(member.0);

            format!("{item} {member}")
        },
        Expr::ArrayAccess { array, index } => {
            let array = translate_expr(array.0);
            let index = translate_expr(index.0);

            format!("{array} {index}")
        },
        Expr::Attribute { arguments } => {
            todo!()
        },
        Expr::AttributeArgument { name, value } => {
            let name = translate_expr(name.0);
            let value = "TODO"; // TODO

            format!("{name} {value}")
        },
        Expr::WhileLoop { condition, body } => {
            let condition = translate_expr(condition.0);
            let body = translate_expr(body.0);

            format!("{condition} {body}")
        },
        Expr::ForLoop { var_init, var_test, var_change, body } => {
            let var_init = translate_expr(var_init.0);
            let var_test = translate_expr(var_test.0);
            let var_change = translate_expr(var_change.0);
            let body = translate_expr(body.0);

            format!("{var_init} {var_test} {var_change} {body}")
        },
        Expr::Cast { cast_operator, operand } => {
            let cast_operator = translate_hextypedef(cast_operator.0);
            let operand = translate_expr(operand.0);

            format!("{cast_operator} {operand}")
        },
        Expr::Union { name, body, template_parameters } => {
            let name = name.0;
            let body = translate_expr(body.0);
            
            if let Some(template_parameters) = template_parameters {
                let template_parameters = translate_expr(template_parameters.0);

                format!("{name} {template_parameters} {body}")
            } else {
                format!("{name} {body}")
            }
        },
        Expr::Type { val } => {
            translate_hextypedef(val)
        },
        Expr::Match { parameters, branches } => {
            let parameters = "TODO"; // TODO
            let branches = "TODO"; // TODO

            format!("{parameters} {branches}")
        },
        Expr::TryCatch { try_block, catch_block } => {
            let try_block = translate_expr(try_block.0);
            let catch_block = "TODO"; // TODO

            format!("{try_block} {catch_block}")
        },
    }
}

fn translate_hextypedef(value_type: HexTypeDef) -> String {
    let HexTypeDef {
        endianness,
        name,
    } = value_type;

    translate_hextype(name.0)
}

fn translate_hextype(htype: HexType) -> String {
    match htype {
        HexType::Custom(htype) => htype,
        HexType::Path(a) => "None".to_string(), // TODO
        HexType::V(v) => match v {
           ValueType::CustomType => "None".to_string(), // TODO
           ValueType::Padding => "padding".to_string(), // TODO
           ValueType::Auto => "auto".to_string(),
           ValueType::U8 => "u8".to_string(),
           ValueType::U16 => "u16".to_string(),
           ValueType::U24 => "u24".to_string(),
           ValueType::U32 => "u32".to_string(),
           ValueType::U48 => "u48".to_string(),
           ValueType::U64 => "u64".to_string(),
           ValueType::U96 => "u96".to_string(),
           ValueType::U128 => "u128".to_string(),
           ValueType::S8 => "s8".to_string(),
           ValueType::S16 => "s16".to_string(),
           ValueType::S24 => "s24".to_string(),
           ValueType::S32 => "s32".to_string(),
           ValueType::S48 => "s48".to_string(),
           ValueType::S64 => "s64".to_string(),
           ValueType::S96 => "s96".to_string(),
           ValueType::S128 => "s128".to_string(),
           ValueType::Float => "Float".to_string(),
           ValueType::Double => "double".to_string(),
           ValueType::Boolean => "Bool".to_string(),
           ValueType::Character => "char".to_string(),
           ValueType::Character16 => "char16".to_string(),
           ValueType::String => "None".to_string(), // TODO
        },
        HexType::Parameted(htype, b) => translate_hextype(htype.as_ref().to_owned()), // TODO
        HexType::Null => "None".to_string(),
    }
}
