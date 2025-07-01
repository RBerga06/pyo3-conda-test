use pyo3::Python;

fn main() {
    Python::with_gil(|py| {
        println!("Python: {}", py.version());
    })
}
