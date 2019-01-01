use gfx;
use glutin;
use std;

quick_error! {
    #[derive(Debug)]
    pub enum ScreenCreateError {

        GlutinFailure( error: glutin::CreationError )
        {
            from()
        }

        PipelineFailure( root_name: &'static str,
            error: gfx::PipelineStateError<std::string::String> )
        {
            display("Pipeline create failure for {} : {:?}", root_name, error)
            // cause(error) -- error does not have trait std::error::Error
            context(root_name: &'static str, error: gfx::PipelineStateError<std::string::String>)
                -> (root_name, error)
        }
    }

}
