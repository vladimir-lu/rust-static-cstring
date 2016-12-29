
// Thanks to https://stackoverflow.com/questions/33850189/how-to-publish-a-constant-string-in-the-rust-ffi

#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

#[repr(C)]
pub struct AString {
    pub name: StaticCString,
}

const MY_NAME: StaticCString = StaticCString(b"hello cruel rust world\0" as *const u8);

#[no_mangle]
pub static MY_STRING: AString = AString {
    name: MY_NAME
};

/* DOES NOT WORK
const my_name: &'static [u8] = b"hello\0";
const my_name_ptr: *const c_char = my_name.as_ptr() as *const c_char;

static s: AString = AString {
    name: my_name_ptr
};
*/
