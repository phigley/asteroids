use gfx;
use glutin;
use std;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScreenCreateError {
    #[error("Failed to create window")]
    GlutinFailure(#[from] glutin::CreationError),

    #[error("Pipeline create failure for {file_name}.[vert,frag] : {source:?}")]
    PipelineFailure {
        source: gfx::PipelineStateError<std::string::String>,
        file_name: &'static str,
    },
}
