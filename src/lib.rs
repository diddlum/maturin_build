#[cfg(feature = "python")]
use pyo3::prelude::*;

pub fn test() {
    println!("-----------");
}

#[cfg(feature = "python")]
#[pymodule]
fn maturin_build(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
