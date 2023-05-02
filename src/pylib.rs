use core::panic;
use pyo3::{
    exceptions::{PyIndexError, PyKeyError, PyNotImplementedError, PyTypeError},
    prelude::*,
};
use std::sync::{Arc, Mutex};

use crate::{NbtList, NbtValue};

#[pyclass(mapping)]
pub struct PyNbt {
    name: String,
    nbt: Arc<Mutex<NbtValue>>,
}

#[pyclass(mapping)]
pub struct PyNbtList {
    name: String,
    list: Arc<Mutex<NbtList>>,
}

impl PyNbtList {
    fn new(name: String, list: Arc<Mutex<NbtList>>) -> Self {
        PyNbtList { name, list }
    }
}

#[pymethods]
impl PyNbt {
    fn __getitem__(slf: PyRef<'_, Self>, obj: &PyAny) -> PyResult<Py<PyAny>> {
        let lock = slf.nbt.lock().unwrap();
        match *lock {
            NbtValue::ByteArray(ref v) => Ok(v
                .get(obj.extract::<usize>()?)
                .ok_or(PyIndexError::new_err("index out of range"))?
                .to_owned()
                .to_object(slf.py())),
            NbtValue::List(ref v) => {
                let lock = v.lock().unwrap();
                let index: usize = obj.extract()?;
                let error = PyIndexError::new_err("index out of range");
                Ok(match *lock {
                    NbtList::ByteList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::ShortList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::IntList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::LongList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::FloatList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::DoubleList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::ByteArrayList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::StringList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::ListList(ref v) => {
                        PyNbtList::new(String::new(), v.get(index).ok_or(error)?.clone())
                            .into_py(slf.py())
                            .to_object(slf.py())
                    }
                    NbtList::CompoundList(ref v) => PyNbt::new(
                        String::new(),
                        Arc::new(Mutex::new(NbtValue::Compound(
                            v.get(index).ok_or(error)?.clone(),
                        ))),
                    )
                    .into_py(slf.py())
                    .to_object(slf.py()),
                    NbtList::IntArrayList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::LongArrayList(ref v) => v.get(index).ok_or(error)?.to_object(slf.py()),
                    NbtList::EmptyList => Err(error)?,
                    _ => panic!(),
                })
            }
            NbtValue::Compound(ref v) => {
                let name: String = obj.extract()?;
                let value = v
                    .get(&name)
                    .ok_or(PyKeyError::new_err("wrong key"))?
                    .clone();
                Ok(PyNbt::new(name, value).into_py(slf.py()))
            }
            NbtValue::IntArray(ref v) => Ok(v.to_owned().to_object(slf.py())),
            NbtValue::LongArray(ref v) => Ok(v.to_owned().to_object(slf.py())),
            _ => Err(PyTypeError::new_err("not an array or map")),
        }
    }
    fn __setitem__(slf: PyRef<'_, Self>, obj: &PyAny, value: &PyAny) -> PyResult<()> {
        let mut lock = slf.nbt.lock().unwrap();
        let error = PyIndexError::new_err("index out of range");
        match *lock {
            NbtValue::ByteArray(ref mut v) => {
                *(v.get_mut(obj.extract::<usize>()?).ok_or(error)?) = value.extract()?;
            }
            NbtValue::List(ref mut v) => {
                let mut lock = v.lock().unwrap();
                let index: usize = obj.extract()?;
                match *lock {
                    NbtList::ByteList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }
                    NbtList::ShortList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }
                    NbtList::IntList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::LongList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::FloatList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::DoubleList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::ByteArrayList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::StringList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::ListList(ref mut v) => {}

                    NbtList::CompoundList(ref mut v) => {
                        let val = v.get_mut(index).ok_or(error)?;
                        Err(PyNotImplementedError::new_err(
                            "compound setting not supported",
                        ))?
                    }

                    NbtList::IntArrayList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::LongArrayList(ref mut v) => {
                        *(v.get_mut(index)).ok_or(error)? = value.extract()?;
                    }

                    NbtList::EmptyList => Err(error)?,
                }
            }
            NbtValue::Compound(ref mut v) => {
                match *(v.get_mut(&obj.extract::<String>()?))
                    .ok_or(error)?
                    .lock()
                    .unwrap()
                {
                    NbtValue::Byte(ref mut v) => *v = value.extract()?,
                    NbtValue::Short(ref mut v) => *v = value.extract()?,
                    NbtValue::Int(ref mut v) => *v = value.extract()?,
                    NbtValue::Long(ref mut v) => *v = value.extract()?,
                    NbtValue::Float(ref mut v) => *v = value.extract()?,
                    NbtValue::Double(ref mut v) => *v = value.extract()?,
                    NbtValue::String(ref mut v) => *v = value.extract()?,
                    _ => Err(PyNotImplementedError::new_err(
                        "list setting is not implemented yet",
                    ))?,
                }
            }
            NbtValue::IntArray(ref mut v) => {
                *(v.get_mut(obj.extract::<usize>()?).ok_or(error)?) = value.extract()?;
            }
            NbtValue::LongArray(ref mut v) => {
                *(v.get_mut(obj.extract::<usize>()?).ok_or(error)?) = value.extract()?;
            }
            _ => Err(PyTypeError::new_err("not an array or map"))?,
        }
        Ok(())
    }

    fn __str__(slf: PyRef<'_, Self>) -> String {
        format!("{}: {:?}", slf.name, *slf.nbt.lock().unwrap())
    }
    fn __repr__(slf: PyRef<'_, Self>) -> String {
        format!("{}: {:?}", slf.name, *slf.nbt.lock().unwrap())
    }
}

impl PyNbt {
    fn new(name: String, nbt: Arc<Mutex<NbtValue>>) -> Self {
        PyNbt { name, nbt }
    }
}

#[pyfunction]
pub fn from_compressed_file(path: String) -> PyNbt {
    let file = std::fs::File::open(path).expect("failed to open file");
    let (name, nbt) = NbtValue::from_compressed_reader(file).expect("failed to parse");
    PyNbt::new(name, nbt).into()
}

#[pymodule]
fn nbt(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_compressed_file, m)?)?;
    Ok(())
}
