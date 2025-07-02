#[pyo3::pymodule(name="pyo3_conda_test")]
mod py {
    use pyo3::{pyclass, pymethods};

    #[pyclass]
    /// A Rust-managed counter that cannot be decremented
    #[derive(Clone, Debug)]
    pub struct Counter { counter: usize }
    #[pymethods]
    impl Counter {
        #[new]
        #[pyo3(signature = (value = 0, /))]
        pub fn new(value: usize) -> Self { Self { counter: value } }
        #[pyo3(signature = (*, by = 1))]
        fn increment(&mut self, by: usize) { self.counter += by }
        #[getter]
        fn value(&self) -> usize { self.counter }
    }
}

fn main() {
    pyo3::append_to_inittab!(py);
    pyo3::Python::with_gil(|py| {
        println!("Python: {}", py.version());
        py.run(pyo3::ffi::c_str!(r#"
from pyo3_conda_test import Counter
c = Counter()
assert c.value == 0
c.increment()
assert c.value == 1
c.increment(by=42)
assert c.value == 43
help(Counter)
import sys
sys.stdout.flush()
        "#), None, None)
    }).unwrap()
}
