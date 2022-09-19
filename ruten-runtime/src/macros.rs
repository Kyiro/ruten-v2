#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        {
            use std::ffi::CString;
            CString::new("").unwrap().into_raw()
        }
    };
}

#[macro_export]
macro_rules! define_app {
    ($start:block) => {
        use winapi::um::winnt::*;

        #[no_mangle]
        #[allow(unused_variables)]
        extern "system" fn DllMain(module: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOL {
            match reason {
                DLL_PROCESS_ATTACH => {
                    #[allow(unused_unsafe)]
                    unsafe {$start}
                }
                DLL_PROCESS_DETACH => (),
                _ => (),
            }
            TRUE
        }
    };
}
