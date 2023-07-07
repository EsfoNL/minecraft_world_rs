use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "nbt")]
fn nbt(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    Ok(())
}
