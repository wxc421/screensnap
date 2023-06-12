mod cropper;
mod focuser;
mod hotkey;
mod msgbox;
mod screengrab;

mod ffi;


#[cfg(test)]
mod tests {
    use crate::ffi::hello_from_rust;
    #[test]
    fn test_add() {
        hello_from_rust()
    }

}