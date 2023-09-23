use std::ops::{Sub, DerefMut};

use pyo3::{prelude::*, types::{PyBytes, PyInt}, pyclass::CompareOp, ffi::PyObject_IsInstance, PyNativeType};

pub fn primitives_submodule(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let primitives = PyModule::new(py, "primitives")?;
    //primitives.add_class();
    parent_module.add_submodule(primitives)?;
    Ok(())
}

#[derive(Clone)]
#[pyclass]
struct Dollar {
    offset: usize,
    bytes: Py<PyBytes>
}

fn int(intable: &PyAny) -> PyResult<isize> {
    intable.call_method("__int__", (), None)?.extract::<isize>()
}

#[pymethods]
impl Dollar {
    #[new]
    fn new(offset: usize, bytes: Py<PyBytes>) -> Dollar {
        Dollar {
            offset,
            bytes
        }
    }

    fn read(&mut self, py: Python, amount: usize) -> &[u8] {
        let read_bytes = &self.bytes.as_bytes(py)[self.offset..self.offset+amount];
        self.offset += amount;

        read_bytes
    }

    fn read_unsigned(&self, py: Python, amount: u128) -> U128 {
        let new_dollar = self.copy();
        let new_val = py.eval(code, None, None);
        let mut new_val = new_val.__matmul__(new_dollar);
        new_val.___dollar______ = new_dollar;

        new_val
    }

    fn eof(&self, py: Python) -> bool {
        self.offset >= self.bytes.as_bytes(py).len()
    }
    
    fn copy(&self) -> Dollar {
        Dollar::new(self.offset, self.bytes.clone())
    }

    fn __index__(&self) -> usize {
        self.offset
    }

    fn __repr__(&self) -> String {
        self.offset.to_string()
    }
    
    fn __str__(&self) -> String {
        self.offset.to_string()
    }

    fn __richcmp__(&self, other: &PyAny, comp_op: CompareOp) -> PyResult<bool> {
        let offset = self.offset as isize;
        let other = int(other)?;
        Ok(match comp_op {
            CompareOp::Eq => offset == other,
            CompareOp::Ne => offset != other,
            CompareOp::Lt => offset < other,
            CompareOp::Le => offset <= other,
            CompareOp::Gt => offset > other,
            CompareOp::Ge => offset >= other,
        })
    }
    
    fn __bool__(&self) -> bool {
        self.offset != 0
    }

    fn __add__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) + int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __sub__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) - int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __mul__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) * int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __truediv__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as f32) / (int(other)? as f32);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __floordiv__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as f32) / (int(other)? as f32);

        Ok(Dollar::new(res.floor() as usize, self.bytes.clone()))
    }

    fn __mod__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) % int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __divmod__(&self, other: &PyAny) -> PyResult<Dollar> {
        self.__floordiv__(other)?.__mod__(other)
    }

    fn __pow__(&self, other: &PyAny, modulo: Option<usize>) -> PyResult<Dollar> {
        if let Some(_) = modulo {
            return PyResult::Err(pyo3::exceptions::PyNotImplementedError::new_err(()))
        }

        let other = int(other)?;
        let res = if other < 0 {
            (self.offset as f32).powi(other as i32) as isize
        } else {
            (self.offset as isize).pow(other as u32)
        };

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __lshift__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) << int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rshift__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) >> int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }
    
    fn __and__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) & int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) ^ int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __or__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (self.offset as isize) | int(other)?;

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? + (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rsub__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? - (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rmul__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? * (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rtruediv__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (int(other)? as f32) / (self.offset as f32);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rfloordiv__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = (int(other)? as f32) / (self.offset as f32);

        Ok(Dollar::new(res.floor() as usize, self.bytes.clone()))
    }

    fn __rmod__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? % (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rdivmod__<'a>(&self, other: &'a PyAny) -> PyResult<&'a PyAny> {
        let res = other.call_method("__floordiv__", (self.offset,), None)?;
        res.call_method("__mod__", (self.offset,), None)
    }

    fn __rpow__(&self, other: &PyAny, modulo: Option<usize>) -> PyResult<Dollar> {
        if let Some(_) = modulo {
            return PyResult::Err(pyo3::exceptions::PyNotImplementedError::new_err(()))
        }

        let res = int(other)?.pow(self.offset as u32);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rlshift__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? << (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rrshift__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? >> (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rand__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? & (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __rxor__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? ^ (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __ror__(&self, other: &PyAny) -> PyResult<Dollar> {
        let res = int(other)? | (self.offset as isize);

        Ok(Dollar::new(res as usize, self.bytes.clone()))
    }

    fn __iadd__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset += int(other)? as usize;

        Ok(())
    }

    fn __isub__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset -= int(other)? as usize;

        Ok(())
    }

    fn __imul__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset *= int(other)? as usize;

        Ok(())
    }

    fn __itruediv__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset /= int(other)? as usize;

        Ok(())
    }

    fn __ifloordiv__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset /= int(other)? as usize;

        Ok(())
    }

    fn __imod__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset %= int(other)? as usize;

        Ok(())
    }

    fn __ipow__(&mut self, other: &PyAny, modulo: Option<usize>) -> PyResult<()> {
        if let Some(_) = modulo {
            return PyResult::Err(pyo3::exceptions::PyNotImplementedError::new_err(()))
        }

        let other = int(other)?;
        let res = if other < 0 {
            (self.offset as f32).powi(other as i32) as isize
        } else {
            (self.offset as isize).pow(other as u32)
        };

        self.offset = res as usize;

        Ok(())
    }

    fn __ilshift__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset <<= int(other)?;
        
        Ok(())
    }

    fn __irshift__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset >>= int(other)?;

        Ok(())
    }

    fn __iand__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset &= int(other)? as usize;

        Ok(())
    }

    fn __ixor__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset ^= int(other)? as usize;

        Ok(())
    }

    fn __ior__(&mut self, other: &PyAny) -> PyResult<()> {
        self.offset |= int(other)? as usize;

        Ok(())
    }

    fn __int__(&self) -> usize {
        self.offset
    }

    fn __float__(&self) -> f32 {
        self.offset as f32
    }
}

impl Sub for Dollar {
    type Output = Dollar;

    fn sub(self, rhs: Self) -> Self::Output {
        let offset = self.offset - rhs.offset;
        Dollar::new(offset, self.bytes)
    }
}

#[pyclass(subclass)]
struct Struct {
    ____name________: String,
    ___breaked___: bool,
    __address____: Option<Dollar>,
    ___dollar______: Option<Dollar>,
    __size_______: Option<Dollar>
}

#[pymethods]
impl Struct {
    #[new]
    #[pyo3(signature = (name=""))]
    fn new(name: &str) -> Struct {
        Struct {
            ____name________: name.to_string(),
            ___breaked___: false,
            __address____: None,
            ___dollar______: None,
            __size_______: None
        }
    }

    fn init_struct(&self, starting_offset: Dollar, end_offset: Dollar) {
        self.__address____ = Some(starting_offset.copy());
        self.___dollar______ = Some(end_offset.copy());
        self.__size_______ = Some(end_offset.copy() - self.__address____.unwrap())
    }

    fn name(&self) -> &str {
        &self.____name________
    }

    fn breaked(&self) -> bool {
        self.___breaked___
    }

    fn break_(&mut self) {
        self.___breaked___ = true
    }

    fn dollar(&self) -> Option<Dollar> {
        self.___dollar______
    }

    fn size(&self) -> Option<Dollar> {
        self.__size_______
    }

    fn address(&self) -> Option<Dollar> {
        self.__address____
    }
}

#[derive(Clone)]
#[pyclass(subclass, extends=Struct)]
struct IntStruct;

#[pymethods]
impl IntStruct {
    #[new]
    #[pyo3(signature = (name=""))]
    fn new(name: &str) -> (IntStruct, Struct) {
        (IntStruct, Struct::new(name))
    }
}

macro_rules! impl_int_struct {
    ($name:ident, $byte_len:literal, $inner_type:ty) => {
        #[pyclass(subclass, extends=IntStruct)]
        struct $name {
            ___length_______: usize,
            ___value_____: $inner_type
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (value=0,name=""))]
            fn new(value: $inner_type, name: &str) -> PyClassInitializer<Self> {
                PyClassInitializer::from(
                    IntStruct::new(name)
                ).add_subclass(Self {
                    ___length_______: $byte_len,
                    ___value_____: value
                })
            }

            fn length(&self) -> usize {
                self.___length_______
            }

            fn value(&self) -> $inner_type {
                self.___value_____
            }

            fn to_dollar(self: PyRef<'_, Self>) -> Dollar {
                Dollar::new(self.___value_____ as usize, self.into_super().as_ref().___dollar______.unwrap().bytes)
            }

            fn __matmul__<'a>(mut self: PyRef<'a, Self>, other: &PyAny, py: Python) -> PyResult<PyRef<'a, Self>> {
                let mut dollar: Dollar = if other.is_instance_of::<IntStruct>() {
                    other.call_method("to_dollar", (), None).unwrap().extract().unwrap()
                } else if other.is_instance_of::<Dollar>() {
                    other.extract().unwrap()
                } else {
                    return PyResult::Err(pyo3::exceptions::PyNotImplementedError::new_err(()))
                };

                let starting_offset = dollar.copy();
                let read_value = dollar.read(py, $byte_len);
                let read_value = unsafe{read_value.as_chunks_unchecked()[0]};
                self.into_super().into_super().init_struct(starting_offset, dollar.copy());
                let a = self;
                self.___value_____ = <$inner_type>::from_le_bytes(read_value);
                Ok(self)
            }
        }
    };
}

impl_int_struct!(U8, 1, u8);
impl_int_struct!(U16, 2, u16);
//impl_int_struct!(U24, 3, u32);
impl_int_struct!(U32, 4, u32);
impl_int_struct!(U64, 8, u64);
//impl_int_struct!(U96, 12, u128);
impl_int_struct!(U128, 16, u128);
