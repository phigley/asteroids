use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScreenCreateError {
    #[error("Failed to create window")]
    WindowCreateFailure(#[source] winit::error::OsError),

    #[error("Failed to create adapter")]
    AdapterCreateFailure,

    #[error("Pipeline create failure for {file_name}.")]
    PipelineFailure {
        source: std::io::Error,
        file_name: &'static str,
    },
}
