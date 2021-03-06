/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_nsTArray<T> {
    pub mBuff: *mut T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_nsStyleAutoArray<T> {
    pub mFirstElement: T,
    pub mOtherElements: Struct_nsTArray<T>,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum nsStyleAutoArray_WithSingleInitialElement {
    WITH_SINGLE_INITIAL_ELEMENT = 0,
}
