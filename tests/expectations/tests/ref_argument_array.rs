/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


pub const NSID_LENGTH: ::std::os::raw::c_uint = 10;
#[repr(C)]
pub struct nsID__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nsID {
    pub vtable_: *const nsID__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_nsID() {
    assert_eq!(::std::mem::size_of::<nsID>() , 8usize);
    assert_eq!(::std::mem::align_of::<nsID>() , 8usize);
}
impl Clone for nsID {
    fn clone(&self) -> Self { *self }
}