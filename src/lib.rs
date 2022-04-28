use std::os::raw::c_char;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[repr(C)]
pub struct MHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct SHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    // shim wrappers
    pub fn new_mavsdk() -> MHandle;
    pub fn del_mavsdk(p: *mut MHandle);

    // helper wrappers
    pub fn connect(p: *mut MHandle, addr: *const c_char) -> SHandle;
}
