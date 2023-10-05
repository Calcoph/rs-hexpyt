use hexparser::Expr;

use crate::{PyLines, one_py_line, translate_hextype, translate_hextypedef};

use self::translators::{translate_value, translate_expr_list, translate_unary, translate_binary, translate_ternary, translate_call, translate_if, translate_if_block, translate_definition, translate_array_definition, translate_bitfield_entry, translate_enum_entry, translate_namespace_access, translate_using, translate_return, translate_func, translate_struct, translate_namespace, translate_enum, translate_bitfield, translate_access, translate_array_access, translate_attribute, translate_attribute_arguument, translate_while_loop, translate_for_loop, translate_cast, translate_union, translate_match, translate_try_catch};

mod translators;

pub(crate) fn translate_expr(expr: Expr, lvl: u32) -> PyLines {
    match expr {
        Expr::Error => one_py_line(lvl, "raise Error".to_string()),
        Expr::Value { val } => translate_value(val, lvl),
        Expr::ExprList { list } => translate_expr_list(list, lvl),
        Expr::UnnamedParameter { type_ } => PyLines::One(translate_hextype(type_.0, lvl)),
        Expr::Local { name } => one_py_line(lvl, name.0),
        Expr::Unary { operation, operand } => translate_unary(operation, operand, lvl),
        Expr::Binary { loperand, operator, roperand } => translate_binary(loperand, operator, roperand, lvl),
        Expr::Ternary { loperand, moperand, roperand } => translate_ternary(loperand, moperand, roperand, lvl),
        Expr::Call { func_name, arguments } => translate_call(func_name, arguments, lvl),
        Expr::If { test, consequent } => translate_if(test, consequent, lvl),
        Expr::IfBlock { ifs, alternative } => translate_if_block(ifs, alternative, lvl),
        Expr::Definition { value_type, name, body } => translate_definition(value_type, name, body, lvl),
        Expr::ArrayDefinition { value_type, array_name, size, body } => translate_array_definition(value_type, array_name, size, body, lvl),
        Expr::BitFieldEntry { name, length } => translate_bitfield_entry(name, length, lvl),
        Expr::EnumEntry { name, value } => translate_enum_entry(name, value, lvl),
        Expr::NamespaceAccess { previous, name } => translate_namespace_access(previous, name, lvl),
        Expr::Using { new_name, template_parameters, old_name } => translate_using(new_name, template_parameters, old_name, lvl),
        Expr::Return { value } => translate_return(value, lvl),
        Expr::Continue => one_py_line(lvl, "continue".to_string()),
        Expr::Break => one_py_line(lvl, "break".to_string()),
        Expr::Func { name, args, body } => translate_func(name, args, body, lvl),
        Expr::Struct { name, body, template_parameters } => translate_struct(name, body, template_parameters, lvl),
        Expr::Namespace { name, body } => translate_namespace(name, body, lvl),
        Expr::Enum { name, value_type, body } => translate_enum(name, value_type, body, lvl),
        Expr::Bitfield { name, body } => translate_bitfield(name, body, lvl),
        Expr::Access { item, member } => translate_access(item, member, lvl),
        Expr::ArrayAccess { array, index } => translate_array_access(array, index, lvl),
        Expr::Attribute { arguments } => translate_attribute(arguments, lvl),
        Expr::AttributeArgument { name, value } => translate_attribute_arguument(name, value, lvl),
        Expr::WhileLoop { condition, body } => translate_while_loop(condition, body, lvl),
        Expr::ForLoop { var_init, var_test, var_change, body } => translate_for_loop(var_init, var_test, var_change, body, lvl),
        Expr::Cast { cast_operator, operand } => translate_cast(cast_operator, operand, lvl),
        Expr::Union { name, body, template_parameters } => translate_union(name, body, template_parameters, lvl),
        Expr::Type { val } => PyLines::One(translate_hextypedef(val, lvl)),
        Expr::Match { parameters, branches } => translate_match(parameters, branches, lvl),
        Expr::TryCatch { try_block, catch_block } => translate_try_catch(try_block, catch_block, lvl),
    }
}
