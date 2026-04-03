use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn to_py_err(e: dkdc_lm::Error) -> PyErr {
    PyErr::new::<PyRuntimeError, _>(e.to_string())
}

// -- Lifecycle ----------------------------------------------------------------

#[pyfunction]
#[pyo3(signature = (model_args, port=8080, gpu_layers=-1, ctx_size=4096))]
fn start(model_args: Vec<String>, port: u16, gpu_layers: i32, ctx_size: u32) -> PyResult<()> {
    dkdc_lm::start(&model_args, port, gpu_layers, ctx_size).map_err(to_py_err)
}

#[pyfunction]
fn stop() -> PyResult<()> {
    dkdc_lm::stop().map_err(to_py_err)
}

#[pyfunction]
#[pyo3(signature = (port=8080))]
fn status(port: u16) -> (bool, bool) {
    dkdc_lm::status(port)
}

#[pyfunction]
#[pyo3(signature = (lines=None))]
fn logs(lines: Option<usize>) -> PyResult<String> {
    dkdc_lm::logs(lines).map_err(to_py_err)
}

#[pyfunction]
fn resolve_builtin(name: &str) -> PyResult<Vec<String>> {
    dkdc_lm::resolve_builtin(name).map_err(to_py_err)
}

#[pyfunction]
fn is_running() -> bool {
    dkdc_lm::is_running()
}

// -- Constants ----------------------------------------------------------------

#[pyfunction]
fn default_port() -> u16 {
    dkdc_lm::DEFAULT_PORT
}

#[pyfunction]
fn default_builtin() -> &'static str {
    dkdc_lm::DEFAULT_BUILTIN
}

#[pyfunction]
fn tmux_session() -> &'static str {
    dkdc_lm::TMUX_SESSION
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
        m.add_function(wrap_pyfunction!(is_running, m)?)?;
        m.add_function(wrap_pyfunction!(default_port, m)?)?;
        m.add_function(wrap_pyfunction!(default_builtin, m)?)?;
        m.add_function(wrap_pyfunction!(tmux_session, m)?)?;
        Ok(())
    }
}
