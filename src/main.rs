#![windows_subsystem = "windows"]

use custom_error::custom_error;

mod cropper;
mod focuser;
mod hotkey;
mod msgbox;
mod screengrab;

use cropper::Cropper;
use screengrab::Screenshot;

custom_error! { ScreenshotError
    Cropping{source: cropper::CropperError} = "error while cropping: {source:?}",
}

fn main() -> Result<(), ScreenshotError> {
    // set the process to be DPI unaware
    // if cfg!(windows) {
    //     unsafe {
    //         winapi::um::shellscalingapi::SetProcessDpiAwareness(
    //             winapi::um::shellscalingapi::PROCESS_DPI_UNAWARE,
    //         );
    //     }
    // }

    // create the cropper
    let mut cropper = Cropper::new()?;

    // hotkey::register(true, || {
    //     // get screenshot
    //     match cropper.apply(Screenshot::take()) {
    //         Err(e) => {
    //             msgbox::error(&format!("{:?}", e));
    //             true
    //         }
    //         Ok(should_quit) => should_quit,
    //     }
    // });
    let a = match cropper.apply(Screenshot::take()) {
        Err(e) => {
            msgbox::error(&format!("{:?}", e));
            true
        }
        Ok(should_quit) => should_quit,
    };

    msgbox::error(&format!("{:?}", "aaaaaaa"));

    Ok(())
}
