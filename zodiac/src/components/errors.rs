
#[derive(Debug)]
pub enum ZodiacError {
    FailedToRender(RendererError)
}

impl From<RendererError> for ZodiacError {
    fn from(error: RendererError) -> Self {
        ZodiacError::FailedToRender(error)
    }
}

#[derive(Debug)]
pub enum RendererError {
    FailedToDisplayWindow,
    FailedToCreateShaders(String),
    FailedToLoadFont,
    BufferSwapError,
    BufferCreationError,
    DrawError
}