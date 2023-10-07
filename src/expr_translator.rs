use hexparser::{Expr, m_parser::{Statement, FuncCall, Definition, HexType}, token::{Spanned, ValueType}};

use crate::{PyLines, one_py_line, PyLine, unkown_py_lines};

use self::translators::{translate_value, translate_expr_list, translate_unary, translate_binary, translate_ternary, translate_call, translate_if, translate_if_block, translate_definition, translate_array_definition, translate_bitfield_entry, translate_enum_entry, translate_namespace_access, translate_using, translate_return, translate_func, translate_struct, translate_namespace, translate_enum, translate_bitfield, translate_access, translate_array_access, translate_attribute, translate_attribute_arguument, translate_while_loop, translate_for_loop, translate_cast, translate_union, translate_match, translate_try_catch, translate_assignment, translate_while_loop_statement, translate_hextypedef};

mod translators;

fn translate_expr(expr: Expr, lvl: usize) -> PyLines {
    match expr {
        Expr::Error => one_py_line(lvl, "raise Error".to_string()),
        Expr::Value { val } => translate_value(val, lvl),
        Expr::ExprList { list } => translate_expr_list(list, lvl),
        Expr::UnnamedParameter { type_ } => PyLines::One(translate_hextype(type_.0, lvl)),
        Expr::Local { name } => one_py_line(lvl, name.0),
        Expr::Unary { operation, operand } => translate_unary(operation, operand, lvl),
        Expr::Binary { loperand, operator, roperand } => translate_binary(loperand, operator, roperand, lvl),
        Expr::Ternary { loperand, moperand, roperand } => translate_ternary(loperand, moperand, roperand, lvl),
        Expr::Call(FuncCall { func_name, arguments }) => translate_call(func_name, arguments, lvl),
        Expr::Definition(Definition { value_type, name, body }) => translate_definition(value_type, name, body, lvl),
        Expr::EnumEntry { name, value } => translate_enum_entry(name, value, lvl),
        Expr::NamespaceAccess { previous, name } => translate_namespace_access(previous, name, lvl),
        Expr::Access { item, member } => translate_access(item, member, lvl),
        Expr::ArrayAccess { array, index } => translate_array_access(array, index, lvl),
        Expr::Attribute { arguments } => translate_attribute(arguments, lvl),
        Expr::AttributeArgument { name, value } => translate_attribute_arguument(name, value, lvl),
        Expr::WhileLoop { condition, body } => translate_while_loop(condition, body, lvl),
        Expr::Cast { cast_operator, operand } => translate_cast(cast_operator, operand, lvl),
        Expr::Type { val } => PyLines::One(translate_hextypedef(val, lvl)),
    }
}

fn translate_statement(stmnt: Statement, lvl: usize) -> PyLines {
    match stmnt {
        Statement::Call(FuncCall { func_name, arguments }) => translate_call(func_name, arguments, lvl),
        Statement::If { test, consequent } => translate_if(test, consequent, lvl),
        Statement::IfBlock { ifs, alternative } => translate_if_block(ifs, alternative, lvl),
        Statement::ArrayDefinition { value_type, array_name, size, body } => translate_array_definition(value_type, array_name, size, body, lvl),
        Statement::BitFieldEntry { name, length } => translate_bitfield_entry(name, length, lvl),
        Statement::Using { new_name, template_parameters, old_name } => translate_using(new_name, template_parameters, old_name, lvl),
        Statement::Return { value } => translate_return(value, lvl),
        Statement::Continue => one_py_line(lvl, "continue".to_string()),
        Statement::Break => one_py_line(lvl, "break".to_string()),
        Statement::Func { name, args, body } => translate_func(name, args, body, lvl),
        Statement::Struct { name, body, template_parameters } => translate_struct(name, body, template_parameters, lvl),
        Statement::Namespace { name, body } => translate_namespace(name, body, lvl),
        Statement::Enum { name, value_type, body } => translate_enum(name, value_type, body, lvl),
        Statement::Bitfield { name, body } => translate_bitfield(name, body, lvl),
        Statement::ForLoop { var_init, var_test, var_change, body } => translate_for_loop(var_init, var_test, var_change, body, lvl),
        Statement::Union { name, body, template_parameters } => translate_union(name, body, template_parameters, lvl),
        Statement::Match { parameters, branches } => translate_match(parameters, branches, lvl),
        Statement::TryCatch { try_block, catch_block } => translate_try_catch(try_block, catch_block, lvl),
        Statement::Definition(Definition { value_type, name, body }) => translate_definition(value_type, name, body, lvl),
        Statement::Assignment { loperand, operator, roperand } => translate_assignment(loperand, operator, roperand, lvl),
        Statement::Error => one_py_line(lvl, "raise Error".to_string()),
        Statement::WhileLoop { condition, body } => translate_while_loop_statement(condition, body, lvl),
        Statement::Padding { padding_body } => translate_expr(padding_body.0, lvl),
    }
}

pub(crate) fn vec_translate_statements(stmnts: Vec<Spanned<Statement>>, lvl: usize) -> PyLines {
    let mut lines = Vec::new();
    for stmnt in stmnts {
        lines.extend(translate_statement(stmnt.0, lvl))
    }

    unkown_py_lines(lines)
}

pub(crate) fn vec_translate_exprs(exprs: Vec<Spanned<Expr>>, lvl: usize) -> PyLines {
    let mut lines = Vec::new();
    for expr in exprs {
        lines.extend(translate_expr(expr.0, lvl))
    }

    unkown_py_lines(lines)
}

fn translate_hextype(htype: HexType, lvl: usize) -> PyLine {
    match htype {
        HexType::Custom(htype) => PyLine {indent_lvl: lvl, line: htype},
        HexType::Path(a) => PyLine {indent_lvl: lvl, line: "None".to_string()}, // TODO
        HexType::V(v) => match v {
           ValueType::CustomType => PyLine {indent_lvl: lvl, line: "None".to_string()}, // TODO
           ValueType::Padding => PyLine {indent_lvl: lvl, line: "padding".to_string()}, // TODO
           ValueType::Auto => PyLine {indent_lvl: lvl, line: "auto".to_string()},
           ValueType::U8 => PyLine {indent_lvl: lvl, line: "u8".to_string()},
           ValueType::U16 => PyLine {indent_lvl: lvl, line: "u16".to_string()},
           ValueType::U24 => PyLine {indent_lvl: lvl, line: "u24".to_string()},
           ValueType::U32 => PyLine {indent_lvl: lvl, line: "u32".to_string()},
           ValueType::U48 => PyLine {indent_lvl: lvl, line: "u48".to_string()},
           ValueType::U64 => PyLine {indent_lvl: lvl, line: "u64".to_string()},
           ValueType::U96 => PyLine {indent_lvl: lvl, line: "u96".to_string()},
           ValueType::U128 => PyLine {indent_lvl: lvl, line: "u128".to_string()},
           ValueType::S8 => PyLine {indent_lvl: lvl, line: "s8".to_string()},
           ValueType::S16 => PyLine {indent_lvl: lvl, line: "s16".to_string()},
           ValueType::S24 => PyLine {indent_lvl: lvl, line: "s24".to_string()},
           ValueType::S32 => PyLine {indent_lvl: lvl, line: "s32".to_string()},
           ValueType::S48 => PyLine {indent_lvl: lvl, line: "s48".to_string()},
           ValueType::S64 => PyLine {indent_lvl: lvl, line: "s64".to_string()},
           ValueType::S96 => PyLine {indent_lvl: lvl, line: "s96".to_string()},
           ValueType::S128 => PyLine {indent_lvl: lvl, line: "s128".to_string()},
           ValueType::Float => PyLine {indent_lvl: lvl, line: "Float".to_string()},
           ValueType::Double => PyLine {indent_lvl: lvl, line: "double".to_string()},
           ValueType::Boolean => PyLine {indent_lvl: lvl, line: "Bool".to_string()},
           ValueType::Character => PyLine {indent_lvl: lvl, line: "char".to_string()},
           ValueType::Character16 => PyLine {indent_lvl: lvl, line: "char16".to_string()},
           ValueType::String => PyLine {indent_lvl: lvl, line: "None".to_string()}, // TODO
        },
        HexType::Parameted(htype, b) => translate_hextype(htype.as_ref().to_owned(), lvl), // TODO
        HexType::Null => PyLine {indent_lvl: lvl, line: "None".to_string()},
    }
}
