use std::path::PathBuf;

use expr_translator::vec_translate_statements;
use pyo3::prelude::*;

use expr_translator::StatementsContext;

mod expr_translator;

struct PyLine {
    indent_lvl: usize,
    line: String
}

enum PyLines {
    One(PyLine),
    Multiple(Vec<PyLine>),
    None,
}

impl PyLines {
    fn unwrap_one(self) -> PyLine {
        match self {
            PyLines::One(line) => line,
            PyLines::Multiple(_) => panic!("Unwrapped one on PyLines::Multiple"),
            PyLines::None => panic!("Unwrapped one on PyLines::None")
        }
    }
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

    for stmnt in vec_translate_statements(ast.0, 0, StatementsContext::None) {
        let indent = indentation.repeat(stmnt.indent_lvl as usize);
        let line = &stmnt.line;
        output_file = format!("{output_file}{indent}{line}\n");
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

fn one_py_line(lvl: usize, line: String) -> PyLines {
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
