use std::path::PathBuf;

use expr_translator::translate_expr;
use hexparser::{m_parser::{HexTypeDef, HexType, FuncArgument}, token::ValueType};
use pyo3::prelude::*;

mod expr_translator;

struct PyLine {
    indent_lvl: u32,
    line: String
}

enum PyLines {
    One(PyLine),
    Multiple(Vec<PyLine>),
    None,
}

impl IntoIterator for PyLines {
    type Item = PyLine;

    type IntoIter = std::vec::IntoIter<PyLine>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            PyLines::One(pyline) => vec![pyline].into_iter(),
            PyLines::Multiple(pylines) => pylines.into_iter(),
            PyLines::None => vec![].into_iter(),
        }
    }
}

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

    for line in translate_expr(ast.0, 0) {
        output_file += &line.line;
    }

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

fn one_py_line(lvl: u32, line: String) -> PyLines {
    PyLines::One(
        PyLine {
            indent_lvl: lvl,
            line,
        } 
    )
}

fn unkown_py_lines(mut lines: Vec<PyLine>) -> PyLines {
    match lines.len() {
        0 => PyLines::None,
        1 => PyLines::One(lines.pop().unwrap()),
        _ => PyLines::Multiple(lines)
    }
}

fn translate_arg(arg: FuncArgument, lvl: u32) -> PyLine {
    match arg {
        FuncArgument::Parameter(par) => {
            match translate_expr(par.0, lvl) {
                PyLines::One(line) => line,
                _ => unreachable!()
            }
        },
        FuncArgument::ParameterPack((pack, _)) => PyLine{indent_lvl: lvl, line: pack},
    }
}

fn translate_hextypedef(value_type: HexTypeDef, lvl: u32) -> PyLine {
    let HexTypeDef {
        endianness,
        name,
    } = value_type;

    translate_hextype(name.0, lvl)
}

fn translate_hextype(htype: HexType, lvl: u32) -> PyLine {
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
