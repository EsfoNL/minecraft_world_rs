use pyo3::{
    exceptions::{PyIOError, PyIndexError, PyKeyError, PyNotImplementedError, PyTypeError},
    prelude::*,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{NbtList, NbtValue};

#[pyclass(mapping)]
pub struct PyNbt {
    name: String,
    nbt: Arc<Mutex<NbtValue>>,
    iter: Mutex<Option<std::vec::IntoIter<PyNbt>>>,
}

#[allow(unused)]
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
    fn __repr__(slf: PyRef<'_, Self>) -> String {
        format!("{}: {:?}", slf.name, *slf.nbt.lock().unwrap())
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

                    NbtList::ListList(ref mut v) => {
                        let _val = v.get_mut(index).ok_or(error)?;
                        Err(PyNotImplementedError::new_err("list setting not supported"))?
                    }

                    NbtList::CompoundList(ref mut v) => {
                        let _val = v.get_mut(index).ok_or(error)?;
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

    #[new]
    #[pyo3(signature = (path, compressed=true))]
    fn from_file(path: String, compressed: bool) -> PyResult<Self> {
        let file = std::fs::File::open(path)?;
        let (name, nbt) = (if compressed {
            NbtValue::from_compressed_reader(file)
        } else {
            NbtValue::from_reader(file)
        })
        .map_err(|e| PyIOError::new_err(e.to_string()))?;
        Ok(PyNbt::new(name, nbt))
    }

    fn set(slf: PyRef<'_, Self>, obj: &PyAny) -> PyResult<()> {
        let mut lock = slf.nbt.lock().unwrap();
        match *lock {
            NbtValue::Byte(ref mut v) => *v = obj.extract()?,
            NbtValue::Short(ref mut v) => *v = obj.extract()?,
            NbtValue::Int(ref mut v) => *v = obj.extract()?,
            NbtValue::Long(ref mut v) => *v = obj.extract()?,
            NbtValue::Float(ref mut v) => *v = obj.extract()?,
            NbtValue::Double(ref mut v) => *v = obj.extract()?,
            NbtValue::String(ref mut v) => *v = obj.extract()?,
            _ => Err(PyNotImplementedError::new_err(
                "compound and list assignment not implemented yet",
            ))?,
        }
        Ok(())
    }

    fn name<'a>(slf: PyRef<'a, Self>) -> String {
        slf.name.clone()
    }

    #[pyo3(signature = (path, compressed=true))]
    fn to_file(slf: PyRef<'_, Self>, path: String, compressed: bool) -> PyResult<()> {
        let mut file = std::fs::File::create(path)?;
        let lock = slf.nbt.lock().unwrap();
        if compressed {
            Ok(lock.to_compressed_writer(&slf.name, &mut file)?)
        } else {
            Ok(lock.to_writer(&slf.name, &mut file)?)
        }
    }
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        let mut lock = slf.nbt.lock().unwrap();
        match *lock {
            NbtValue::Compound(ref mut v) => {
                let mut lock = slf.iter.lock().unwrap();
                *lock = Some(
                    v.iter()
                        .map(|f| PyNbt::new(f.0.to_owned(), f.1.clone()))
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
            }
            _ => Err(PyNotImplementedError::new_err(
                "not yet implemented for non-compounds",
            ))?,
        }
        drop(lock);
        Ok(slf)
    }
    fn __next__(slf: PyRef<'_, Self>) -> Option<PyNbt> {
        if let Some(ref mut v) = *slf.iter.lock().unwrap() {
            v.next()
        } else {
            None
        }
    }
}

impl PyNbt {
    fn new(name: String, nbt: Arc<Mutex<NbtValue>>) -> Self {
        PyNbt {
            name,
            nbt,
            iter: Mutex::new(None),
        }
    }
}

#[pymodule]
#[pyo3(name = "nbt")]
fn nbt(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNbt>()?;
    m.add_class::<PyNbtList>()?;
    Ok(())
}
