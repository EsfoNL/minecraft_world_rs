// use pyo3::prelude::*;

// #[pymodule]
// mod nbt {
//     use either::Either;
//     use pyo3::{
//         exceptions::PyValueError,
//         prelude::*,
//         types::{PyBytes, PyString, PyTuple},
//     };

//     #[pymodule_export]
//     use crate::{NbtList, NbtValue};

//     #[pyclass]
//     struct Nbt(String, NbtValue);

//     #[pymethods]
//     impl Nbt {
//         #[new]
//         #[pyo3(signature = (obj, compressed=false))]
//         fn new(obj: Bound<'_, PyAny>, compressed: bool) -> PyResult<Self> {
//             if obj.is_instance_of::<PyBytes>() {
//                 let obj = obj.cast_into::<PyBytes>().unwrap();
//                 let (name, data) = if compressed {
//                     NbtValue::from_compressed_reader(obj.as_bytes())
//                 } else {
//                     NbtValue::from_reader(obj.as_bytes())
//                 }
//                 .map_err(|e| PyValueError::new_err(format!("reading error: {e:?}")))?;
//                 Ok(Nbt(name, data))
//             } else if obj.is_instance_of::<PyTuple>() {
//                 let tuple = obj.cast_into::<PyTuple>().unwrap();
//                 if tuple.len() != 2 {
//                     return Err(PyValueError::new_err("invalid data to construct a Nbt obj"));
//                 }

//                 let (Ok(name), Ok(value)) = (
//                     tuple
//                         .get_item(0)
//                         .and_then(|e| e.cast_into::<PyString>().map_err(Into::into))
//                         .into(),
//                     tuple
//                         .get_item(1)
//                         .and_then(|e| e.cast_into::<NbtValue>().map_err(Into::into))
//                         .into(),
//                 ) else {
//                     return Err(PyValueError::new_err("invalid data to construct a Nbt obj"));
//                 };
//                 Ok(Self(name.to_cow()?.to_string(), value.get().clone()))
//             } else {
//                 Err(PyValueError::new_err("invalid data to construct a Nbt obj"))
//             }
//         }

//         fn name(&self) -> String {
//             self.0.clone()
//         }

//         fn nbt_value(&self, py: Python<'_>) -> PyResult<Py<NbtValue>> {
//             Py::new(py, self.1.clone())
//         }

//         fn __str__(&self) -> String {
//             format!("Nbt({}: {:?})", self.0, self.1)
//         }
//     }

//     #[derive(FromPyObject)]
//     enum NbtType {
//         Byte,
//         Short,
//         Int,
//     }

//     #[pymethods]
//     impl NbtValue {
//         fn __str__(&self) -> String {
//             format!("{self:?}")
//         }

//         #[new]
//         fn new(obj: Py<PyAny>, ty: NbtType, py: Python) -> PyResult<Self> {}

//         fn __getitem__(&self, index: Either<String, usize>) -> PyResult<Self> {
//             match index {
//                 Either::Left(str_i) => match self {
//                     NbtValue::Compound(hash_map) => hash_map
//                         .get(&str_i)
//                         .ok_or(PyValueError::new_err("invalid index"))
//                         .cloned(),
//                     _ => Err(PyValueError::new_err("invalid index type")),
//                 },
//                 Either::Right(usize_i) => match self {
//                     NbtValue::ByteArray(items) => todo!(),
//                     NbtValue::String(_) => todo!(),
//                     NbtValue::List(nbt_list) => todo!(),
//                     NbtValue::IntArray(items) => todo!(),
//                     NbtValue::LongArray(items) => todo!(),
//                     _ => Err(PyValueError::new_err("invalid index type")),
//                 },
//             }
//         }
//     }
// }
