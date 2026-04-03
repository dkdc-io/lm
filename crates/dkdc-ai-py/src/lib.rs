use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn to_py_err(e: impl std::fmt::Display) -> PyErr {
    PyErr::new::<PyRuntimeError, _>(e.to_string())
}

// -- Lifecycle ----------------------------------------------------------------

#[pyfunction]
#[pyo3(signature = (model_args, port=8080, gpu_layers=-1, ctx_size=4096))]
fn start(
    model_args: Vec<String>,
    port: u16,
    gpu_layers: i32,
    ctx_size: u32,
) -> PyResult<()> {
    let refs: Vec<&str> = model_args.iter().map(|s| s.as_str()).collect();
    dkdc_ai::start(&refs, port, gpu_layers, ctx_size).map_err(to_py_err)
}

#[pyfunction]
fn stop() -> PyResult<()> {
    dkdc_ai::stop().map_err(to_py_err)
}

#[pyfunction]
#[pyo3(signature = (port=8080))]
fn status(port: u16) -> (bool, bool) {
    dkdc_ai::status(port)
}

#[pyfunction]
#[pyo3(signature = (lines=None))]
fn logs(lines: Option<usize>) -> PyResult<String> {
    dkdc_ai::logs(lines).map_err(to_py_err)
}

#[pyfunction]
fn resolve_builtin(name: &str) -> PyResult<Vec<String>> {
    dkdc_ai::resolve_builtin(name).map_err(to_py_err)
}

// -- Constants ----------------------------------------------------------------

#[pyfunction]
fn default_port() -> u16 {
    dkdc_ai::DEFAULT_PORT
}

#[pyfunction]
fn default_builtin() -> &'static str {
    dkdc_ai::DEFAULT_BUILTIN
}

#[pyfunction]
fn tmux_session() -> &'static str {
    dkdc_ai::TMUX_SESSION
}

// -- Module -------------------------------------------------------------------

#[pymodule]
mod core {
    use super::*;

    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(start, m)?)?;
        m.add_function(wrap_pyfunction!(stop, m)?)?;
        m.add_function(wrap_pyfunction!(status, m)?)?;
        m.add_function(wrap_pyfunction!(logs, m)?)?;
        m.add_function(wrap_pyfunction!(resolve_builtin, m)?)?;
        m.add_function(wrap_pyfunction!(default_port, m)?)?;
        m.add_function(wrap_pyfunction!(default_builtin, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_session, m)?)?;
        Ok(())
    }
}
