/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct detail_PointerType {
    pub _address: u8,
}
pub type detail_PointerType_Type<T> = *mut T;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UniquePtr {
    pub _address: u8,
}
pub type UniquePtr_Pointer = detail_PointerType;
