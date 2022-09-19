use winapi::shared::minwindef::*;

pub mod curl;
pub mod curldef;
pub mod macros;
pub mod memory;
pub mod util;

mod pattern;

pub use pattern::Pattern;

unsafe extern "system" fn main(_lib: LPVOID) -> DWORD {
    util::set_panic_hook();
    curl::init().unwrap();

    0
}

define_app!({
    util::create_thread(main);
});
