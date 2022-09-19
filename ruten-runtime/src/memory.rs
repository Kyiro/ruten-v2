use crate::Pattern;
use std::ptr;
use winapi::um::libloaderapi::*;
use winapi::um::winnt::*;

pub unsafe fn pattern_scan(pattern: Pattern) -> Option<*mut ()> {
    let handle = GetModuleHandleA(ptr::null_mut());
    let dos_header = handle as PIMAGE_DOS_HEADER;
    let nt_headers = (handle as *const u8).offset((*dos_header).e_lfanew as _)
        as PIMAGE_NT_HEADERS;
    let image_size = (*nt_headers).OptionalHeader.SizeOfImage;

    let scan_bytes = handle as *mut u8;
    let pattern = pattern.bytes();

    'a: for i in 0..(image_size as usize - pattern.len()) {
        'b: for (j, byte) in pattern.iter().enumerate() {
            let byte = match byte {
                None => continue 'b,
                Some(s) => *s,
            };

            if *scan_bytes.add(i + j) != byte {
                continue 'a;
            }
        }

        return Some(scan_bytes.add(i) as _);
    }

    None
}