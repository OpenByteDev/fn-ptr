use fn_ptr::{FnPtr, StaticFnPtr};
use static_assertions::{assert_impl_all, assert_not_impl_all};

#[test]
fn static_fn_ptr_is_implemented_for_static_signature() {
    type F = extern "C" fn(i32) -> i32;
    assert_impl_all!(F: FnPtr, StaticFnPtr);
}

#[test]
fn static_fn_ptr_is_implemented_for_unsafe_static_signature() {
    type F = unsafe extern "system" fn(u8, u16) -> ();
    assert_impl_all!(F: FnPtr, StaticFnPtr);
}

#[test]
fn non_static_argument_does_not_implement_static_fn_ptr() {
    type F = fn(&i32);
    assert_not_impl_all!(F: StaticFnPtr);
}
