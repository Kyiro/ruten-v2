use std::fmt::Display;
use std::ptr;
use std::iter::once;
use winapi::um::winuser::*;
use winapi::um::processthreadsapi::*;
use winapi::shared::minwindef::*;

pub unsafe fn create_thread(func: unsafe extern "system" fn(LPVOID) -> DWORD) {
    CreateThread(ptr::null_mut(), 0, Some(func), ptr::null_mut(), 0, ptr::null_mut());
}

pub fn message_box<T: Display, M: Display>(title: T, message: M) {
    let lp_text: Vec<u16> = format!("{}", title).encode_utf16().chain(once(0)).collect();
    let lp_caption: Vec<u16> = format!("{}", message).encode_utf16().chain(once(0)).collect();

    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            lp_caption.as_ptr(),
            lp_text.as_ptr(),
            MB_OK
        );
    }
}

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|err| {
        message_box(
            "Panic Occured",
            err.to_string()
        );
    }))
}