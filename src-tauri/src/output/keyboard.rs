use crate::error::VoxTypeError;

pub fn type_text(_text: &str) -> Result<(), VoxTypeError> {
    #[cfg(target_os = "macos")]
    {
        Err(VoxTypeError::Output("macOS keyboard simulation not yet implemented".into()))
    }

    #[cfg(target_os = "windows")]
    {
        Err(VoxTypeError::Output("Windows keyboard simulation not yet implemented".into()))
    }

    #[cfg(target_os = "linux")]
    {
        Err(VoxTypeError::Output("Linux keyboard simulation not yet implemented".into()))
    }
}
