use hexparser::{token::Spanned, Value, Expr, m_parser::{UnaryOp, HexTypeDef, BinaryOp, FuncArgument, MatchBranch, Statement, AssignmentOp}};

use crate::{PyLines, one_py_line, unkown_py_lines, PyLine, expr_translator::{vec_translate_exprs, translate_statement}};

use super::{translate_expr, vec_translate_statements, translate_hextype, StatementsContext};

pub(crate) fn translate_value(val: Value, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    match val {
        Value::Null => one_py_line(lvl, "None".to_string()),
        Value::Bool(b) => if b {
            one_py_line(lvl, "True".to_string())
        } else {
            one_py_line(lvl, "False".to_string())
        },
        Value::Num(n) => one_py_line(lvl, n.to_string()),
        Value::Str(s) => one_py_line(lvl, s),
        Value::Char(c) => one_py_line(lvl, c.to_string()),
        Value::Func(f) => one_py_line(lvl, f),
    }
}

pub(crate) fn translate_expr_list(list: Vec<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    unkown_py_lines(list.into_iter()
            .map(|(expr, _)| translate_expr(expr, lvl, context))
            .flatten()
            .collect::<Vec<_>>())
}

pub(crate) fn translate_unary(operation: UnaryOp, operand: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let operand = translate_expr(operand.0, lvl, context);
    let operand = operand.unwrap_one().line;

    match operation {
        UnaryOp::Add => one_py_line(lvl, format!("+{}", operand)),
        UnaryOp::Sub => one_py_line(lvl, format!("-{}", operand)),
        UnaryOp::LNot => one_py_line(lvl, format!("not {}", operand)),
        UnaryOp::BNot => one_py_line(lvl, format!("!{}", operand)),
    }
}

pub(crate) fn translate_binary(loperand: Box<Spanned<Expr>>, operator: BinaryOp, roperand: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
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
        BinaryOp::LXor => "^",
        BinaryOp::LOr => "or",
        BinaryOp::Assign(a) => match a {
            AssignmentOp::Just => "=",
            AssignmentOp::Add => "+=",
            AssignmentOp::Sub => "-=",
            AssignmentOp::Mul => "*=",
            AssignmentOp::Div => "/=",
            AssignmentOp::Mod => "%=",
            AssignmentOp::RShift => ">>=",
            AssignmentOp::LShift => "<<=",
            AssignmentOp::BOr => "|=",
            AssignmentOp::BAnd => "&=",
            AssignmentOp::BXor => "^=",
        },
    };
    let loperand = translate_expr(loperand.0, lvl, context);
    let roperand = translate_expr(roperand.0, lvl, context);

    let loperand = loperand.unwrap_one().line;
    let roperand = roperand.unwrap_one().line;

    let line = format!("{loperand} {operator} {roperand}");
    one_py_line(lvl, line)
}

pub(crate) fn translate_assignment(loperand: Box<Spanned<Expr>>, operator: AssignmentOp, roperand: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let operator = match operator {
        AssignmentOp::Just => "=",
        AssignmentOp::Add => "+=",
        AssignmentOp::Sub => "-=",
        AssignmentOp::Mul => "*=",
        AssignmentOp::Div => "/=",
        AssignmentOp::Mod => "%=",
        AssignmentOp::RShift => ">>=",
        AssignmentOp::LShift => "<<=",
        AssignmentOp::BOr => "|=",
        AssignmentOp::BAnd => "&=",
        AssignmentOp::BXor => "^=",
    };
    let loperand = translate_expr(loperand.0, lvl, context);
    let roperand = translate_expr(roperand.0, lvl, context);

    let loperand = loperand.unwrap_one().line;

    let roperand = roperand.unwrap_one().line;

    let line = format!("{loperand} {operator} {roperand}");
    one_py_line(lvl, line)
}

pub(crate) fn translate_ternary(loperand: Box<Spanned<Expr>>, moperand: Box<Spanned<Expr>>, roperand: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let loperand = translate_expr(loperand.0, lvl, context);
    let moperand = translate_expr(moperand.0, lvl+1, context);
    let roperand = translate_expr(roperand.0, lvl+1, context);

    let mut lines = vec![
        PyLine { indent_lvl: lvl, line: "if ".into() }
    ];

    let first_line = lines[0].line.clone();
    match loperand {
        PyLines::One(line) => {
            lines[0].line = format!("{first_line}, {}:", line.line);
        },
        PyLines::Multiple(py_lines) => {
            lines[0].line = format!("{first_line}(");
            lines.extend(py_lines.into_iter().map(|line| {
                PyLine { indent_lvl: lvl+1, line: line.line }
            }));
            lines.push(PyLine { indent_lvl: lvl, line: "):".into() })
        },
        PyLines::None => todo!(),
    };

    lines.extend(moperand);
    lines.push(PyLine { indent_lvl: lvl, line: "else:".into() });
    lines.extend(roperand);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_call(func_name: Box<Spanned<Expr>>, arguments: Spanned<Vec<Spanned<Expr>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let func_name = translate_expr(func_name.0, lvl, context);
    let func_name = func_name.unwrap_one().line;
    let arguments = arguments.0.into_iter()
        .map(|(expr, _)| translate_expr(expr, lvl, context))
        .map(|py_line| py_line.unwrap_one().line)
        .fold(String::new(), |old_str, expr| old_str + ", " + &expr);

    let line = format!("{func_name}({arguments})");
    one_py_line(lvl, line)
}

pub(crate) fn translate_if(test: Box<Spanned<Expr>>, consequent: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let test = translate_expr(test.0, lvl, context);
    let test = test.unwrap_one().line;
    let consequent = vec_translate_statements(consequent.0, lvl+1, StatementsContext::If);

    let mut lines = vec![PyLine { indent_lvl: lvl, line: format!("if {test}:") }];
    lines.extend(consequent.into_iter());

    PyLines::Multiple(lines)
}

pub(crate) fn translate_if_block(ifs: Spanned<Vec<Spanned<Statement>>>, alternative: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let ifs = vec_translate_statements(ifs.0, lvl, StatementsContext::IfBlock);
    let alternative = vec_translate_statements(alternative.0, lvl+1, StatementsContext::IfAlternative);

    let mut lines = ifs.into_iter()
        .collect::<Vec<_>>();

    lines.push(PyLine { indent_lvl: lvl, line: "else:".into() });
    lines.extend(alternative);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_definition(value_type: Spanned<HexTypeDef>, name: Box<Spanned<Expr>>, body: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let value_type = translate_hextypedef(value_type.0, lvl, context);
    let value_type = value_type.line;
    let name = translate_expr(name.0, lvl, context);
    let name = name.unwrap_one().line;
    let body = translate_expr(body.0, lvl, context);
    let body = body.unwrap_one().line;

    let line = format!("{name} = {value_type}({body})");

    one_py_line(lvl, line)
}

pub(crate) fn translate_array_definition(value_type: Spanned<HexTypeDef>, array_name: Box<Spanned<Expr>>, size: Box<Spanned<Expr>>, body: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let value_type = translate_hextypedef(value_type.0, lvl, context);
    let value_type = value_type.line;
    let array_name = translate_expr(array_name.0, lvl, context);
    let array_name = array_name.unwrap_one().line;
    let body = translate_expr(body.0, lvl, context);
    let body = body.unwrap_one().line;

    let line = format!("{array_name} = Array({value_type}, {body})");

    one_py_line(lvl, line)
}

pub(crate) fn translate_bitfield_entry(name: Spanned<String>, length: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let length = translate_expr(length.0, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("{name}: {length}"))
}

pub(crate) fn translate_enum_entry(name: Spanned<String>, value: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let value = translate_expr(value.0, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("{name} = {value}"))
}

pub(crate) fn translate_namespace_access(previous: Box<Spanned<Expr>>, name: Spanned<String>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let previous = translate_expr(previous.0, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("{previous}.{name}"))
}

pub(crate) fn translate_using(new_name: Spanned<String>, template_parameters: Vec<Spanned<Expr>>, old_name: Spanned<HexTypeDef>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let new_name = new_name.0;
    let old_name = translate_hextypedef(old_name.0, lvl, context).line;

    let line = if template_parameters.is_empty() {
        format!("{new_name} = {old_name}")
    } else {
        let template_parameters = vec_translate_exprs(template_parameters, lvl, context).unwrap_one().line;

        format!("{new_name} = {old_name}<{template_parameters}>")
    };

    one_py_line(lvl, line)
}

pub(crate) fn translate_return(value: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let value = translate_expr(value.0, lvl, context).unwrap_one().line;

    let line = format!("return {value}");
    one_py_line(lvl, line)
}

pub(crate) fn translate_func(name: Spanned<String>, args: Spanned<Vec<Spanned<FuncArgument>>>, body: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let args = args.0.into_iter()
        .map(|arg| translate_arg(arg.0, lvl, context))
        .map(|arg| arg.line)
        .fold(String::new(), |old, new| format!("{old}, {new}"));
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::Function);

    let mut lines = vec![
        PyLine{ indent_lvl: lvl, line: format!("def {name}({args}):") }
    ];

    lines.extend(body.into_iter());

    PyLines::Multiple(lines)
}

pub(crate) fn translate_struct(name: Spanned<String>, body: Spanned<Vec<Spanned<Statement>>>, template_parameters: Vec<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::Struct);

    /*
class {name}(Struct):
    """//TODO: This comment is not done yet
hexpat definition:
```hexpat
{docstring}
```"""
    def __init__(self, name: str=""):
        """
        struct

        Args:
            name (str, optional): The name of this instance. Can be whatever you want or just an empty string. Defaults to "".
        """
        super().__init__(name)

    def __matmul__(self, _dollar___offset):
        if not (isinstance(_dollar___offset, Dollar) or isinstance(_dollar___offset, IntStruct)):
            raise Exception(f'An object of class "Dollar" must be used with the "@" operator. {{type(_dollar___offset)}} was used instead')
        if isinstance(_dollar___offset, IntStruct):
            _dollar___offset = _dollar___offset.to_dollar()
        _dollar___offset_copy = _dollar___offset.copy()
*/
    let mut lines = if template_parameters.is_empty() {
        vec![
            PyLine{ indent_lvl: lvl, line: format!("class {name}(Struct):") },
        ]
    } else {
        let template_parameters = vec_translate_exprs(template_parameters, lvl, context).unwrap_one().line;

        vec![
            PyLine{ indent_lvl: lvl, line: format!("class {name}(Struct):#<{template_parameters}>") }
        ]
    };

    lines.extend(vec![
        PyLine{ indent_lvl: lvl+1, line: r#"def __init__(self, name: str=""):"#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"""""#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"struct"#.into() },
        PyLine{ indent_lvl: 0, line: r#""#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"Args"#.into() },
        PyLine{ indent_lvl: lvl+3, line: r#"name (str, optional): The name of this instance. Can be whatever you want or just an empty string. Defaults to ""."#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"""""#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"super().__init__(name)"#.into() },
        PyLine{ indent_lvl: lvl+1, line: r#"def __matmul__(self, _dollar___offset):"#.into() },

        PyLine{ indent_lvl: lvl+2, line: r#"if not (isinstance(_dollar___offset, Dollar) or isinstance(_dollar___offset, IntStruct)):"#.into() },
        PyLine{ indent_lvl: lvl+3, line: r#"raise Exception(f'An object of class "Dollar" must be used with the "@" operator. {{type(_dollar___offset)}} was used instead')"#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"if isinstance(_dollar___offset, IntStruct):"#.into() },
        PyLine{ indent_lvl: lvl+3, line: r#"_dollar___offset = _dollar___offset.to_dollar()"#.into() },
        PyLine{ indent_lvl: lvl+2, line: r#"_dollar___offset_copy = _dollar___offset.copy()"#.into() },
    ]);

    lines.extend(body.into_iter());

    PyLines::Multiple(lines)
}

pub(crate) fn translate_namespace(name: Box<Spanned<Expr>>, body: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = translate_expr(name.0, lvl, context).unwrap_one().line;
    let body = vec_translate_statements(body.0, lvl, StatementsContext::Namespace);

    let mut lines = vec![
        PyLine{ indent_lvl: lvl, line: name}
    ];

    lines.extend(body.into_iter());

    PyLines::Multiple(lines)
}

pub(crate) fn translate_enum(name: Spanned<String>, value_type: Spanned<HexTypeDef>, body: Spanned<Vec<Spanned<Expr>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let value_type = translate_hextypedef(value_type.0, lvl, context).line;
    let body = vec_translate_exprs(body.0, lvl+1, context);

    let mut lines = vec![
        PyLine { indent_lvl: lvl, line: format!("enum {name} {value_type}") }
    ];

    lines.extend(body);
    PyLines::Multiple(lines)
}

pub(crate) fn translate_bitfield(name: Spanned<String>, body: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::Bitfield);

    let mut lines = vec![
        PyLine { indent_lvl: lvl, line: name }
    ];

    lines.extend(body);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_access(item: Box<Spanned<Expr>>, member: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let item = translate_expr(item.0, lvl, context).unwrap_one().line;
    let member = translate_expr(member.0, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("{item}.{member}"))
}

pub(crate) fn translate_array_access(array: Box<Spanned<Expr>>, index: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let array = translate_expr(array.0, lvl, context).unwrap_one().line;
    let index = translate_expr(index.0, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("{array}[{index}]"))
}

pub(crate) fn translate_attribute(arguments: Spanned<Vec<Spanned<Expr>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    one_py_line(lvl, arguments.0.into_iter()
        .map(|(arg, _)| translate_expr(arg, lvl, context))
        .map(|arg| match arg {
            PyLines::One(arg) => arg.line,
            _ => todo!()
        }).fold(String::new(), |old, new| {
            format!("{old}, {new}")
        }))
}

pub(crate) fn translate_attribute_arguument(name: Box<Spanned<Expr>>, value: Vec<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = translate_expr(name.0, lvl, context).unwrap_one().line;
    let value = vec_translate_exprs(value, lvl, context).unwrap_one().line;

    one_py_line(lvl, format!("#[{name}={value}]"))
}

pub(crate) fn translate_while_loop(condition: Box<Spanned<Expr>>, body: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let condition = translate_expr(condition.0, lvl, context).unwrap_one().line;
    let body = translate_expr(body.0, lvl+1, context);

    let mut lines = vec![
        PyLine { indent_lvl: lvl, line: format!("while {condition}:") }
    ];

    lines.extend(body);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_while_loop_statement(condition: Box<Spanned<Expr>>, body: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let condition = translate_expr(condition.0, lvl, context).unwrap_one().line;
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::WhileLoop);

    let mut lines = vec![
        PyLine { indent_lvl: lvl, line: format!("while {condition}:") }
    ];

    lines.extend(body);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_for_loop(var_init: Box<Spanned<Statement>>, var_test: Box<Spanned<Expr>>, var_change: Box<Spanned<Statement>>, body: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let var_init = translate_statement(var_init.0, lvl, context).unwrap_one();
    let var_test = translate_expr(var_test.0, lvl, context).unwrap_one().line;
    let var_change = translate_statement(var_change.0, lvl+1, context).unwrap_one();
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::ForLoop);

    let mut lines = vec![
        var_init,
        PyLine { indent_lvl: lvl, line: format!("while {var_test}:") }
    ];

    lines.extend(body);
    lines.push(var_change);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_cast(cast_operator: Spanned<HexTypeDef>, operand: Box<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let cast_operator = translate_hextypedef(cast_operator.0, lvl, context).line;
    let operand = translate_expr(operand.0, lvl, context).unwrap_one().line;

    let line = format!("{cast_operator}({operand})");
    one_py_line(lvl, line)
}

pub(crate) fn translate_union(name: Spanned<String>, body: Spanned<Vec<Spanned<Statement>>>, template_parameters: Vec<Spanned<Expr>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let name = name.0;
    let body = vec_translate_statements(body.0, lvl+1, StatementsContext::Union);

    let mut lines = if template_parameters.is_empty() {
        vec![
            PyLine { indent_lvl: lvl, line: name }
        ]
    } else {
        let template_parameters = vec_translate_exprs(template_parameters, lvl, context).unwrap_one().line;
        vec![
            PyLine { indent_lvl: lvl, line: format!("{name}<{template_parameters}>") }
        ]
    };

    lines.extend(body);

    PyLines::Multiple(lines)
}

pub(crate) fn translate_match(parameters: Vec<Spanned<Expr>>, branches: Vec<MatchBranch>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let parameters = "TODO"; // TODO
    let branches = "TODO"; // TODO

    one_py_line(lvl, format!("{parameters} {branches}"))
}

pub(crate) fn translate_try_catch(try_block: Spanned<Vec<Spanned<Statement>>>, catch_block: Spanned<Vec<Spanned<Statement>>>, lvl: usize, context: StatementsContext) -> PyLines {
    panic!("Didn't take context into account");
    let try_block = vec_translate_statements(try_block.0, lvl, StatementsContext::Try);
    let catch_block = vec![]; // TODO

    let mut lines = vec![
        PyLine{indent_lvl: lvl, line: "try:".to_string()}
    ];
    lines.extend(try_block.into_iter());
    lines.extend(vec![
        PyLine{indent_lvl: lvl, line: "except Exception:".to_string()}
    ]);
    lines.extend(catch_block.into_iter());

    PyLines::Multiple(lines)
}

fn translate_arg(arg: FuncArgument, lvl: usize, context: StatementsContext) -> PyLine {
    panic!("Didn't take context into account");
    match arg {
        FuncArgument::Parameter(par) => {
            match translate_expr(par.0, lvl, context) {
                PyLines::One(line) => line,
                _ => unreachable!()
            }
        },
        FuncArgument::ParameterPack((pack, _)) => PyLine{indent_lvl: lvl, line: pack},
    }
}

pub(crate) fn translate_hextypedef(value_type: HexTypeDef, lvl: usize, context: StatementsContext) -> PyLine {
    panic!("Didn't take context into account");
    let HexTypeDef {
        endianness,
        name,
    } = value_type;

    translate_hextype(name.0, lvl, context)
}
