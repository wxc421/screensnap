use custom_error::custom_error;
use crate::cropper::Cropper;
use crate::screengrab::Screenshot;
use crate::msgbox;


// cross build --target x86_64-pc-windows-msvc
// cargo xwin build --target x86_64-pc-windows-msvc

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
    // set the process to be DPI unaware
    // if cfg!(windows) {
    //     unsafe {
    //         winapi::um::shellscalingapi::SetProcessDpiAwareness(
    //             winapi::um::shellscalingapi::PROCESS_DPI_UNAWARE,
    //         );
    //     }
    // }

    // create the cropper
    let mut cropper = Cropper::new().unwrap();
    println!("Hello from Rust!");
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

}