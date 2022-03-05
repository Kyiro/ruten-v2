// use libc::*;
// use std::mem::size_of;
// use std::ptr::null_mut;

// #[repr(C)]
// pub struct TArray<T> {
//     data: *mut T,
//     count: i32,
//     max: i32
// }

// impl<T> TArray<T> {
//     pub fn new() -> Self {
//         Self {
//             data: null_mut(),
//             count: 0,
//             max: 0
//         }
//     }

//     pub fn len(&self) -> i32 {
//         self.count
//     }

//     pub fn has(&self, i: i32) -> bool {
//         i < self.len()
//     }

//     pub unsafe fn get(&self, i: i32) -> *mut T {
//         self.data.add(i as usize * size_of::<T>())
//     }

//     pub unsafe fn push(&mut self, data: T) {
//         self.data = realloc(self.data, size_of::<T>() * (self.count + 1));
//         self.count += 1;
//     }
// }
