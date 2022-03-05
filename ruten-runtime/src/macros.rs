#[macro_export]
macro_rules! define_app {
    ($module:ident, $reason:ident, $reserved:ident, $start:block) => {
        use minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
        use winapi::shared::minwindef;
        use winapi::um::winnt;

        #[no_mangle]
        #[allow(unused_variables)]
        extern "system" fn DllMain($module: HINSTANCE, $reason: DWORD, $reserved: LPVOID) -> BOOL {
            match $reason {
                winnt::DLL_PROCESS_ATTACH => {
                    std::thread::spawn(move || $start);
                }
                winnt::DLL_PROCESS_DETACH => (),
                _ => (),
            }
            minwindef::TRUE
        }
    };

    ($start:block) => {
        define_app!(module, reason, reserved, $start);
    };
}
