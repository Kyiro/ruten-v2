#![allow(non_camel_case_types, non_upper_case_globals)]
use crate::{cstr, Pattern};
use crate::curldef::*;
use crate::memory::pattern_scan;
use detour::GenericDetour;
use std::error::Error as StdError;
use std::mem;

type fn_curl_easy_setopt = extern "C" fn(curl: *mut (), option: CURLoption, paramaters: *mut ()) -> CURLcode;

static mut curl_easy_setopt: Option<GenericDetour<fn_curl_easy_setopt>> = None;

extern "C" fn curl_easy_setopt_detour(curl: *mut (), option: CURLoption, parameters: *mut ()) -> CURLcode {
    unsafe {
        let func = curl_easy_setopt.as_ref().unwrap();

        match option {
            CURLoption::CURLOPT_NOPROXY => func.call(curl, option, cstr!("") as _),
            CURLoption::CURLOPT_SSL_VERIFYPEER => func.call(curl, option, 0 as _),
            CURLoption::CURLOPT_SSL_VERIFYHOST => func.call(curl, option, 0 as _),
            CURLoption::CURLOPT_PINNEDPUBLICKEY => CURLcode::CURLE_OK,
            _ => func.call(curl, option, parameters)
        }
    }
}

pub unsafe fn init() -> Result<(), Box<dyn StdError>> {
    let easy = pattern_scan(Pattern::from_str("89 54 24 10 4C 89 44 24 18 4C 89 4C 24 20 48 83 EC 28 48 85 C9"))
        .ok_or("curl_easy_setopt pattern wasn't found")?;
    let easy = mem::transmute::<*const (), fn_curl_easy_setopt>(easy as *const ());

    let easy_detour = GenericDetour::<fn_curl_easy_setopt>::new(easy, curl_easy_setopt_detour)?;

    easy_detour.enable()?;

    curl_easy_setopt = Some(easy_detour);

    Ok(())
}