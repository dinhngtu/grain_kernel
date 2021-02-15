use core::{mem::size_of, str::from_utf8};

pub unsafe fn from_bytes<'a, T>(bytes: &'a [u8]) -> Option<&'a T> {
    if bytes.len() == size_of::<T>() {
        Some(&*(bytes.as_ptr() as *const T))
    } else {
        None
    }
}

pub unsafe fn slice_from_bytes<'a, T>(bytes: &'a [u8]) -> Option<&'a [T]> {
    assert_ne!(size_of::<T>(), 0);
    if bytes.len() % size_of::<T>() == 0 {
        let nr = bytes.len() / size_of::<T>();
        Some(core::slice::from_raw_parts(bytes.as_ptr() as *const T, nr))
    } else {
        None
    }
}

pub fn str_from_cstr<'a>(bytes: &'a [u8], offset: usize) -> Option<&'a str> {
    assert!(offset < bytes.len());
    for end in offset..bytes.len() {
        if bytes[end] == 0 {
            return bytes.get(offset..end).and_then(|x| from_utf8(x).ok());
        }
    }
    None
}
